use piston_window::Glyphs;
use piston_window::texture::TextureSettings;
use rand::Rng;

use piston_window::graphics::{Transformed, clear, text};
use piston_window::{
    Button, EventLoop, Key, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

mod draw;

pub const CELL_SIZE: f64 = 20.0;

pub struct Snake {
    pub body: Vec<[f64; 2]>,
    pub size: f64,
    pub direction: Direction,
}

pub struct Food {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_opposite(current: &Direction, next: &Direction) -> bool {
    matches!(
        (current, next),
        (Direction::Down, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
    )
}

pub fn hit_wall(snake: &Snake) -> bool {
    let head = snake.body[0];

    head[0] < 0.0 || head[1] < 0.0 || head[0] + snake.size > 600.0 || head[1] + snake.size > 400.0
}

pub fn hit_self(snake: &Snake) -> bool {
    let head = snake.body[0];

    snake
        .body
        .iter()
        .skip(1)
        .any(|segment| segment[0] == head[0] && segment[1] == head[1])
}

pub fn move_snake(snake: &mut Snake, grow: bool) {
    let mut new_head = snake.body[0];

    match snake.direction {
        Direction::Up => new_head[1] -= CELL_SIZE,
        Direction::Down => new_head[1] += CELL_SIZE,
        Direction::Left => new_head[0] -= CELL_SIZE,
        Direction::Right => new_head[0] += CELL_SIZE,
    }

    snake.body.insert(0, new_head);

    if !grow {
        snake.body.pop();
    };
}

fn random_food(snake: &Snake) -> Food {
    let mut rng = rand::rng();

    loop {
        let column = rng.random_range(0..30);
        let row = rng.random_range(0..20);

        let food = Food {
            x: column as f64 * CELL_SIZE,
            y: row as f64 * CELL_SIZE,
            size: CELL_SIZE,
        };

        let on_snake = snake
            .body
            .iter()
            .any(|segment| segment[0] == food.x && segment[1] == food.y);

        if !on_snake {
            return food;
        }
    }
}

fn eats_food(snake: &Snake, food: &Food) -> bool {
    let head = snake.body[0];
    head[0] == food.x && head[1] == food.y
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("My Snake Game", [600, 400])
        .exit_on_esc(true)
        .build()
        .expect("could not create window!");

    window.set_ups(60);

    let mut snake = Snake {
        body: vec![[100.0, 100.0]],
        size: CELL_SIZE,
        direction: Direction::Right,
    };

    let mut food = random_food(&snake);

    let mut movement_timer = 0.0;
    let movement_interval = 0.2;
    let mut game_over = false;
    let mut score = 0;

    let mut glyphs = window
        .load_font("assets/DejaVuSans.ttf", TextureSettings::new())
        .expect("Could not load font");

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let next_direction = match key {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Right => Some(Direction::Right),
                Key::Left => Some(Direction::Left),
                _ => None,
            };

            if let Some(next_direction) = next_direction {
                if !is_opposite(&snake.direction, &next_direction) {
                    snake.direction = next_direction;
                }
            }
        }

        if let Some(args) = event.update_args() {
            if !game_over {
                movement_timer += args.dt;

                if movement_timer >= movement_interval {
                    // Move the snake first.
                    move_snake(&mut snake, false);

                    movement_timer -= movement_interval;

                    if hit_wall(&snake) || hit_self(&snake) {
                        game_over = true;
                    } else if eats_food(&snake, &food) {
                        // Grow the snake by restoring the tail segment.
                        let tail = *snake.body.last().unwrap();
                        snake.body.push(tail);

                        score += 1;
                        food = random_food(&snake);
                    }
                }
            }
        }

        window.draw_2d(&event, |context, graphics, device| {
            clear([0.05, 0.05, 0.08, 1.0], graphics);

            draw::draw_square(context, graphics, &snake);
            draw::draw_food(context, graphics, &food);

            text(
                [1.0, 1.0, 1.0, 1.0],
                24,
                &format!("Score: {}", score),
                &mut glyphs,
                context.transform.trans(10.0, 30.0),
                graphics,
            )
            .expect("Could not draw score");

            if game_over {
                draw::draw_game_over(context, graphics);
            }

            // Send any new character images to the GPU.
            // glyphs.factory.encoder.flush(device);
        });
    }
}
