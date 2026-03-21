use std::time::Instant;

use crate::game::runner::GameIter;
use crate::game::types::{Cell, Point};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

mod game;

const MAX_TURNS: usize = 100;

#[macroquad::main("exotech")]
async fn main() {
    let mut games = Vec::with_capacity(MAX_TURNS);
    for game in GameIter::new(MAX_TURNS) {
        games.push(game);
        clear_background(WHITE);
        let text = format!("{}%", games.len() * 100 / MAX_TURNS);
        let font_size = screen_width().max(screen_height()) / 5.;
        let text_dimensions = measure_text(&text, None, font_size as u16, 1.0);
        let text_width = text_dimensions.width;
        let text_height = text_dimensions.height;

        draw_text(
            &text,
            screen_width() / 2. - text_width / 2.,
            screen_height() / 2. + text_height / 2.,
            font_size,
            BLACK,
        );
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
    let mut speed: f32 = 1.0;

    // Dimensions à partir de la première partie
    let board_width = games[0].grid.width as f32 + 2.;
    let board_height = games[0].grid.height as f32 + 2.;

    let mut current_index = 0;

    let size = (screen_width() / board_width).min(screen_height() / board_height);
    let mut last_update = get_time();

    loop {
        clear_background(WHITE);
        draw_rectangle(0., 0., size * board_width, size * board_height, DARKGRAY);

        let current_game = &games[current_index];

        // Dessin de la grille
        let grid = &current_game.grid;
        for y in 0..grid.height {
            for x in 0..grid.width {
                let cell = grid[Point::new(x, y)];
                let color = match cell {
                    Cell::Wall => BLACK,
                    Cell::Snake(0) | Cell::Snake(1) => Color::from_hex(0x51ff00),
                    Cell::Snake(2) | Cell::Snake(3) => Color::from_hex(0x1fe7c6),
                    Cell::Snake(4) | Cell::Snake(5) => Color::from_hex(0x22dfe6),
                    Cell::Snake(6) | Cell::Snake(7) => Color::from_hex(0xff0000),
                    Cell::Snake(8) | Cell::Snake(9) => Color::from_hex(0xff7300),
                    Cell::Snake(10) | Cell::Snake(11) => Color::from_hex(0xffbb00),
                    Cell::Food => YELLOW,
                    _ => GRAY,
                };

                match cell {
                    Cell::Snake(id) if id & 1 == 0 => draw_circle(
                        (x as f32 + 1.5) * size,
                        (y as f32 + 1.5) * size,
                        size * 0.42,
                        color,
                    ),
                    Cell::Snake(_) => draw_rectangle(
                        (x as f32 + 1.1) * size,
                        (y as f32 + 1.1) * size,
                        size * 0.8,
                        size * 0.8,
                        color,
                    ),
                    _ => draw_rectangle(
                        (x as f32 + 1.05) * size,
                        (y as f32 + 1.05) * size,
                        size * 0.9,
                        size * 0.9,
                        color,
                    ),
                }
            }
        }

        // Avancer l'index selon la speed
        let now = get_time();
        if now - last_update >= 1.0 / speed as f64 {
            current_index = (current_index + 1) % games.len();
            last_update = now;
        }

        // Détecte les touches
        let touch = touches();
        let touch = touch.last();
        // Détecte clic/touch efficace
        let mouse_or_touch = is_mouse_button_released(MouseButton::Left) || touch.is_some();

        // Si touch ou clic, on récupère la dernière position effective
        let in_corner = if let Some(touch) = touch {
            touch.position.x.max(touch.position.y) < screen_width().min(screen_height())
        } else {
            let (mx, my) = mouse_position();
            mx.max(my) < screen_width().min(screen_height())
        };

        // +1 ou -1 par les touches clavier
        let key_up = is_key_pressed(KeyCode::Up);
        let key_down = is_key_pressed(KeyCode::Down);

        // Applique changement
        if (mouse_or_touch && in_corner) || key_up {
            speed = (speed + 1.).min(10.0);
            println!("speed increased to {}", speed);
        }

        if (mouse_or_touch && !in_corner) || key_down {
            speed = (speed - 1.).max(1.0);
            println!("speed decreased to {}", speed);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
