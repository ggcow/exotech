use crate::game::types::*;

impl GameState {
    pub fn apply(&mut self, directions: Vec<Direction>) {
        for (snake, dir) in self.snakes.iter_mut().zip(directions) {
            self.grid.clear_snake(snake);
            snake.r#move(dir, self.grid[snake.head()] == Cell::Food);
        }

        self.resolve_collisions();

        for snake in &self.snakes {
            self.grid.place_snake(snake);
        }
    }

    fn resolve_collisions(&mut self) {
        // collisions with walls
        for snake in &mut self.snakes {
            if self.grid[snake.head()] == Cell::Wall {
                snake.lose_head();
            }
        }
        self.snakes.retain(|s| s.len() > 2);

        // collisions with other snakes
        for i in 0..self.snakes.len() {
            for j in (i + 1)..self.snakes.len() {
                if self.snakes[i].body[1..].contains(&self.snakes[j].head()) {
                    if self.snakes[i].body[1..].contains(&self.snakes[j].head()) {
                        self.snakes[i].lose_head();
                    }
                    self.snakes[j].lose_head();
                }
                if self.snakes[j].body[1..].contains(&self.snakes[i].head()) {
                    self.snakes[i].lose_head();
                }
            }
        }

        self.snakes.retain(|s| s.len() > 2);
        self.snakes
            .iter()
            .for_each(|snake| self.grid.place_snake(snake));

        // gravity
        fn is_supported(snake: &Snake, grid: &Grid) -> bool {
            snake
                .body
                .iter()
                .any(|&p| grid[p + Point::new(0, 1)] != Cell::Empty)
        }

        let mut moved = true;

        while moved {
            moved = false;
            for snake in &mut self.snakes {
                if !is_supported(snake, &self.grid) {
                    moved = true;

                    self.grid.clear_snake(snake);
                    snake.fall();
                    self.grid.place_snake(snake);
                }
            }
        }
    }
}
