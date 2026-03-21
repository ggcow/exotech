use macroquad::prelude::*;

mod game;

#[macroquad::main("exotech")]
async fn main() {
    let games = game::runner::run();
    for game in &games {
        println!("{}", game.grid);
    }
    let start_time = get_time();
    // get canvas size and calculate cell size
    let (width, height) = (screen_width(), screen_height());
    let (grid_width, grid_height) = (games[0].grid.width as f32, games[0].grid.height as f32);
    let size = f32::min(width / grid_width, height / grid_height);
    loop {
        // poll for escape key to exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        let elapsed = get_time() - start_time;
        if elapsed > games.len() as f64 {
            break;
        }
        let game = &games[elapsed as usize % games.len()];
        clear_background(BLACK);

        let grid = &game.grid;

        for y in 0..grid.height {
            for x in 0..grid.width {
                let cell = grid[game::types::Point::new(x, y)];
                let color = match cell {
                    game::types::Cell::Wall => BLACK,
                    game::types::Cell::Snake('○') => DARKGREEN,
                    game::types::Cell::Snake('•') => DARKPURPLE,
                    game::types::Cell::Snake('☺') => GREEN,
                    game::types::Cell::Snake('☻') => PURPLE,
                    game::types::Cell::Food => YELLOW,
                    _ => GRAY,
                };
                draw_rectangle(
                    (x as f32 + 0.1) * size,
                    (y as f32 + 0.1) * size,
                    size * 0.8,
                    size * 0.8,
                    color,
                );
            }
        }

        next_frame().await
    }
}
