use std::ops;

type Coord = i16;

#[derive(Clone)]
pub struct Grid {
    pub width: Coord,
    pub height: Coord,
    pub cells: Vec<Cell>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self[Point::new(x, y)];
                let symbol = match cell {
                    Cell::Empty => '.',
                    Cell::Wall => '#',
                    Cell::Snake(s) => s,
                    Cell::Food => 'F',
                }
                .to_string()
                    + " ";
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(rows: Vec<Vec<Cell>>) -> Self {
        Self {
            width: rows[0].len() as Coord,
            height: rows.len() as Coord,
            cells: rows.into_iter().flatten().collect(),
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.x < self.width as Coord
            && point.y >= 0
            && point.y < self.height as Coord
    }

    pub fn place_food(&mut self, point: Point) {
        if self.in_bounds(point) {
            self[point] = Cell::Food;
        }
    }

    pub fn place_snake(&mut self, snake: &Snake) {
        self[snake.head()] = Cell::Snake(if snake.mine { '☺' } else { '☻' });
        for &point in &snake.body[1..] {
            self[point] = Cell::Snake(if snake.mine { '○' } else { '•' });
        }
    }

    pub fn clear_snake(&mut self, snake: &Snake) {
        for &p in &snake.body {
            self[p] = Cell::Empty;
        }
    }
}

impl ops::IndexMut<Point> for Grid {
    fn index_mut(&mut self, point: Point) -> &mut Cell {
        if !self.in_bounds(point) {
            panic!("Point out of bounds: ({}, {})", point.x, point.y);
        }
        let idx = (point.y as usize) * (self.width as usize) + (point.x as usize);
        &mut self.cells[idx]
    }
}

impl ops::Index<Point> for Grid {
    type Output = Cell;

    fn index(&self, point: Point) -> &Cell {
        if !self.in_bounds(point) {
            return &Cell::Wall;
        }
        let idx = (point.y as usize) * (self.width as usize) + (point.x as usize);
        &self.cells[idx]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Snake(char),
    Food,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}

impl Point {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<Coord> for Point {
    type Output = Self;

    fn mul(self, rhs: Coord) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(0, -1),
            Direction::Down => Self::new(0, 1),
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
        }
    }
}

#[derive(Clone)]
pub struct Snake {
    pub body: Vec<Point>,
    pub id: u8,
    pub mine: bool,
}

impl Snake {
    pub fn head(&self) -> Point {
        self.body[0]
    }

    pub fn r#move(&mut self, dir: Direction, grow: bool) {
        let new_head = self.head() + Point::from(dir);
        self.body.insert(0, new_head);
        if !grow {
            self.body.pop();
        }
    }

    pub fn fall(&mut self) {
        for p in &mut self.body {
            p.y += 1;
        }
    }

    pub fn lose_head(&mut self) {
        self.body.remove(0);
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }
}

#[derive(Clone)]
pub struct GameState {
    pub grid: Grid,
    pub snakes: Vec<Snake>,
    pub food: Vec<Point>,
}

impl GameState {
    pub fn new(mut grid: Grid, snakes: Vec<Snake>, food: Vec<Point>) -> Self {
        for snake in &snakes {
            grid.place_snake(snake);
        }
        for &point in &food {
            grid.place_food(point);
        }

        Self { grid, snakes, food }
    }
}
