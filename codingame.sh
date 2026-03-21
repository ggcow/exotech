#!/bin/bash

TMP_FILE=$(mktemp)

cat << 'EOF' > "$TMP_FILE"
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_id = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let width = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32);
    let mut rows = Vec::new();
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string();
        rows.push(row);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let snakebots_per_player = parse_input!(input_line, i32);
    let mut my_ids = Vec::new();
    for i in 0..snakebots_per_player as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_snakebot_id = parse_input!(input_line, i16);
        my_ids.push(my_snakebot_id);
    }
    for i in 0..snakebots_per_player as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let opp_snakebot_id = parse_input!(input_line, i32);
    }

    let cells: Vec<Vec<Cell>> = rows
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    'F' => Cell::Food,
                    _ => Cell::Snake(0),
                })
                .collect()
        })
        .collect();

    let empty_grid = Grid::new(cells);
    
    loop {
        let mut grid = empty_grid.clone();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let power_source_count = parse_input!(input_line, i32);
        for i in 0..power_source_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i16);
            let y = parse_input!(inputs[1], i16);
            grid[Point::new(x, y)] = Cell::Food;
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let snakebot_count = parse_input!(input_line, i32);
        let mut my_snakebots = Vec::new();
        for i in 0..snakebot_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let snakebot_id = parse_input!(inputs[0], i16);
            let body = inputs[1].trim().to_string();
            let body = body
            .split(":")
            .map(|p| {
                let inputs = p.split(",").collect::<Vec<_>>();
                Point {
                    x: inputs[0].parse::<i16>().unwrap(),
                    y: inputs[1].parse::<i16>().unwrap(),
                }
            })
            .collect::<Vec<_>>();
            my_snakebots.push(Snake {
                id: snakebot_id,
                body,
                mine: my_ids.contains(&snakebot_id),
            });
        }
        
        let game = GameState::new(grid, my_snakebots.clone(), Vec::new());
        
        eprintln!("{}", game.grid);

        let directions = ai2(&game, true);
        let s = directions.iter()
            .zip(my_snakebots.iter())
            .map(|(x, k)| format!("{} {}", k.id, x.to_string()))
            .collect::<Vec<_>>()
            .join(";");

        println!("{}", s);
    }
}
EOF

for file in src/game/types.rs src/game/engine.rs src/game/ai2.rs; do
    sed '/use crate::game::types::\*;/d' "$file" >> "$TMP_FILE"
done

# Windows clipboard
cat "$TMP_FILE" | clip

rm "$TMP_FILE"