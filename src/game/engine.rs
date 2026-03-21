use crate::game::types::*;

impl GameState {
    pub fn apply(&mut self, directions: &[Direction], mine: Option<bool>) {
        for (snake, dir) in self
            .snakes
            .iter_mut()
            .filter(|snake| {
                if let Some(mine) = mine {
                    snake.mine == mine
                } else {
                    true
                }
            })
            .zip(directions.iter())
        {
            let eat = self.grid[snake.head() + dir] == Cell::Food;
            if !eat {
                self.grid[snake.tail()] = Cell::Empty;
            }
            snake.r#move(&dir, eat);
        }

        self.resolve_collisions();

        for snake in &self.snakes {
            self.grid.place_snake(snake);
        }
    }

    fn resolve_collisions(&mut self) {
        let collision_grid_mask = self.grid.map(|cell| match cell {
            Cell::Empty | Cell::Food => false,
            _ => true,
        });

        for snake in &mut self.snakes {
            if !self.grid.in_bounds(snake.head()) || collision_grid_mask[snake.head()] {
                snake.lose_head();
            } else {
                self.grid.place_head(snake);
            }
        }

        // remove dead snakes (snake.len() == 2) and for each clear the grid
        self.snakes
            .iter()
            .filter(|snake| snake.len() == 2)
            .for_each(|snake| self.grid.clear_snake(snake));
        self.snakes.retain(|snake| snake.len() > 2);

        self.snakes
            .iter()
            .for_each(|snake| self.grid.place_snake(snake));

        // gravity
        let mut moved = true;
        while moved {
            moved = false;
            for snake in &mut self.snakes {
                if !snake.supported(&self.grid) {
                    moved = true;

                    self.grid.clear_snake(snake);
                    snake.fall();
                    self.grid.place_snake(snake);
                }
            }
        }
    }
}
