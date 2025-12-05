use std::fs::read_to_string;

fn best_two_digits(digits: &str) -> u32 {
    best_n_digits(digits, 2)
}

fn solve_part_1(input: &str) -> u32 {
    input.lines().map(best_two_digits).sum()
}

fn best_n_digits(digits: &str, batteries: usize) -> u32 {
    if digits.len() < batteries {
        panic!("The string can't be shorter than the number of batteries.");
    }
    let mut interim = vec![0];
    for digit_char in digits.chars().rev() {
        let digit = digit_char.to_digit(10).unwrap();
        if digit >= *interim.last().unwrap() {
            interim.push(digit);
        }
    }
    println!("My answer vector is {:?}", interim);
    let mut accu = 0;
    for _ in 0..batteries {
        accu = (10 * accu) + interim.pop().unwrap();
    }
    accu
}

fn main() {
    println!("should be 89: {}", best_two_digits("811111111111119"));
    println!("should be 88: {}", best_two_digits("811111111111118"));
    println!("should be 99: {}", best_two_digits("8999658"));
    println!("should be 92: {}", best_two_digits("818181911112111"));
    let test_input = "987654321111111
811111111111119
234234234234278
818181911112111";
    println!("should be 357: {}", solve_part_1(test_input));
    let real_input = read_to_string("data/input_2025_03.txt").unwrap();
    println!("part 1 answer: {}", solve_part_1(&real_input));
}
