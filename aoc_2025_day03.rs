use std::fs::read_to_string;

fn best_two_digits(digits: &str) -> u32 {
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    for digit_char in digits.chars().rev() {
        let digit = digit_char.to_digit(10).unwrap();
        if second == 0 {
            second = digit;
            continue;
        }
        if digit > first {
            if first > second {
                second = first;
            }
            first = digit;
        } else if digit == first && digit > second {
            second = digit;
        }
    }
    let answer = (first * 10) + second;
    // println!("{} -> {}", digits, answer);
    answer
}

fn solve_part_1(input: &str) -> u32 {
    input.lines().map(best_two_digits).sum()
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
