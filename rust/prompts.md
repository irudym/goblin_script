## Adding javascript support

The idea of this educational game is to help kids learn JavaScript (and other programming languages in the future) and algorithms.
The application is a top-down, pixel-art-style RPG game where a user can control the main goblin character by programming his behavior in JavaScript.

To implement such functionality help me to add javascript interpretator to my application. Research, which crate I can use for my Rust based application. It should support defining rust functions call. For example, when an user call move_east(), or move_right() function in JS, it should call Rust character function fn move_east().


## Steps fix
To fix error, that character in WalkState state can miss target point due to frame drops and can process going indefinitely, you suggested to change a character position in WalkState update function and set character current_speed to 0. But this is not a right solution, as now character in the WalkState is not taking into consideration step tiles and cannot go in diagonal to simulate waking downstairs of upstairs, as this behaviour implemented by using a velocity vector which is calculated by get_effective_velocity function. Could you help to fix it and implement new WalkState update function using character current_speed value to support stairs movement, but still somehow checking that character has reached the target point even with frame drops.
