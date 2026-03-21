use crate::game::types::*;

pub fn ai(game: &GameState, mine: bool) -> Vec<Direction> {
    game.snakes
        .iter()
        .filter_map(|snake| {
            (snake.mine == mine).then(|| {
                if mine {
                    Direction::Left
                } else {
                    Direction::Right
                }
            })
        })
        .collect()
}
