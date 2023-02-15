use std::collections::HashMap;
// use std::fs::read_to_string;

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

fn move_one_part_2(
    map_width: usize,
    map_height: usize,
    start: usize,
    direction: Direction,
    side_map: &HashMap<usize, (Direction3D, Direction3D)>,
    side_length: usize,
) -> (usize, Direction) {
    let (old_x, old_y) = square_coords(start, map_width);
    match direction {
        Direction::Up => {
            if old_y == 0 {
                go_around_corner(side_map, side_length, map_width, start, direction)
            } else {
                (two_coords_to_one(map_width, old_x, old_y - 1), direction)
            }
        }
        Direction::Down => {
            if old_y == map_height - 1 {
                go_around_corner(side_map, side_length, map_width, start, direction)
            } else {
                (two_coords_to_one(map_width, old_x, old_y + 1), direction)
            }
        }
        Direction::Left => {
            if old_x == 0 {
                go_around_corner(side_map, side_length, map_width, start, direction)
            } else {
                (two_coords_to_one(map_width, old_x - 1, old_y), direction)
            }
        }
        Direction::Right => {
            if old_x == map_width - 1 {
                go_around_corner(side_map, side_length, map_width, start, direction)
            } else {
                (two_coords_to_one(map_width, old_x + 1, old_y), direction)
            }
        }
    }
}

fn two_coords_to_one(map_width: usize, x: usize, y: usize) -> usize {
    (y * map_width) + x
}

