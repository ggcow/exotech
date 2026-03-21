use crate::game::types::*;

const MIN_GRID_HEIGHT: Coord = 10;
const MAX_GRID_HEIGHT: Coord = 25;
const ASPECT_RATIO: f32 = 1.0;

// RNG minimal pseudo-aléatoire
struct SimpleRng {
    seed: u64,
}

impl SimpleRng {
    fn new() -> Self {
        Self {
            seed: 0x12345678abcdef,
        }
    }

    fn next_f64(&mut self) -> f64 {
        // xorshift simple
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        ((self.seed % 1_000_000) as f64) / 1_000_000.0
    }

    fn range(&mut self, min: Coord, max: Coord) -> Coord {
        min + ((max - min) as f64 * self.next_f64()).round() as Coord
    }

    fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            Some(&slice[(self.next_f64() * slice.len() as f64) as usize])
        }
    }
}

pub fn generate_grid(league_level: i32) -> Grid {
    let mut rng = SimpleRng::new();

    let skew = match league_level {
        1 => 2.0,
        2 => 1.0,
        3 => 0.8,
        _ => 0.3,
    };

    let rand = rng.next_f64();
    let height = MIN_GRID_HEIGHT
        + ((rand.powf(skew) * (MAX_GRID_HEIGHT - MIN_GRID_HEIGHT) as f64).round() as Coord);

    let mut width = (height as f32 * ASPECT_RATIO).round() as Coord;
    if width % 2 != 0 {
        width += 1;
    }

    let mut grid = Grid {
        width,
        height,
        cells: vec![Cell::Empty; (width * height) as usize],
    };

    let b = 5.0 + rng.next_f64() * 10.0;

    for x in 0..width {
        grid[Point::new(x, height - 1)] = Cell::Wall;
    }

    for y in (0..height - 1).rev() {
        let y_norm = (height - 1 - y) as f64 / (height - 1) as f64;
        let block_chance = 1.0 / (y_norm + 0.1) / b;

        for x in 0..width {
            if rng.next_f64() < block_chance {
                grid[Point::new(x, y)] = Cell::Wall;
            }
        }
    }

    mirror(&mut grid);
    fill_small_air_pockets(&mut grid);
    destroy_tight_spaces(&mut grid, &mut rng);
    sink_lowest_island(&mut grid, &mut rng);
    spawn_food(&mut grid, &mut rng);
    cleanup_lonely_walls(&mut grid);

    grid
}

fn opposite(p: Point, width: Coord) -> Point {
    Point::new(width - p.x - 1, p.y)
}

fn mirror(grid: &mut Grid) {
    let w = grid.width;
    let h = grid.height;

    for y in 0..h {
        for x in 0..w {
            let p = Point::new(x, y);
            let opp = opposite(p, w);
            grid[opp] = grid[p];
        }
    }
}

use std::collections::{HashSet, VecDeque};

fn neighbours(grid: &Grid, p: Point) -> Vec<Point> {
    DIRECTIONS
        .iter()
        .map(|d| p + d)
        .filter(|n| grid.in_bounds(*n))
        .collect()
}

fn flood_fill_empty(grid: &Grid, start: Point, visited: &mut HashSet<Point>) -> HashSet<Point> {
    let mut q = VecDeque::new();
    let mut comp = HashSet::new();
    q.push_back(start);
    visited.insert(start);

    while let Some(p) = q.pop_front() {
        comp.insert(p);
        for n in neighbours(grid, p) {
            if !visited.contains(&n) && grid[n] != Cell::Wall {
                visited.insert(n);
                q.push_back(n);
            }
        }
    }

    comp
}

fn fill_small_air_pockets(grid: &mut Grid) {
    let mut visited = HashSet::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            if grid[p] == Cell::Wall || visited.contains(&p) {
                continue;
            }
            let comp = flood_fill_empty(grid, p, &mut visited);
            if comp.len() < 10 {
                for c in comp {
                    let opp = opposite(c, grid.width);
                    grid[c] = Cell::Wall;
                    grid[opp] = Cell::Wall;
                }
            }
        }
    }
}

fn destroy_tight_spaces(grid: &mut Grid, rng: &mut SimpleRng) {
    let mut changed = true;
    while changed {
        changed = false;

        for y in 0..grid.height {
            for x in 0..grid.width {
                let p = Point::new(x, y);
                if grid[p] == Cell::Wall {
                    continue;
                }

                let neigh: Vec<_> = neighbours(grid, p);
                let walls: Vec<_> = neigh
                    .iter()
                    .copied()
                    .filter(|n| grid[*n] == Cell::Wall)
                    .collect();

                if walls.len() >= 3 {
                    let destroyable: Vec<_> = walls.into_iter().filter(|n| n.y <= p.y).collect();
                    if let Some(target) = rng.choose(&destroyable) {
                        let opp = opposite(*target, grid.width);
                        grid[*target] = Cell::Empty;
                        grid[opp] = Cell::Empty;
                        changed = true;
                    }
                }
            }
        }
    }
}

fn detect_lowest_island(grid: &Grid) -> Vec<Point> {
    let start = Point::new(0, grid.height - 1);
    if grid[start] != Cell::Wall {
        return vec![];
    }

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    let mut out = vec![];
    q.push_back(start);
    visited.insert(start);

    while let Some(p) = q.pop_front() {
        out.push(p);
        for n in neighbours(grid, p) {
            if !visited.contains(&n) && grid[n] == Cell::Wall {
                visited.insert(n);
                q.push_back(n);
            }
        }
    }
    out
}

fn sink_lowest_island(grid: &mut Grid, rng: &mut SimpleRng) {
    let island = detect_lowest_island(grid);
    if island.is_empty() {
        return;
    }

    let mut lower_by = 0;
    'outer: loop {
        for x in 0..grid.width {
            let p = Point::new(x, grid.height - 2 - lower_by);
            if !island.contains(&p) {
                break 'outer;
            }
        }
        lower_by += 1;
    }

    if lower_by >= 2 {
        lower_by = rng.range(2, lower_by);
    }

    for &c in &island {
        grid[c] = Cell::Empty;
        let opp = opposite(c, grid.width);
        grid[opp] = Cell::Empty;
    }

    for &c in &island {
        let lowered = Point::new(c.x, c.y + lower_by);
        if grid.in_bounds(lowered) {
            grid[lowered] = Cell::Wall;
            let opp = opposite(lowered, grid.width);
            grid[opp] = Cell::Wall;
        }
    }
}

fn spawn_food(grid: &mut Grid, rng: &mut SimpleRng) {
    for y in 0..grid.height {
        for x in 0..grid.width / 2 {
            let p = Point::new(x, y);
            if grid[p] == Cell::Empty && rng.next_f64() < 0.025 {
                let opp = opposite(p, grid.width);
                grid[p] = Cell::Food;
                grid[opp] = Cell::Food;
            }
        }
    }
}

fn cleanup_lonely_walls(grid: &mut Grid) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            if grid[p] != Cell::Wall {
                continue;
            }

            let count = neighbours(grid, p)
                .into_iter()
                .filter(|n| grid[*n] == Cell::Wall)
                .count();
            if count == 0 {
                let opp = opposite(p, grid.width);
                grid[p] = Cell::Food;
                grid[opp] = Cell::Food;
            }
        }
    }
}
