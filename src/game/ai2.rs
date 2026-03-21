use std::collections::{HashSet, VecDeque};

use crate::game::types::*;

fn neighbours(grid: &Grid, p: Point) -> Vec<Direction> {
    DIRECTIONS
        .iter()
        .copied()
        .filter(|dir| grid.in_bounds(p + dir))
        .filter(|dir| matches!(grid[p + dir], Cell::Empty | Cell::Food))
        .collect()
}

fn snake_moves(grid: &Grid, snake: &Snake) -> Vec<Direction> {
    let neighbours = neighbours(grid, snake.head());

    if neighbours.is_empty() {
        vec![Direction::Up]
    } else {
        neighbours
    }
}

pub fn ai2(game: &GameState, mine: bool) -> Vec<Direction> {
    let depth = 5;
    // Trier les snakes par ID pour fixer l’ordre et éviter doublons
    let my_snakes: Vec<&Snake> = game.snakes.iter().filter(|s| s.mine == mine).collect();

    find_best_moves(game, mine, depth, &my_snakes)
}

fn evaluate(game: &GameState, mine: bool) -> i32 {
    let total_length = game
        .snakes
        .iter()
        .filter(|s| s.mine == mine)
        .map(|s| s.len())
        .sum::<usize>() as i32;
    let total_distance = game
        .snakes
        .iter()
        .filter(|s| s.mine == mine)
        .map(|s| distance_to_closest_food(&game.grid, s))
        .sum::<i32>();

    100 + total_length - total_distance
}

fn distance_to_closest_food(grid: &Grid, snake: &Snake) -> i32 {
    // bfs from snake head
    let mut queue = VecDeque::new();
    queue.push_back((snake.head(), 0));
    let mut visited = HashSet::new();
    visited.insert(snake.head());

    while let Some((p, distance)) = queue.pop_front() {
        if grid[p] == Cell::Food {
            return distance;
        }

        for dir in neighbours(grid, p) {
            let n = p + dir;
            if !visited.contains(&n) {
                visited.insert(n);
                queue.push_back((n, distance + 1));
            }
        }
    }

    0
}

fn find_best_moves(
    game: &GameState,
    mine: bool,
    depth: i32,
    my_snakes: &[&Snake],
) -> Vec<Direction> {
    fn recursive(game: &GameState, mine: bool, depth: i32, history: &Vec<Vec<Direction>>) -> i32 {
        if depth == 0 {
            return evaluate(game, mine);
        }

        let my_snakes: Vec<&Snake> = game.snakes.iter().filter(|s| s.mine == mine).collect();

        let moves_per_snake: Vec<Vec<Direction>> = my_snakes
            .iter()
            .map(|s| snake_moves(&game.grid, s))
            .collect();

        let mut best_score = 0;

        for combo in moves_per_snake.cartesian_product_iter() {
            let mut new_game = game.clone();
            new_game.apply(&combo, Some(mine));

            let mut new_history = history.clone();
            new_history.push(combo.clone());

            let score = recursive(&new_game, mine, depth - 1, &new_history);
            if score > best_score {
                best_score = score;
            }
        }

        best_score
    }

    let moves_per_snake: Vec<Vec<Direction>> = my_snakes
        .iter()
        .map(|s| snake_moves(&game.grid, s))
        .collect();

    let mut best_combo = None;
    let mut best_score = i32::MIN;

    for combo in moves_per_snake.cartesian_product_iter() {
        let mut new_game = game.clone();
        new_game.apply(&combo, Some(mine));
        let score = recursive(&new_game, mine, depth - 1, &vec![combo.clone()]);
        if score > best_score {
            best_score = score;
            best_combo = Some(combo);
        }
    }

    best_combo.unwrap()
}

// ------------------ Trait et itérateur ------------------

pub trait CartesianProductIterTrait<T: Clone> {
    fn cartesian_product_iter(&self) -> CartesianProductIter<'_, T>;
}

pub struct CartesianProductIter<'a, T: Clone> {
    pools: &'a [Vec<T>],
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T: Clone> CartesianProductIterTrait<T> for Vec<Vec<T>> {
    fn cartesian_product_iter(&self) -> CartesianProductIter<'_, T> {
        let done = self.iter().any(|v| v.is_empty());
        let indices = vec![0; self.len()];
        CartesianProductIter {
            pools: &self[..],
            indices,
            done,
        }
    }
}

impl<'a, T: Clone> Iterator for CartesianProductIter<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self
            .indices
            .iter()
            .enumerate()
            .map(|(i, &idx)| self.pools[i][idx].clone())
            .collect::<Vec<_>>();

        for i in (0..self.indices.len()).rev() {
            self.indices[i] += 1;
            if self.indices[i] < self.pools[i].len() {
                break;
            } else {
                self.indices[i] = 0;
                if i == 0 {
                    self.done = true;
                }
            }
        }

        Some(result)
    }
}
