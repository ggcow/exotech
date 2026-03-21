use core::panic;
use std::collections::{HashMap, HashSet};

use crate::game::types::*;

pub fn ai(game: &GameState, mine: bool) -> Vec<Direction> {
    let snakes = game
        .snakes
        .iter()
        .filter(|snake| snake.mine == mine)
        .collect::<Vec<&Snake>>();

    let grid = &game.grid;

    let mut directions = Vec::with_capacity(snakes.len());

    for snake in snakes {
        if let Some(dir) = find_food(grid, snake) {
            directions.push(dir);
        } else {
            directions.push(find_safe_move(grid, snake));
        }
    }

    directions
}

fn find_safe_move(grid: &Grid, snake: &Snake) -> Direction {
    let neighbours = neighbours(grid, snake.head());

    if neighbours.is_empty() {
        Direction::Up
    } else {
        neighbours[0]
    }
}

fn neighbours(grid: &Grid, p: Point) -> Vec<Direction> {
    DIRECTIONS
        .iter()
        .copied()
        .filter(|d| grid.in_bounds(p + *d))
        .filter(|d| matches!(grid[p + *d], Cell::Empty | Cell::Food))
        .collect()
}

fn find_food(grid: &Grid, snake: &Snake) -> Option<Direction> {
    let starting_length = snake.len()
        - snake
            .body
            .iter()
            .position(|&p| p.supported(grid, snake))
            .unwrap();

    let mut queue = Vec::new();

    queue.push((snake.head(), starting_length));

    let mut visited = HashSet::new();
    visited.insert(snake.head());
    let mut parents = HashMap::new();

    while queue.len() > 0 {
        let (p, mut remaining_length) = queue.remove(0);

        if remaining_length == 0 {
            continue;
        }

        if p.supported(grid, snake) {
            remaining_length = snake.len();
        }

        if grid[p] == Cell::Food {
            let mut current = p;
            let mut path = Vec::new();
            while let Some(parent) = parents.get(&current) {
                path.push(current);
                current = *parent;
            }
            let dir = match *path.last().unwrap() - snake.head() {
                Point { x: _, y: -1 } => Direction::Up,
                Point { x: 1, y: _ } => Direction::Right,
                Point { x: _, y: 1 } => Direction::Down,
                Point { x: -1, y: _ } => Direction::Left,
                Point { x, y } => panic!("Unexpected direction: ({}, {})", x, y),
            };
            return Some(dir);
        }

        for dir in neighbours(grid, p) {
            let n = p + dir;
            if !visited.contains(&n) {
                visited.insert(n);
                parents.insert(n, p);
                queue.push((n, remaining_length - 1));
            }
        }
    }

    None
}
