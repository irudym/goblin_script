### Goblin Script

A simple game to teach kids how to program.

#### Directories
* **goblin-godot** - Godot project with the game's assets
* **rust** - rust binding, game logic.


### Rust implementation
Crates: 
* platform - generic traits of Animator and Logger definition
* game_core - main game logic implementation: Finite State Machine, Behaviour Tree, CharacterLogic
* console_app - application to test game logic in a terminal
* godot_app - rust-godot binding
* benchmark - stress test to benchmark multithreaded behaviour tree implementation


### Usage
To run execution in console
```
cargo run -p console_app
```

To build Godot extension
```
cargo build -p godot_app
```

To run stress test
```
cargo run -p bench
```

### In game control

**Shift + D** - Show a debug overlay. 
**Shift + G** - Show a grid overlay.
