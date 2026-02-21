# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GoblinScript is a game engine/framework for teaching kids programming, built in Rust with Godot engine integration. The Rust side handles all game logic; Godot handles rendering and input.

## Build Commands

```bash
# Build entire workspace
cargo build

# Run console test app (260-cycle character patrol simulation)
cargo run -p console_app

# Build Godot native library (cdylib)
cargo build -p godot_app

# Run benchmark (10k characters × 5k ticks)
cargo run -p bench

# Check all crates without building
cargo check --workspace

# Run tests
cargo test --workspace
```

## Workspace Structure

Five crates under `crates/`:

- **platform** — Platform abstraction layer. Defines `Animator` and `Logger` traits, shared types (`Vector2D`, `Vector2Di`, `Direction`). No external dependencies. All other crates depend on this.
- **game_core** — Pure game logic library. Character system, FSM, Behaviour Tree, AI worker, LogicMap. Uses `#![forbid(unsafe_code)]`.
- **console_app** — Terminal test harness. Implements `ConsoleAnimator` and `ConsoleLogger` for running game logic without Godot.
- **godot_app** — Godot engine integration (cdylib). Implements Godot-specific animators, loggers, debug/grid overlays. Depends on gdext master branch.
- **benchmark** — Stress test for multithreaded BT evaluation.

## Architecture

### Dependency Flow

```
platform (traits + types)
    ↑
game_core (logic)
    ↑
scripting_vm
    ↑
console_app / godot_app / benchmark
```

### Character System (`game_core/src/character/`)

`CharacterLogic` is the central entity. Its `process(delta, logic_map)` method:
1. Submits a BT job to the AI worker thread
2. Retrieves BT results and applies commands (movement, direction changes, state requests)
3. Handles FSM state transitions via `pending_request` (Arc<Mutex>, last-win strategy)
4. Updates position, validates movement against LogicMap, applies step height offsets

`CharacterSnapshot` is a read-only clone sent to the worker thread for thread-safe BT evaluation.

### FSM (`game_core/src/fsm/`)

States: Idle, Run, Turn, Walk. Each implements the `FSM` trait (`enter`, `exit`, `update`, `can_transition_to`, `can_exit`). Transitions are requested via `StateRequest` enum, not called directly.

### Behaviour Tree (`game_core/src/bt/`)

`BTNode` trait with composite nodes (Sequence, Selector) and leaf nodes (MoveToTarget, WalkToTarget, NextWaypoint, Wait, FindTarget, IsAtTarget). Nodes communicate via `Blackboard` (Arc<RwLock<HashMap>>) using string keys namespaced by node ID (e.g., `"{id}.idx"`). BT evaluation returns `BTResult` containing a list of `BTCommand`s.

### Multithreaded AI (`game_core/src/ai/worker.rs`)

Worker thread receives `BTJob`s via crossbeam channel, evaluates BTs in parallel using rayon thread pool (batches of up to 32), stores results in a global `RESULT_MAP`. Main thread retrieves results with `take_result(character_id)`.

### Map System (`game_core/src/map/logic_map.rs`)

`LogicMap` is a 2D grid of `LogicCell` (walkable, height, is_step). Serialized to/from RON format (`logic_map.ron` at project root). Validates character movement including step/height transitions.

### Scripting System (`scripting_vm/src/vm/vm.rs`)

ScriptVM loads and executes JavaScript string. 



## Key Patterns

- **Trait-based platform abstraction**: `Animator` and `Logger` traits decouple game logic from engine specifics
- **Global logger**: Initialized once via `init_logger()` using `OnceLock`, accessed through `log_info!`/`log_debug!` macros in platform
- **Snapshot pattern**: Characters are snapshotted before BT evaluation to avoid shared mutable state across threads
- **Grid-based coordinates**: Characters operate on a cell grid; pixel position = `(grid_coord * cell_size) + cell_size/2`
- **Direction-aware animations**: Animation names follow `"{action}_{direction}"` convention (e.g., `"run_east"`, `"turn_north_south"`)
- **RON serialization**: Level data uses Rust Object Notation via serde/ron
