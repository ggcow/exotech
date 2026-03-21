use crate::game::ai::ai;
use crate::game::random::generate_grid;
use crate::game::types::*;

pub fn run() -> Vec<GameState> {
    let mut games = vec![GameState::new(
        generate_grid(1),
        vec![
            Snake {
                body: vec![Point::new(5, 5), Point::new(5, 6), Point::new(5, 7)],
                id: 0,
                mine: true,
            },
            Snake {
                body: vec![Point::new(2, 2), Point::new(2, 3), Point::new(2, 4)],
                id: 1,
                mine: false,
            },
        ],
        vec![Point::new(1, 1)],
    )];

    for _ in 0..100 {
        let mut game = games.last().unwrap().clone();
        let my_directions = ai(&game, true);
        let his_directions = ai(&game, false);
        // concat directions and apply to game
        let directions = my_directions
            .into_iter()
            .chain(his_directions.into_iter())
            .collect();
        game.apply(directions);
        let stop = game.snakes.iter().all(|s| s.mine) || game.snakes.iter().all(|s| !s.mine);
        games.push(game);
        if stop {
            break;
        }
    }
    println!("Game over!");
    games
}