fn move_distance(
    map: &Vec<Square>,
    map_width: usize,
    start: usize,
    (direction, distance): (Direction, usize),
) -> usize {
    let map_height = map.len() / map_width;
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

fn side_to_orientation(
    side_map: &HashMap<usize, (Direction3D, Direction3D)>,
    side: Direction3D,
) -> Direction3D {
    // this is so fucking terrible
    for (_, (this_side, this_orientation)) in side_map.iter() {
        if *this_side == side {
            return *this_orientation;
        }
    }
    panic!("I didn't find the side I wanted.")
}

fn go_around_corner(
    side_map: &HashMap<usize, (Direction3D, Direction3D)>,
    side_length: usize,
    map_width: usize,
    start: usize,
    direction: Direction,
) -> (usize, Direction) {
    // what are the possible coords in side_map if side_len is X
    let (x, y) = square_coords(start, map_width);
    let upper_left_x = side_length * (x / side_length);
    let upper_left_y = side_length * (y / side_length);
    let upper_left = (upper_left_y * map_width) + upper_left_x;
    println!(
        "My coordinates are ({},{}) so I think my upper left is {}",
        x, y, upper_left
    );
    let (side, orientation) = *side_map.get(&upper_left).unwrap();
    let next_side = convert_grid_direction(side, orientation, direction);
    println!(
        "I am going grid-{:?} from the {:?} side to the {:?} side.",
        direction, side, next_side
    );
    // okay I still need to know my direction on the new side
    // say I was going grid-up from Right3 to Front3
    // I guess on Front3 I am coming from Right3
    let next_side_orientation = side_to_orientation(side_map, next_side);
    let source_direction = direction_3d_to_grid(next_side, side, next_side_orientation);
    println!(
        "To get back to {:?} from {:?} means going grid-{:?}",
        side, next_side, source_direction
    );
    let final_direction = opposite_direction(source_direction);
    let lateral_position =
        get_lateral_exit_position(upper_left, side_length, map_width, start, direction);
    let mut next_upper_left = 0; // why do I have to do this
    for (coord, (side_val, _)) in side_map.iter() {
        if *side_val == next_side {
            next_upper_left = *coord;
            break;
        }
    }
    let next_position = get_entry_position(
        next_upper_left,
        side_length,
        map_width,
        lateral_position,
        final_direction,
    );
    let (final_x, final_y) = square_coords(next_position, map_width);
    println!(
        "So now I am going grid-{:?} from ({},{})",
        final_direction, final_x, final_y
    );
    (next_position, final_direction)
}

fn get_lateral_exit_position(
    upper_left: usize,
    side_length: usize,
    map_width: usize,
    start: usize,
    direction: Direction,
) -> usize {
    let (upper_left_x, upper_left_y) = square_coords(upper_left, map_width);
    let (x, y) = square_coords(start, map_width);
    match direction {
        Direction::Up => x - upper_left_x,
        Direction::Right => y - upper_left_y,
        Direction::Down => (upper_left_x + side_length) - (x + 1),
        Direction::Left => (upper_left_y + side_length) - (y + 1),
    }
}

fn get_entry_position(
    upper_left: usize,
    side_length: usize,
    map_width: usize,
    lateral_position: usize,
    direction: Direction,
) -> usize {
    let (upper_left_x, upper_left_y) = square_coords(upper_left, map_width);
    let max_x = upper_left_x + side_length - 1;
    let max_y = upper_left_y + side_length - 1;
    let (x, y) = match direction {
        Direction::Up => (upper_left_x + lateral_position, max_y),
        Direction::Right => (upper_left_x, upper_left_y + lateral_position),
        Direction::Down => (max_x - lateral_position, upper_left_y),
        Direction::Left => (max_x, max_y - lateral_position),
    };
    x + (map_width * y)
}

fn move_distance_part_2(
    map: &Vec<Square>,
    map_width: usize,
    side_map: &HashMap<usize, (Direction3D, Direction3D)>,
    side_length: usize,
    start: usize,
    (start_direction, distance): (Direction, usize),
) -> usize {
    let map_height = map.len() / map_width;
    let mut my_position = start;
    let mut direction = start_direction;
    for _i in 1..=distance {
        let (next_position, next_direction) = move_one_part_2(
            map_width,
            map_height,
            my_position,
            direction,
            side_map,
            side_length,
        );
        match map[next_position] {
            Square::Open => {
                my_position = next_position;
                direction = next_direction;
            }
            Square::Void => {
                println!(
                    "There is a void at square {:?}.",
                    square_coords(next_position, map_width)
                );
                // we need to find our next square
                let (around_corner, turned_direction) =
                    go_around_corner(side_map, side_length, map_width, my_position, direction);

                if map[around_corner] == Square::Wall {
                    println!(
                        "... and a wall at {:?} so I am done.",
                        square_coords(around_corner, map_width)
                    );
                    break;
                } else {
                    my_position = around_corner;
                    direction = turned_direction;
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
            square_coords(my_position, map_width)
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
    println!(
        "I think the widest line I found was {} characters.",
        max_map_line_len
    );
    for line in input.lines() {
        println!("This line has {} characters: {}", line.len(), line);
    }
    let mut squares = vec![];
    let mut line_id = 0;
    for map_line in input.lines().take(blank_line_index) {
        let padding_len = max_map_line_len - map_line.len();
        squares.append(&mut parse_map_line(&map_line));
        println!(
            "I am appending {} void squares to line {}",
            padding_len, line_id
        );
        for _i in 1..=padding_len {
            squares.push(Square::Void);
        }
        line_id += 1;
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
    println!("The map is of width {}", width);
    for square_id in 0..map.len() {
        println!("At square {} there is a {:?}", square_id, map[square_id]);
    }
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

fn direction_3d_to_grid(
    side: Direction3D,
    destination_side: Direction3D,
    orientation: Direction3D,
) -> Direction {
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

    let mut rotation = 0;
    // So say I am on Right and my Grid-Up is Down, which is index 2
    // in my four possible directions.
    for i in 0..4 {
        if four_possible_directions[i] == orientation {
            rotation = i;
            break;
        }
    }
    // so rotation = 2
    // so say I want to find the Back side. That is index 1 in my four.
    let mut direction_3d_index = 0;
    for i in 0..4 {
        if four_possible_directions[i] == destination_side {
            direction_3d_index = i;
            break;
        }
    }
    // so direction_3d_index = 1
    // I think I just want 1 + 2 mod 4
    match (rotation + direction_3d_index) % 4 {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        3 => Direction::Left,
        q => panic!("I modded a value by 4 and got {}", q),
    }
}

fn solve_part_2(input: &str) -> usize {
    let (map, width, movements) = parse_input(input);
    // test input is 12 rows by 16 columns
    // puzzle input is 200 rows by 150 columns
    // can't make assumptions about the side length anymore.
    //  WRONG ->  let side_length = width / 4;
    // ok this won't work in a general case but it works for both inputs
    let (side_map, side_length) = fold_cube(&map, width);

    let mut position = start_position(&map);
    let mut last_direction = Direction::Up;
    for movement in movements.iter() {
        let distance;
        (last_direction, distance) = *movement;
        println!(
            "I am about to move {} squares {:?}",
            distance, last_direction
        );
        position = move_distance_part_2(&map, width, &side_map, side_length, position, *movement);
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

fn fold_cube(
    map: &Vec<Square>,
    width: usize,
) -> (HashMap<usize, (Direction3D, Direction3D)>, usize) {
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
    let mut side_map: HashMap<usize, (Direction3D, Direction3D)> = Default::default();
    let mut found_first_side: bool = false;
    loop {
        let mut sides_added = 0;
        for edge_ref in edges_to_explore.iter() {
            let (from, to): (usize, usize) = *edge_ref;
            let (from_x, from_y) = square_coords(from, width);
            if !found_first_side {
                found_first_side = true;
                side_map.insert(from, (Direction3D::Front, Direction3D::Up));
                println!("({},{}) is the front side.", from_x, from_y);
            }
            if side_map.contains_key(&to) {
                continue;
            }
            if !side_map.contains_key(&from) {
                continue;
            }
            let (from_side, from_orientation) = *side_map.get(&from).unwrap();
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
            let next_side = convert_grid_direction(from_side, from_orientation, grid_direction);
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
                get_grid_up(next_side, opposite_direction(grid_direction), from_side);
            println!(
                "Grid-up on the {:?} side will be {:?}",
                next_side, next_orientation
            );
            side_map.insert(to, (next_side, next_orientation));
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
    (side_map, side_length)
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
    assert_eq!(solve_part_1(&test_input), 6032);
    //    let real_input = read_to_string("data/input22.txt").unwrap();
    //    println!("Part 1 solution: {:?}", solve_part_1(&real_input));
    assert_eq!(solve_part_2(&test_input), 5031);
}
