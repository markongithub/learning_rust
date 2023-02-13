use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
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
        let next_back = move_one(map_width, map_height, loopback_spot, backwards);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction3D {
    Up,
    Down,
    Front,
    Back,
    Left,
    Right,
}

/*
fn move_from_side(from_side: Direction3D, direction: Direction3D) -> (Direction3D, Direction3D) {
    match (from_side, direction) {
        (Side::Top, Direction::Up) => (Side::Back, Direction::Down),
        (Side::Top, Direction::Left) => (Side::Left, Direction::Down),
        (Side::Top, Direction::Right) => (Side::Right, Direction::Down),
        (Side::Top, Direction::Down) => (Side::Front, Direction::Down),
        (Side::Left, Direction::Up) => (Side::Top, Direction::Right),
        (Side::Left, Direction::Left) => (Side::Back, Direction::Left),
        (Side::Left, Direction::Right) => (Side::Front, Direction::Right),
        (Side::Left, Direction::Down) => (Side::Bottom, Direction::Right),
        (Side::Right, Direction::Up) => (Side::Top, Direction::Left),
        (Side::Right, Direction::Left) => (Side::Front, Direction::Left),
        (Side::Right, Direction::Right) => (Side::Back, Direction::Right),
        (Side::Right, Direction::Down) => (Side::Bottom, Direction::Left),
        (Side::Bottom, Direction::Up) => (Side::Front, Direction::Up),
        (Side::Bottom, Direction::Left) => (Side::Left, Direction::Up),
        (Side::Bottom, Direction::Right) => (Side::Right, Direction::Up),
        (Side::Bottom, Direction::Down) => (Side::Back, Direction::Up),
        (Side::Front, Direction::Up) => (Side::Top, Direction::Up),
        (Side::Front, Direction::Left) => (Side::Left, Direction::Up),
        (Side::Front, Direction::Right) => (Side::Right, Direction::Up),
        (Side::Front, Direction::Down) => (Side::Back, Direction::Up),
        (Side::Back, Direction::Up) => (Side::Front, Direction::Up),
        (Side::Back, Direction::Left) => (Side::Left, Direction::Up),
        (Side::Back, Direction::Right) => (Side::Right, Direction::Up),
        (Side::Back, Direction::Down) => (Side::Back, Direction::Up),


    }
}
*/

/*
fn orientation_to_direction(grid_direction: Direction, orientation: Direction3D) -> Direction3D {
    match (grid_direction, orientation) {
        Direction::Up ->
    }
}
*/

/*
fn turn_right(side: Direction3D, direction: Direction3D) -> Direction3D {
    match (side, direction) {
        (Direction3D::Front, d)  => Direction3D::Right,
        Direction3D:: => Direction3D::Right,

    }
}
*/

/*
on front, grid-up is up, then every grid direction = its 2d equivalent
can we do this with coordinates, I ask again?
up = y+, down = y-, left = x-, etc
No I don't think so
but this is going to require 6 X 4 X 4 = 96 match cases unless I figure out a better way
what if we say on front the four possible directions clockwise are up right down left
and then we apply a shift using those?
E is back and its grid up is bottom
grid up is bottom, grid right is still right I think? why?
because the grid-up and being back cancel each other out?

back's four directions are top left bottom right (NOTE the left and right)
to turn right, go one right in that list
if gu is bottom turn right twice first
since gu is bottom, shift by 2 -> bottom right top left
then map the four grid directions onto that! OMG!

*/

fn convert_grid_direction(
    side: Direction3D,
    grid_up: Direction3D,
    grid_direction: Direction,
) -> Direction3D {
    let four_possible_directions: Vec<Direction3D> = match side {
        Direction3D::Front => vec![
            Direction3D::Up,
            Direction3D::Right,
            Direction3D::Down,
            Direction3D::Left,
        ],
        Direction3D::Back => vec![
            Direction3D::Up,
            Direction3D::Left,
            Direction3D::Down,
            Direction3D::Right,
        ],
        Direction3D::Up => vec![
            Direction3D::Back,
            Direction3D::Right,
            Direction3D::Front,
            Direction3D::Left,
        ],
        Direction3D::Down => vec![
            Direction3D::Front,
            Direction3D::Right,
            Direction3D::Back,
            Direction3D::Left,
        ],
        Direction3D::Right => vec![
            Direction3D::Up,
            Direction3D::Back,
            Direction3D::Down,
            Direction3D::Front,
        ],
        Direction3D::Left => vec![
            Direction3D::Up,
            Direction3D::Front,
            Direction3D::Down,
            Direction3D::Back,
        ],
    };

    let mut rotation_count = 0;
    // I think I can do this faster.
    for i in 0..4 {
        if four_possible_directions[i] == grid_up {
            rotation_count = i;
            break;
        }
    }

    four_possible_directions[(rotation_count + (grid_direction as usize)) % 4]
}

