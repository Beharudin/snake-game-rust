use piston_window::graphics::{Context, Graphics, rectangle};

use crate::CELL_SIZE;
use crate::Snake;

pub fn draw_square<G: Graphics>(c: Context, g: &mut G, snake: &Snake) {
    let color = [0.0, 0.1, 0.0, 1.0];

    for segment in &snake.body {
        let square = [segment[0], segment[1], snake.size, snake.size];

        rectangle(color, square, c.transform, g);
    }
}

pub fn draw_game_over<G: Graphics>(c: Context, g: &mut G) {
    let color = [1.0, 0.0, 0.0, 1.0];

    let bar = [200.0, 180.0, 200.0, 40.0];

    rectangle(color, bar, c.transform, g);
}

pub fn draw_food<G: Graphics>(c: Context, g: &mut G, food: &crate::Food) {
    let square = [food.x, food.y, food.size, food.size];
    let color = [1.0, 0.0, 0.0, 1.0];

    rectangle(color, square, c.transform, g);
}
