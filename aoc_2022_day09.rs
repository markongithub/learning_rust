use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Motion {
    direction: char,
    steps: usize,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coords {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct RopeState {
    head: Coords,
    tail: Coords,
    visited: HashSet<Coords>,
}

fn parse_motion(line: &str) -> Motion {
    let mut words: Vec<&str> = line.split_whitespace().collect();
    let steps_str = words.pop().unwrap();
    let steps: usize = steps_str.parse::<usize>().unwrap();
    let direction: char = words.pop().unwrap().chars().next().unwrap();
    Motion {
        direction: direction,
        steps: steps,
    }
}

fn parse_input(one_big_string: &str) -> Vec<Motion> {
    let lines = one_big_string.lines();
    let motions = lines.map(parse_motion).collect();
    motions
}

fn move_head(old: Coords, direction: char) -> Coords {
    match direction {
        'U' => Coords {
            x: old.x,
            y: old.y + 1,
        },
        'D' => Coords {
            x: old.x,
            y: old.y - 1,
        },
        'L' => Coords {
            x: old.x - 1,
            y: old.y,
        },
        'R' => Coords {
            x: old.x + 1,
            y: old.y,
        },
        c => panic!("No one expects direction {}", c),
    }
}

fn move_one_step(old: RopeState, direction: char) -> RopeState {
    let new_head = move_head(old.head, direction);

    let new_tail = if new_head.x.abs_diff(old.tail.x) <= 1 && new_head.y.abs_diff(old.tail.y) <= 1 {
        old.tail
    } else if new_head.x == old.tail.x + 2 {
        Coords {
            x: old.tail.x + 1,
            y: new_head.y,
        }
    } else if new_head.x == old.tail.x - 2 {
        Coords {
            x: old.tail.x - 1,
            y: new_head.y,
        }
    } else if new_head.y == old.tail.y + 2 {
        Coords {
            x: new_head.x,
            y: old.tail.y + 1,
        }
    } else if new_head.y == old.tail.y - 2 {
        Coords {
            x: new_head.x,
            y: old.tail.y - 1,
        }
    } else {
        panic!("I fucked up the head/tail cases.")
    };
    let mut visited = old.visited; // I want to take the value here
    visited.insert(Coords {
        x: new_tail.x,
        y: new_tail.y,
    });
    let output = RopeState {
        head: new_head,
        tail: new_tail,
        visited: visited,
    };
    //    println!("{:?}", output);
    output
}

fn run_motion(old: RopeState, motion: &Motion) -> RopeState {
    /*println!(
        "We are at {},{} and will now go {} by {} steps.",
        old.head.x, old.head.y, motion.direction, motion.steps
    );*/
    let mut output = RopeState {
        head: old.head,
        tail: old.tail,
        visited: old.visited,
    };
    for _i in 1..=motion.steps {
        output = move_one_step(output, motion.direction);
    }
    output
}

fn run_motions(motions: Vec<Motion>) -> RopeState {
    let mut motions_iter = motions.iter();
    let first_motion = motions_iter.next().unwrap();
    let initial_head = move_head(Coords { x: 0, y: 0 }, first_motion.direction);
    let initial_tail = Coords { x: 0, y: 0 };
    let mut initial_visited: HashSet<Coords> = HashSet::new();
    initial_visited.insert(Coords { x: 0, y: 0 });

    let rest_of_first_motion = Motion {
        direction: first_motion.direction,
        steps: first_motion.steps - 1,
    };
    let mut next_state = run_motion(
        RopeState {
            head: initial_head,
            tail: initial_tail,
            visited: initial_visited,
        },
        &rest_of_first_motion,
    );
    for motion in motions_iter {
        next_state = run_motion(next_state, motion);
    }
    next_state
}

fn solve_part_1(input: &str) -> usize {
    let motions = parse_input(input);
    let final_state = run_motions(motions);
    final_state.visited.len()
}

fn main() {
    let test_input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    println!("Part 1 test: {}", solve_part_1(test_input));
    let real_input = read_to_string("data/input09.txt").unwrap();
    println!("Part 1 solution: {:?}", solve_part_1(&real_input));
}