fn get_grid_up(
    side: Direction3D,
    grid_direction: Direction,
    where_it_goes: Direction3D,
) -> Direction3D {
    let four_possible_directions: Vec<Direction3D> = match side {
        Direction3D::Front => vec![
            Direction3D::Up,
            Direction3D::Right,
            Direction3D::Down,
            Direction3D::Left,
        ],
        Direction3D::Back => vec![
            Direction3D::Up,
            Direction3D::Left,
            Direction3D::Down,
            Direction3D::Right,
        ],
        Direction3D::Up => vec![
            Direction3D::Back,
            Direction3D::Right,
            Direction3D::Front,
            Direction3D::Left,
        ],
        Direction3D::Down => vec![
            Direction3D::Front,
            Direction3D::Right,
            Direction3D::Back,
            Direction3D::Left,
        ],
        Direction3D::Right => vec![
            Direction3D::Up,
            Direction3D::Back,
            Direction3D::Down,
            Direction3D::Front,
        ],
        Direction3D::Left => vec![
            Direction3D::Up,
            Direction3D::Front,
            Direction3D::Down,
            Direction3D::Back,
        ],
    };

    let mut direction_3d_index = 0;
    // I think I can do this faster.
    for i in 0..4 {
        if four_possible_directions[i] == where_it_goes {
            direction_3d_index = i;
            break;
        }
    }
    let direction_2d_index = grid_direction as usize;
    let rotation = if direction_2d_index > direction_3d_index {
        (direction_3d_index + 4) - direction_2d_index
    } else {
        direction_3d_index - direction_2d_index
    };
    four_possible_directions[rotation]
}

