use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::FromIterator;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct SandMap {
    rocks: HashSet<Coords>,
    max_y: usize,
}

fn parse_coords(coords_str: &str) -> Coords {
    let mut number_strs = coords_str.split(",");
    let x = number_strs.next().unwrap().parse::<usize>().unwrap();
    let y = number_strs.next().unwrap().parse::<usize>().unwrap();
    Coords { x: x, y: y }
}

fn plot_path(from: &Coords, to: &Coords) -> Vec<Coords> {
    let mut output = vec![];
    if from.x == to.x {
        let lesser_y = min(from.y, to.y);
        let greater_y = max(from.y, to.y);
        for y in lesser_y..=greater_y {
            output.push(Coords { x: from.x, y: y });
        }
    } else if from.y == to.y {
        let lesser_x = min(from.x, to.x);
        let greater_x = max(from.x, to.x);
        for x in lesser_x..=greater_x {
            output.push(Coords { x: x, y: from.y });
        }
    } else {
        panic!("I did not expect the pair of {:?} and {:?}", from, to);
    }
    output
}

fn parse_path(path: &str) -> Vec<Coords> {
    let mut output = vec![];
    let mut pair_strs = path.split(" -> ");
    let mut previous_coords = parse_coords(pair_strs.next().unwrap());
    for next_str in pair_strs {
        let these_coords = parse_coords(next_str);
        let mut this_path = plot_path(&previous_coords, &these_coords);
        output.append(&mut this_path);
        previous_coords = these_coords;
    }
    output
}

fn parse_input(input: &str) -> SandMap {
    let mut output = vec![];
    for line in input.lines() {
        output.append(&mut parse_path(line));
    }
    let mut max_y = 0;
    for coords in output.iter() {
        if coords.y > max_y {
            max_y = coords.y;
        }
    }
    let final_rocks = HashSet::from_iter(output);
    SandMap {
        rocks: final_rocks,
        max_y: max_y,
    }
}

fn is_obstructed(state: &SandMap, coords: &Coords) -> bool {
    coords.y == state.max_y + 2 || state.rocks.contains(coords)
}

fn drop_sand(state: &mut SandMap) -> bool {
    // returns true IF IT CAME TO REST
    let mut position: Coords = Coords { x: 500, y: 0 };
    while position.y < state.max_y {
        let new_y = position.y + 1;
        if !is_obstructed(
            &state,
            &Coords {
                x: position.x,
                y: new_y,
            },
        ) {
            position = Coords {
                x: position.x,
                y: new_y,
            };
        } else if !is_obstructed(
            &state,
            &Coords {
                x: position.x - 1,
                y: new_y,
            },
        ) {
            position = Coords {
                x: position.x - 1,
                y: new_y,
            };
        } else if !is_obstructed(
            &state,
            &Coords {
                x: position.x + 1,
                y: new_y,
            },
        ) {
            position = Coords {
                x: position.x + 1,
                y: new_y,
            };
        } else {
            // we have come to rest at position.
            state.rocks.insert(position);
            return true;
        }
    }
    false
}

fn solve_part_1(input: &str) -> usize {
    let mut sand_map = parse_input(input);
    let mut grains_at_rest = 0;
    while drop_sand(&mut sand_map) {
        grains_at_rest += 1;
    }
    grains_at_rest
}

fn main() {
    let test_input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    println!("Part 1 test: {}", solve_part_1(test_input));
    let real_input = read_to_string("data/input14.txt").unwrap();
    println!("Part 1 solution: {}", solve_part_1(&real_input));
}
