use std::cmp::min;
use std::fs::read_to_string;

#[derive(Debug)]
struct Instruction {
    quantity: i64,
    from: usize,
    to: usize,
}

//123456789112345678921234567893123456789
//[Q]         [M] [P]     [Q]     [J];
//[V]         [T]         [J]        ;

fn parse_stack_level(level: &str) -> Vec<char> {
    let mut output = Vec::new();

    let mut cur = level;
    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(min(4, cur.len()));
        output.push(chunk.chars().nth(1).unwrap());
        cur = rest;
    }
    output
}

fn parse_stacks(rows: Vec<&str>) -> Vec<Vec<char>> {
    let num_stacks = parse_stack_level(rows[0]).len();
    let mut stacks: Vec<Vec<char>> = vec![];
    for _n in 1..=num_stacks {
        let next_vec = vec![];
        stacks.push(next_vec);
    }
    for row in rows {
        let columns: Vec<char> = parse_stack_level(row);
        for (stack, value) in stacks.iter_mut().zip(columns.iter()) {
            if *value != ' ' {
                stack.push(*value);
            }
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    stacks
}

fn parse_instruction(line: &str) -> Instruction {
    // move 10 from 7 to 6
    let words: Vec<&str> = line.split_whitespace().collect();
    let quantity: i64 = words[1].parse::<i64>().unwrap();
    let from: usize = words[3].parse::<usize>().unwrap();
    let to: usize = words[5].parse::<usize>().unwrap();
    Instruction {
        quantity: quantity,
        from: from,
        to: to,
    }
}

fn parse_input(one_big_string: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut lines = one_big_string.lines();
    let stack_lines = lines.take_while(|s| !s.starts_with(" 1")).collect();
    let stacks = parse_stacks(stack_lines);
    // there has to be a better way to do this
    lines = one_big_string.lines();
    let instruction_lines = lines.skip_while(|s| !s.starts_with("move"));
    let instructions = instruction_lines.map(parse_instruction).collect();
    (stacks, instructions)
}

fn move_one_crate(stacks: &mut Vec<Vec<char>>, from: usize, to: usize) {
    let value_to_move = stacks[from - 1].pop().unwrap();
    stacks[to - 1].push(value_to_move);
}

fn run_instruction(stacks: &mut Vec<Vec<char>>, instruction: &Instruction) {
    for _i in 1..=instruction.quantity {
        move_one_crate(stacks, instruction.from, instruction.to);
    }
}

fn run_instructions(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) {
    for instruction in instructions.iter() {
        run_instruction(stacks, instruction);
    }
}

fn top_of_each_stack(stacks: &Vec<Vec<char>>) -> Vec<char> {
    stacks.iter().map(|s| *s.last().unwrap()).collect()
}

fn solve_part_1(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);
    run_instructions(&mut stacks, &instructions);
    top_of_each_stack(&stacks).iter().collect()
}

fn main() {
    let test_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    println!("Part 1 test: {:?}", solve_part_1(test_input));
    println!(
        "Part 1 solution: {:?}",
        solve_part_1(&read_to_string("data/input05.txt").unwrap())
    );
}
