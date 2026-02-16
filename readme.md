### Goblin Script

This project is an educational game designed to teach kids programming through play. Players learn JavaScript, and other programming languages, by solving challenges and understanding algorithms in an interactive world. Instead of memorizing syntax, they develop real computational thinking skills while guiding a character through adventures.

#### Application description

This application is a top-down, pixel-art style RPG where players control a goblin character by programming his behavior in JavaScript.
By writing code, players move the character, interact with the environment, solve puzzles, and complete quests. Each level introduces new programming concepts and algorithmic thinking in a fun, visual way, making coding intuitive and engaging for beginners.

The game turns programming from an abstract subject into a hands-on experience. Players don’t just learn code, they see it come alive.

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