fn solve_part_2(input: &str) {
    let (map, width, movements) = parse_input(input);
    // test input is 12 rows by 16 columns
    // puzzle input is 200 rows by 150 columns
    // can't make assumptions about the side length anymore.
    //  WRONG ->  let side_length = width / 4;
    // ok this won't work in a general case but it works for both inputs
    let num_rows = map.len() / width;
    let mut side_length = width;
    for y in 0..num_rows {
        let mut this_row_nonempty = 0;
        for x in 0..width {
            if map[(width * y) + x] != Square::Void {
                this_row_nonempty += 1;
            }
        }
        if this_row_nonempty < side_length {
            side_length = this_row_nonempty;
        }
    }
    let num_big_rows = num_rows / side_length;
    let num_big_columns = width / side_length;
    println!(
        "The rows are {} squares wide, and I think each side is a square with side length {}. So that means {} big rows and {} big columns?",
        width, side_length, num_big_rows, num_big_columns
    );
    let mut side_coords = vec![];
    let mut edges_to_explore: Vec<(usize, usize)> = vec![];
    for row_of_sides in 0..num_big_rows {
        let map_row = row_of_sides * side_length;
        for column_of_sides in 0..num_big_columns {
            let map_column = column_of_sides * side_length;
            let map_coord = (map_row * width) + map_column;
            //  println!(
            //     "Considering big column {} of big row {}, based at ({},{})",
            //    column_of_sides, row_of_sides, map_column, map_row
            //);
            if map[map_coord] != Square::Void {
                println!(
                    "There is a side whose upper left corner is {} ({},{})",
                    map_coord, map_column, map_row
                );
                side_coords.push(map_coord);
                if column_of_sides > 0 {
                    let side_to_left = map_coord - side_length;
                    if map[side_to_left] != Square::Void {
                        println!("  - there is also a side to its left at {}", side_to_left);
                        edges_to_explore.push((map_coord, side_to_left));
                    }
                }
                if column_of_sides < (num_big_columns - 1) {
                    let side_to_right = map_coord + side_length;
                    if map[side_to_right] != Square::Void {
                        println!("  - there is also a side to its right at {}", side_to_right);
                        edges_to_explore.push((map_coord, side_to_right));
                    }
                }
                if row_of_sides > 0 {
                    let side_above = map_coord - (side_length * width);
                    if map[side_above] != Square::Void {
                        println!("  - there is also a side above it at {}", side_above);
                        edges_to_explore.push((map_coord, side_above));
                    }
                }
                if row_of_sides < (num_big_rows - 1) {
                    let side_below = map_coord + (side_length * width);
                    if map[side_below] != Square::Void {
                        println!("  - there is also a side below it at {}", side_below);
                        edges_to_explore.push((map_coord, side_below));
                    }
                }
            }
        }
    }
    let mut side_directions: HashMap<usize, Direction3D> = Default::default();
    let mut side_orientations: HashMap<usize, Direction3D> = Default::default();
    let mut found_first_side: bool = false;
    loop {
        let mut sides_added = 0;
        for edge_ref in edges_to_explore.iter() {
            let (from, to): (usize, usize) = *edge_ref;
            let (from_x, from_y) = square_coords(from, width);
            if !found_first_side {
                found_first_side = true;
                side_directions.insert(from, Direction3D::Front);
                println!("({},{}) is the front side.", from_x, from_y);
                side_orientations.insert(from, Direction3D::Up);
            }
            if side_directions.contains_key(&to) {
                continue;
            }
            if !side_directions.contains_key(&from) {
                continue;
            }
            let from_side = side_directions.get(&from).unwrap();
            let from_orientation = side_orientations.get(&from).unwrap();
            let grid_direction = if to == from + (side_length * width) {
                Direction::Down
            } else if to == from + side_length {
                Direction::Right
            } else if to == from - side_length {
                Direction::Left
            } else if to == from - (side_length * width) {
                Direction::Up
            } else {
                panic!("What direction is {} from {}?", to, from)
            };
            let next_side = convert_grid_direction(*from_side, *from_orientation, grid_direction);
            let (to_x, to_y) = square_coords(to, width);
            println!(
                "This edge leads grid-{:?} from the {:?} side, where grid-Up means {:?}. So our next side ({},{}) is {:?}.",
                grid_direction, from_side, from_orientation, to_x, to_y, next_side
            );
            /*
            fn get_grid_up(
                side: Direction3D,
                grid_direction: Direction,
                where_it_goes: Direction3D,
            ) -> Direction3D {
                */
            let next_orientation =
                get_grid_up(next_side, opposite_direction(grid_direction), *from_side);
            println!(
                "Grid-up on the {:?} side will be {:?}",
                next_side, next_orientation
            );
            side_directions.insert(to, next_side);
            side_orientations.insert(to, next_orientation);
            sides_added += 1;
            // So we combine grid_direction and from_orientation to get the direction of the new side.
            // Up + Up -> Top
            // if my grid-up is Front I have to know which side I am on
            // Right + Right -> Down -> Bottom
        }
        if sides_added == 0 {
            break;
        }
    }
    /*

    let mut side_iter = side_coords.iter();
    for side_coord in side_iter {
        let (x, y) = square_coords(*side_coord, width);
        if !found_first_side {
            found_first_side = true;
            side_directions.insert(*side_coord, Direction3D::Front);
            println!("({},{}) is the front side.", x, y);
        }
        if x == last_x && y - last_y == side_length {
            println!(
                "({},{}) and ({},{}) will fold on the x axis.",
                last_x, last_y, x, y
            );
        } else if y == last_y && x - last_x == side_length {
            println!(
                "({},{}) and ({},{}) will fold on the y axis.",
                last_x, last_y, x, y
            );
        }
        last_side = *side_coord;
    }
        */
}
/*
fn fold_on_axis(x1: usize, y1: usize, x2: usize, y2: usize, on_x: bool) -> (usize, usize, bool) {

}
(0,4) and (4,4) will fold on the y axis.
(4,4) and (8,4) will fold on the y axis.
(8,4) and (8,8) will fold on the x axis.
(8,8) and (12,8) will fold on the y axis.
So as you go from 0,4 towards 4,4 and 4,7 your z remains constant
let's say you're at 0,4,0 to 4,4,0 or 4,7,0
but after the fold your x becomes a z
so when you go from 4,4 to 5,4 on 2d, you go from 4,4,0 to 4,4,1 on 3d
for any x in {4..7}  and y in {4..7} z = x - 4 and y=y
but then we fold again at 8,4
and your z stays constant at 4 and you
how do we represent this?
z = ax + by
no
B is on the 0,4 plane at 0,6
D is on the 4,4 plane at 5,4
(0,4) and (4,4) will fold on the y axis.
the 0,4 plane is the front at z=0
the 4,4 plane will be the right and move toward z=1
in the 4,4 plane x=4 and z=x-4 and y=y
D=(5,4)=(4,4,1)

(4,4) and (8,4) will fold on the y axis.
8,4 will be the backwards plane
in the 8,4 plane z=4 and y=y and x=4-(x-8)=12-x
A is on the 8,4 plane at 11,6
A=(11,6)=(1,6,4)

(8,4) and (8,8) will fold on the x axis.
8,8 is below 8,4 and will be the bottom side
so z decreases as y increases
z=4-(y-8)=12-y
y=8
x=12-x same as before
90% sure I learned how to do this with vectors in linear algebra

(8,8) and (12,8) will fold on the y axis.
12,8 is to the right as you look at the bottom
12,8 is the left side

*/

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
    //    let real_input = read_to_string("data/input22.txt").unwrap();
    //    println!("Part 1 solution: {:?}", solve_part_1(&real_input));
    solve_part_2(&test_input);
}
// let's say the bottom of 4 is horizontal
// it goes 0,0,0 to 3,0,0
// bottom 4 adjoins top 5
// left 4 adjoins right 3
// top 4 adjoins bottom 1
// right 4?
// 4 goes from 0,0,0 to 3,3,0
// when you hit the top of 4 and go up you turn 90 degrees
// 1 goes from 0,3,0 to 0,3,3
