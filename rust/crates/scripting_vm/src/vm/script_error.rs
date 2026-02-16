#[derive(Debug)]
pub struct ScriptError {
    pub message: String,
    pub line: i32,
    pub col: i32,
}

impl ScriptError {
    /// Parse line and column from boa's error string.
    /// Backtrace entries look like: `at update (unknown at :8:27)`
    /// We extract `:line:col` from the first backtrace entry.
    pub fn from_js_error(err: boa_engine::JsError) -> Self {
        let message = err.to_string();
        let (line, col) = parse_position(&message);
        Self { message, line, col }
    }
}

/// Extracts line and column from boa's error string.
/// Handles two formats:
/// - Syntax errors:  `"at line 4, col 9"`
/// - Runtime errors: `"(unknown at :8:27)"`
fn parse_position(s: &str) -> (i32, i32) {
    // Try syntax error format: "at line <N>, col <N>"
    if let Some(i) = s.find("at line ") {
        let after_line = &s[i + 8..];
        if let Some((line, rest)) = parse_u32(after_line) {
            if let Some(j) = rest.find("col ") {
                if let Some((col, _)) = parse_u32(&rest[j + 4..]) {
                    return (line as i32, col as i32);
                }
            }
        }
    }

    // Try runtime/backtrace format: ":line:col)"
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b':' {
            if let Some((line, rest)) = parse_u32(&s[i + 1..]) {
                if rest.starts_with(':') {
                    if let Some((col, rest2)) = parse_u32(&rest[1..]) {
                        if rest2.starts_with(')') {
                            return (line as i32, col as i32);
                        }
                    }
                }
            }
        }
        i += 1;
    }

    (-1, -1)
}

/// Parse leading digits, return (value, remaining_str).
fn parse_u32(s: &str) -> Option<(u32, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    s[..end].parse::<u32>().ok().map(|v| (v, &s[end..]))
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ScriptError {}
