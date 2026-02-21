/// List of known command function names that should be tracked
const COMMAND_FUNCTIONS: &[&str] = &["step_up", "step_down", "step_right", "step_left"];

/// Instruments JavaScript source code by inserting `__line(N);` calls
/// before each command function call, where N is the 1-based line number.
///
/// The user writes clean JS — this preprocessing is invisible to them.
///
/// Example:
/// ```text
/// for (let i = 0; i < 3; i++) {   // line 1 — no command, unchanged
///     step_up();                    // line 2 → "__line(2); step_up();"
/// }
/// step_right();                    // line 4 → "__line(4); step_right();"
/// ```
pub fn instrument_code(code: &str) -> String {
    let mut result = String::with_capacity(code.len() + code.len() / 4);

    for (index, line) in code.lines().enumerate() {
        let line_number = index + 1; // 1-based

        if contains_command_call(line) {
            // Find the position of the command call and insert __line(N); before it
            if let Some(insert_pos) = find_command_insert_position(line) {
                result.push_str(&line[..insert_pos]);
                result.push_str(&format!("__line({});", line_number));
                result.push_str(&line[insert_pos..]);
            } else {
                result.push_str(line);
            }
        } else {
            result.push_str(line);
        }

        result.push('\n');
    }

    result
}

/// Checks if a line contains any known command function call
fn contains_command_call(line: &str) -> bool {
    COMMAND_FUNCTIONS.iter().any(|func| line.contains(func))
}

/// Finds the byte position where `__line(N);` should be inserted.
/// Returns the position of the first command function call found,
/// preserving leading whitespace.
fn find_command_insert_position(line: &str) -> Option<usize> {
    let mut earliest: Option<usize> = None;

    for func in COMMAND_FUNCTIONS {
        if let Some(pos) = line.find(func) {
            match earliest {
                Some(current) if pos < current => earliest = Some(pos),
                None => earliest = Some(pos),
                _ => {}
            }
        }
    }

    earliest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let code = "step_up();";
        let result = instrument_code(code);
        assert_eq!(result.trim(), "__line(1);step_up();");
    }

    #[test]
    fn test_indented_command() {
        let code = "    step_right();";
        let result = instrument_code(code);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "    __line(1);step_right();");
    }

    #[test]
    fn test_for_loop_with_command() {
        let code = "for (let i = 0; i < 3; i++) {\n    step_up();\n}";
        let result = instrument_code(code);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "for (let i = 0; i < 3; i++) {");
        assert_eq!(lines[1], "    __line(2);step_up();");
        assert_eq!(lines[2], "}");
    }

    #[test]
    fn test_no_command_line_unchanged() {
        let code = "let x = 5;";
        let result = instrument_code(code);
        assert_eq!(result.trim(), "let x = 5;");
    }

    #[test]
    fn test_multiple_lines() {
        let code = "step_up();\nlet x = 1;\nstep_right();";
        let result = instrument_code(code);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "__line(1);step_up();");
        assert_eq!(lines[1], "let x = 1;");
        assert_eq!(lines[2], "__line(3);step_right();");
    }

    #[test]
    fn test_function_definition_not_instrumented() {
        let code = "function update() {\n    step_up();\n}";
        let result = instrument_code(code);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "function update() {");
        assert_eq!(lines[1], "    __line(2);step_up();");
        assert_eq!(lines[2], "}");
    }
}
