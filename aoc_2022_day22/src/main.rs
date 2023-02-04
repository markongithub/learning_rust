use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Open,
    Wall,
    Void,
}

fn char_to_square(c: char) -> Square {
    match c {
        ' ' => Square::Void,
        '.' => Square::Open,
        '#' => Square::Wall,
        w => panic!("I did not expect {}", w),
    }
}

fn turn(dir: Direction, c: char) -> Direction {
    match (dir, c) {
        (Direction::Up, 'L') => Direction::Left,
        (Direction::Up, 'R') => Direction::Right,
        (Direction::Down, 'L') => Direction::Right,
        (Direction::Down, 'R') => Direction::Left,
        (Direction::Left, 'L') => Direction::Down,
        (Direction::Left, 'R') => Direction::Up,
        (Direction::Right, 'L') => Direction::Up,
        (Direction::Right, 'R') => Direction::Down,
        (d, q) => panic!("I did not expect {:?} and {}", d, q),
    }
}

fn parse_map_line(map_line: &str) -> Vec<Square> {
    map_line.chars().map(char_to_square).collect()
}

fn parse_directions(directions: &str) -> Vec<(Direction, usize)> {
    let mut i = 0;
    let mut number_began_at = None;
    let mut output = vec![];
    let mut current_direction = Direction::Right;
    for c in directions.chars() {
        if c.is_ascii_digit() {
            if number_began_at.is_none() {
                number_began_at = Some(i);
            }
        } else {
            let number_slice = &directions[number_began_at.unwrap()..i];
            let value = number_slice.parse::<usize>().unwrap();
            output.push((current_direction, value));
            number_began_at = None;
            current_direction = turn(current_direction, c);
        }
        i += 1;
    }
    let number_slice = &directions[number_began_at.unwrap()..i];
    let value = number_slice.parse::<usize>().unwrap();
    output.push((current_direction, value));
    output
}

fn opposite_direction(d: Direction) -> Direction {
    turn(turn(d, 'R'), 'R')
}

fn square_coords(square: usize, map_width: usize) -> (usize, usize) {
    (square % map_width, square / map_width)
}

fn move_one(map_width: usize, map_height: usize, start: usize, direction: Direction) -> usize {
    /*
    if map height is 7
    the indices in the last row will all be 6w + c

    */
    let (old_x, old_y) = square_coords(start, map_width);
    let (new_x, new_y) = match direction {
        Direction::Up => (
            old_x,
            if old_y == 0 {
                map_height - 1
            } else {
                old_y - 1
            },
        ),
        Direction::Down => (old_x, (old_y + 1) % map_height),
        Direction::Left => (if old_x == 0 { map_width - 1 } else { old_x - 1 }, old_y),
        Direction::Right => ((old_x + 1) % map_width, old_y),
    };
    (new_y * map_width) + new_x
}

fn move_distance(
    map: &Vec<Square>,
    map_width: usize,
    start: usize,
    (direction, distance): (Direction, usize),
) -> usize {
    let map_height = map.len() / map_width;
    // first find our loopback spot
    let mut loopback_spot = start;
    let backwards = opposite_direction(direction);
    loop {
        let  next_back = move_one(map_width, map_height, loopback_spot, backwards);
        if next_back == start || map[next_back] == Square::Void {
            break;
        }
        loopback_spot = next_back;
    }
    println!(
        "If I hit a void I will loop back to {:?}",
        square_coords(loopback_spot, map_width)
    );

    let mut my_position = start;
    for _i in 1..=distance {
        let next_position = move_one(map_width, map_height, my_position, direction);
        match map[next_position] {
            Square::Open => {
                my_position = next_position;
            }
            Square::Void => {
                println!(
                    "There is a void at square {:?}.",
                    square_coords(next_position, map_width)
                );
                if map[loopback_spot] == Square::Wall {
                    println!(
                        "... and a wall at {:?} so I am done.",
                        square_coords(loopback_spot, map_width)
                    );
                    break;
                } else {
                    my_position = loopback_spot;
                }
            }
            Square::Wall => {
                println!(
                    "There is a wall at square {:?}.",
                    square_coords(next_position, map_width)
                );
                break;
            }
        }
        println!(
            "I have moved to square {:?}",
            square_coords(next_position, map_width)
        );
    }
    my_position
}

fn parse_input(input: &str) -> (Vec<Square>, usize, Vec<(Direction, usize)>) {
    let mut blank_line_index = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        blank_line_index += 1;
    }

    let max_map_line_len = input
        .lines()
        .take(blank_line_index)
        .map(|s| s.len())
        .into_iter()
        .max()
        .unwrap();
    let mut squares = vec![];
    for map_line in input.lines().take(blank_line_index) {
        let padding_len = max_map_line_len - map_line.len();
        squares.append(&mut parse_map_line(&map_line));
        for i in 1..=padding_len {
            squares.push(Square::Void);
        }
    }
    let directions_line = input.lines().skip(blank_line_index + 1).next().unwrap();
    (squares, max_map_line_len, parse_directions(directions_line))
}
fn start_position(map: &Vec<Square>) -> usize {
    for i in 0..map.len() {
        if map[i] == Square::Open {
            return i;
        }
    }
    panic!("I didn't find an open square.")
}
fn solve_part_1(input: &str) -> usize {
    let (map, width, movements) = parse_input(input);
    let mut position = start_position(&map);
    let mut last_direction = Direction::Up;
    for movement in movements.iter() {
        let distance;
        (last_direction, distance) = *movement;
        println!(
            "I am about to move {} squares {:?}",
            distance, last_direction
        );
        position = move_distance(&map, width, position, *movement);
        println!("After that movement I am at square {}", position);
    }
    let direction_value = match last_direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    let final_row = (position / width) + 1;
    let final_column = (position % width) + 1;
    (1000 * final_row) + (4 * final_column) + direction_value
}
fn main() {
    let test_input = "        ...#
    .#..
    #...
    ....
...#.......#
........#...
..#....#....
..........#.
    ...#....
    .....#..
    .#......
    ......#.

10R5L5R10L4R5L5";
    println!("Part 1 test: {:?}", solve_part_1(&test_input));
    let real_input = read_to_string("data/input22.txt").unwrap();
    println!("Part 1 solution: {:?}", solve_part_1(&real_input));
}
