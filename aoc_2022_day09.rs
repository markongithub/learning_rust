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
    knots: Vec<Coords>,
    visited: HashSet<Coords>,
    time: usize,
    max_knots: usize,
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

fn move_head(old: &Coords, direction: char) -> Coords {
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
    let mut new_knots: Vec<Coords> = vec![];

    let mut knots_iter = old.knots.iter();
    let old_head: &Coords = knots_iter.next().unwrap();
    let mut new_head = move_head(old_head, direction);
    let mut new_head_copy = Coords {
        x: new_head.x,
        y: new_head.y,
    };
    new_knots.push(new_head_copy);
    for old_knot in knots_iter {
        new_head = update_tail_from_head(&new_head, old_knot);
        // println!("... so I moved to {:?}", new_head);
        new_head_copy = Coords {
            x: new_head.x,
            y: new_head.y,
        };
        new_knots.push(new_head_copy);
    }
    // we still haven't updated visited
    // we only do that when we place the last knot
    // I think maybe instead of an iterator we want to use indices?
    // no wait, we still have one new_head left over from up above.
    let mut visited = old.visited; // I want to take the value here
    visited.insert(new_head);
    let output = RopeState {
        knots: new_knots,
        visited: visited,
        time: old.time + 1,
        max_knots: old.max_knots,
    };
    // println!("{:?}", output);
    output
}

fn run_motion(old: RopeState, motion: &Motion) -> RopeState {
    /*println!(
        "We are at {},{} and will now go {} by {} steps.",
        old.head.x, old.head.y, motion.direction, motion.steps
    );*/
    // wait, why am I copying this?
    let mut output = RopeState {
        knots: old.knots,
        visited: old.visited,
        time: old.time,
        max_knots: old.max_knots,
    };
    for _i in 1..=motion.steps {
        output = move_one_step(output, motion.direction);
    }
    output
}

fn run_motions(motions: Vec<Motion>, max_knots: usize) -> RopeState {
    let mut initial_knots = vec![];
    for _i in 1..=max_knots {
        initial_knots.push(Coords { x: 0, y: 0 });
    }

    let mut initial_visited = HashSet::new();
    initial_visited.insert(Coords { x: 0, y: 0 });
    let mut next_state = RopeState {
        knots: initial_knots,
        visited: initial_visited,
        time: 1,
        max_knots: max_knots,
    };
    for motion in motions.iter() {
        next_state = run_motion(next_state, motion);
    }
    next_state
}

fn solve_part_1(input: &str) -> usize {
    let motions = parse_input(input);
    let final_state = run_motions(motions, 2);
    final_state.visited.len()
}

fn solve_part_2(input: &str) -> usize {
    let motions = parse_input(input);
    let final_state = run_motions(motions, 10);
    final_state.visited.len()
}

fn update_tail_from_head(new_head: &Coords, old_tail: &Coords) -> Coords {
    //println!(
    //    "I am at {:?} and my leader is at {:?}...",
    //    old_tail, new_head
    //);
    let new_tail = if new_head.x.abs_diff(old_tail.x) <= 1 && new_head.y.abs_diff(old_tail.y) <= 1 {
        Coords {
            x: old_tail.x,
            y: old_tail.y,
        }
    } else if (new_head.x == old_tail.x + 2) && (new_head.y == old_tail.y + 2) {
        Coords {
            x: old_tail.x + 1,
            y: old_tail.y + 1,
        }
    } else if new_head.x == old_tail.x + 2 && new_head.y == old_tail.y - 2 {
        Coords {
            x: old_tail.x + 1,
            y: old_tail.y - 1,
        }
    } else if new_head.x == old_tail.x - 2 && new_head.y == old_tail.y + 2 {
        Coords {
            x: old_tail.x - 1,
            y: old_tail.y + 1,
        }
    } else if new_head.x == old_tail.x - 2 && new_head.y == old_tail.y - 2 {
        Coords {
            x: old_tail.x - 1,
            y: old_tail.y - 1,
        }
    } else if new_head.x == old_tail.x + 2 {
        Coords {
            x: old_tail.x + 1,
            y: new_head.y,
        }
    } else if new_head.x == old_tail.x - 2 {
        Coords {
            x: old_tail.x - 1,
            y: new_head.y,
        }
    } else if new_head.y == old_tail.y + 2 {
        Coords {
            x: new_head.x,
            y: old_tail.y + 1,
        }
    } else if new_head.y == old_tail.y - 2 {
        Coords {
            x: new_head.x,
            y: old_tail.y - 1,
        }
    } else {
        panic!(
            "I fucked up the head/tail cases. new_head: {:?}, old_tail: {:?}",
            new_head, old_tail
        )
    };
    if manhattan_distance(&new_tail, old_tail) > 2 {
        panic!("Why did we move from {:?} to {:?}?", old_tail, new_tail)
    }
    new_tail
}

fn manhattan_distance(a: &Coords, b: &Coords) -> u64 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
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
    let test_input_2 = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    println!("Part 1 test: {}", solve_part_1(test_input));
    let real_input = read_to_string("data/input09.txt").unwrap();
    println!("Part 1 solution: {:?}", solve_part_1(&real_input));
    println!("Part 2 test 1: {}", solve_part_2(test_input));
    println!("Part 2 test 2: {}", solve_part_2(test_input_2));
    println!("Part 2 solution: {:?}", solve_part_2(&real_input));
}
