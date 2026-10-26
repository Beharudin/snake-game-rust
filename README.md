# Snake

A classic Snake game written in Rust with [piston_window](https://crates.io/crates/piston_window). I built it as a project for learning Rust.

## How to play

- Use the **arrow keys** to steer the snake.
- Eat the red food to grow and score a point.
- The game ends if you hit a wall or your own body.
- Press **Esc** to quit.

## Running it

You need [Rust](https://www.rust-lang.org/tools/install) installed. Then, from the project folder:

```
cargo run
```

## Project layout

- `src/main.rs` has the game state, movement, collisions and the main loop.
- `src/draw.rs` draws the snake, the food and the game-over bar.
- `assets/DejaVuSans.ttf` is the font used for the score.

## What I learned

- Structs and enums for game state (`Snake`, `Food`, `Direction`)
- Pattern matching with `match` and `matches!`
- Borrowing: `&` for reading, `&mut` for changing
- Working with `Vec` to grow and move the snake's body
- A fixed-step game loop using update and render events
- Drawing shapes and text with `piston_window`
