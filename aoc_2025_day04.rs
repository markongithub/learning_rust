#![allow(unused)]

// use array2d::{Array2D, Error};
use std::cmp::max;
use std::cmp::min;
use std::fs::read_to_string;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

fn parse_row(row: &str) -> Vec<bool> {
    row.chars()
        .map(|c| match c {
            '.' => false,
            '@' => true,
            _ => panic!("the input wasn't . or @"),
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(parse_row).collect()
}

fn neighbor_counts(rows: &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let num_rows = rows.len();
    let num_columns = rows[0].len();
    let mut neighbor_coords = vec![vec![0; num_columns]; num_rows];
    for y in 0..num_rows {
        for x in 0..num_columns {
            if !rows[y][x] {
                continue;
            }
            let min_y = if y == 0 { 0 } else { y - 1 };
            let min_x = if x == 0 { 0 } else { x - 1 };
            for neighbor_y in min_y..(min(num_rows, y + 2)) {
                for neighbor_x in min_x..min(num_columns, x + 2) {
                    if (x, y) != (neighbor_x, neighbor_y) {
                        /*                    println!(
                            "Since {},{} is true I am going to increment {},{} from {}",
                            y, x, neighbor_y, neighbor_x, neighbor_coords[y][x]
                        ); */
                        neighbor_coords[neighbor_y][neighbor_x] += 1;
                    }
                }
            }
        }
    }
    neighbor_coords
}

fn solve_part_1(input: &str) -> usize {
    let grid = parse_input(input);
    let count_grid = neighbor_counts(&grid);
    let mut y = 0;
    let mut accessible = 0;
    for row in count_grid.iter() {
        let mut x = 0;
        for column in row.iter() {
            if *column < 4 && grid[y][x] {
                accessible += 1;
            }
            x += 1;
        }
        y += 1;
    }
    accessible
}

fn main() {
    println!("whatever: {:?}", parse_row("..@@.@@@@."));
    println!("whatever: {:?}", parse_input("..@@.@@@@."));
    println!(
        "whatever: {:?}",
        neighbor_counts(&parse_input(".@...@@.@@@@."))
    );
    let test_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    println!("should be 13: {}", solve_part_1(test_input));
    let real_input = read_to_string("data/input_2025_04.txt").unwrap();
    println!("part 1 answer: {}", solve_part_1(&real_input));
}
