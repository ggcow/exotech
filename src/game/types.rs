use std::ops;

pub type Coord = i16;

#[derive(Clone)]
pub struct Grid<T: Clone = Cell> {
    pub width: Coord,
    pub height: Coord,
    pub cells: Vec<T>,
}

impl std::fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self[Point::new(x, y)];
                let symbol = match cell {
                    Cell::Empty => '.',
                    Cell::Wall => '#',
                    Cell::Snake(id) => char::from_digit(id as u32 >> 1, 10).unwrap(),
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

impl std::fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = if self[Point::new(x, y)] { "# " } else { ". " };
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid<Cell> {
    pub fn new(rows: Vec<Vec<Cell>>) -> Self {
        Self {
            width: rows[0].len() as Coord,
            height: rows.len() as Coord,
            cells: rows.into_iter().flatten().collect(),
        }
    }

    pub fn place_food(&mut self, point: Point) {
        if self.in_bounds(point) {
            self[point] = Cell::Food;
        }
    }

    pub fn place_snake(&mut self, snake: &Snake) {
        self.place_head(snake);
        for &point in &snake.body[1..] {
            self[point] = Cell::Snake(snake.id << 1);
        }
    }

    pub fn place_head(&mut self, snake: &Snake) {
        self[snake.head()] = Cell::Snake(snake.id << 1 | 1);
    }

    pub fn clear_snake(&mut self, snake: &Snake) {
        for &p in &snake.body {
            self[p] = Cell::Empty;
        }
    }

    pub fn map<F, T: Clone>(&self, f: F) -> Grid<T>
    where
        F: Fn(&Cell) -> T,
    {
        Grid {
            width: self.width,
            height: self.height,
            cells: self.cells.iter().map(f).collect(),
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.x < self.width as Coord
            && point.y >= 0
            && point.y < self.height as Coord
    }
}

impl<T: Clone> ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut T {
        let idx = (point.y as usize) * (self.width as usize) + (point.x as usize);
        &mut self.cells[idx]
    }
}

impl<T: Clone> ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &T {
        let idx = (point.y as usize) * (self.width as usize) + (point.x as usize);
        return &self.cells[idx];
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    Wall,
    Snake(Coord),
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

    pub fn supported(&self, grid: &Grid, snake: &Snake) -> bool {
        grid[*self + Direction::Down] != Cell::Empty
            && !matches!(grid[*self + Direction::Down], Cell::Snake(id) if id >> 1 == snake.id)
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

impl ops::Add<&Direction> for Point {
    type Output = Self;

    fn add(self, rhs: &Direction) -> Self {
        self + Point::from(rhs)
    }
}

impl ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        self + &rhs
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Left,
    Direction::Up,
    Direction::Down,
];

impl Direction {
    pub fn to_string(&self) -> String {
        match self {
            Direction::Up => "UP".to_string(),
            Direction::Down => "DOWN".to_string(),
            Direction::Left => "LEFT".to_string(),
            Direction::Right => "RIGHT".to_string(),
        }
    }
}

impl From<&Direction> for Point {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::Up => Self::new(0, -1),
            Direction::Down => Self::new(0, 1),
            Direction::Left => Self::new(-1, 0),
            Direction::Right => Self::new(1, 0),
        }
    }
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        Self::from(&dir)
    }
}

#[derive(Clone, Debug)]
pub struct Snake {
    pub body: Vec<Point>,
    pub id: Coord,
    pub mine: bool,
}

impl Snake {
    pub fn head(&self) -> Point {
        self.body[0]
    }

    pub fn tail(&self) -> Point {
        self.body[self.body.len() - 1]
    }

    pub fn r#move(&mut self, dir: &Direction, grow: bool) {
        let new_head = self.head() + dir;
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

    pub fn supported(&self, grid: &Grid) -> bool {
        self.body.iter().any(|&p| p.supported(grid, self))
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
