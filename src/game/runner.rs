use std::vec;

use crate::game::{ai::ai, ai2::ai2, random::generate_grid, types::*};

#[derive(Clone)]
pub struct GameIter {
    game: GameState,
    max_steps: usize,
    step: usize,
    finished: bool,
}

impl GameIter {
    pub fn new(max_steps: usize) -> Self {
        let grid = generate_grid(1);
        let mut food = Vec::new();
        for x in 0..grid.width {
            for y in 0..grid.height {
                let p = Point::new(x, y);
                if grid[p] == Cell::Food {
                    food.push(p);
                }
            }
        }

        let snakes = vec![
            Snake {
                body: vec![Point::new(5, 8), Point::new(5, 9), Point::new(5, 10)],
                id: 0,
                mine: true,
            },
            Snake {
                body: vec![Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)],
                id: 1,
                mine: true,
            },
            Snake {
                body: vec![Point::new(20, 7), Point::new(20, 8), Point::new(20, 9)],
                id: 3,
                mine: false,
            },
            Snake {
                body: vec![Point::new(22, 4), Point::new(22, 5), Point::new(22, 6)],
                id: 4,
                mine: false,
            },
        ];

        let game = GameState::new(grid, snakes, food);

        Self {
            game,
            max_steps,
            step: 0,
            finished: false,
        }
    }
}

impl Iterator for GameIter {
    type Item = GameState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.step >= self.max_steps {
            return None;
        }

        // Premier step : juste renvoyer l'état initial
        if self.step == 0 {
            self.step += 1;
            return Some(self.game.clone());
        }

        // Appliquer mouvements à partir du second step
        let my_directions = ai2(&self.game, true);
        let his_directions = ai(&self.game, false);

        let my_snakes = self.game.snakes.iter().filter(|s| s.mine).count();
        let his_snakes = self.game.snakes.iter().filter(|s| !s.mine).count();
        assert_eq!(my_directions.len(), my_snakes);
        assert_eq!(his_directions.len(), his_snakes);

        let directions = my_directions
            .into_iter()
            .chain(his_directions.into_iter())
            .collect::<Vec<_>>();

        self.game.apply(&directions, None);

        if self.game.snakes.iter().all(|s| s.mine) || self.game.snakes.iter().all(|s| !s.mine) {
            self.finished = true;
        }

        self.step += 1;
        Some(self.game.clone())
    }
}
