use std::cmp::min;

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

fn main() {
    println!("{:?}", parse_stack_level("    [D]    "));
}
