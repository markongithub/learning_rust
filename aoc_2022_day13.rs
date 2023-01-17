use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    ValueP(usize),
    ListP(Vec<Packet>),
}

fn parse_packet(input: &str) -> Packet {
    // I think we need a vec of vecs to track our descent into sublists.
    // I push some stuff onto my current vec
    // then I see a left bracket and a new list starts
    // I push my current vec onto the stack vec.
    // I start a new vec
    // eventually I hit a right bracket
    // I pop the stack vec
    // I push my current ListP onto that
    // I keep pushing stuff onto that
    let mut stack_of_vecs: Vec<Vec<Packet>> = vec![];
    let mut current_vec: Vec<Packet> = vec![];
    let mut current_digits: Vec<char> = vec![];
    let mut input_chars = input.chars();
    while let Some(next_char) = input_chars.next() {
        //println!("Considering the character {}...", next_char);
        if next_char.is_digit(10) {
            current_digits.push(next_char);
        } else if next_char == ',' {
            if !current_digits.is_empty() {
                let number_str: String = current_digits.into_iter().collect();
                //println!("I got the number {} before a comma", number_str);
                let number_value = number_str.parse::<usize>().unwrap();
                current_digits = vec![];
                current_vec.push(Packet::ValueP(number_value));
            }
        } else if next_char == ']' {
            if !current_digits.is_empty() {
                let number_str: String = current_digits.into_iter().collect();
                // println!("I got the number {} before a comma", number_str);
                let number_value = number_str.parse::<usize>().unwrap();
                current_digits = vec![];
                current_vec.push(Packet::ValueP(number_value));
            }
            let new_packet = Packet::ListP(current_vec);
            current_vec = stack_of_vecs.pop().unwrap();
            current_vec.push(new_packet);
        } else if next_char == '[' {
            stack_of_vecs.push(current_vec);
            current_vec = vec![];
        }
    }
    current_vec.pop().unwrap()
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        //        println!("Comparing packets of {:?} and {:?}", self, other);
        match (&self, &other) {
            (Packet::ValueP(n1), Packet::ValueP(n2)) => n1.cmp(&n2),
            (Packet::ListP(v1), Packet::ListP(v2)) => {
                let mut v1_iter = v1.iter();
                let mut v2_iter = v2.iter();
                loop {
                    let v1_next = v1_iter.next();
                    let v2_next = v2_iter.next();
                    if v1_next.is_none() && v2_next.is_none() {
                        //                        println!("Two lists ended at the same time, how do we break out of this?");
                        return Ordering::Equal;
                    } else if v1_next.is_none() {
                        return Ordering::Less;
                    } else if v2_next.is_none() {
                        return Ordering::Greater;
                    } else {
                        // both vectors are unempty!
                        let head1 = v1_next.unwrap();
                        let head2 = v2_next.unwrap();
                        //                        println!("Comparing heads of {:?} and {:?}", head1, head2);
                        let head_cmp = head1.cmp(head2);
                        if head_cmp != Ordering::Equal {
                            //                            println!("That was easy, we have a verdict.");
                            return head_cmp;
                        }
                        //                       println!("The heads were equal, we will have to keep looping.");
                    }
                }
            }
            (Packet::ValueP(n1), other_list) => {
                let mut new_vec: Vec<Packet> = vec![];
                new_vec.push(Packet::ValueP(*n1));
                Packet::ListP(new_vec).cmp(other_list)
            }
            (other_list, Packet::ValueP(n2)) => {
                let mut new_vec: Vec<Packet> = vec![];
                new_vec.push(Packet::ValueP(*n2));
                other_list.cmp(&&Packet::ListP(new_vec))
            }
        }
    }
}

fn solve_part_1(input: &str) -> usize {
    let mut first_line: &str = "pointless default value";
    let mut got_first_line: bool = false;
    let mut pair_index = 0;
    let mut sum_of_good_indices = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        } else if !got_first_line {
            pair_index += 1;
            first_line = line;
            got_first_line = true;
        } else {
            let packet1 = parse_packet(first_line);
            let packet2 = parse_packet(line);
            got_first_line = false;
            if packet1 < packet2 {
                //                println!("{} was correctly before {}", first_line, line);
                sum_of_good_indices += pair_index;
            } else if packet1 > packet2 {
                //              println!("{} should NOT have been before {}", first_line, line);
            } else {
                panic!(
                    "Oh god they are equal or I fucked up the ordering: {} and {}",
                    first_line, line
                );
            }
        }
    }
    sum_of_good_indices
}

fn solve_part_2(input: &str) -> usize {
    let mut main_vec: Vec<Packet> = vec![];
    for line in input.lines() {
        if !line.is_empty() {
            main_vec.push(parse_packet(line));
        }
    }
    main_vec.push(parse_packet("[[2]]"));
    main_vec.push(parse_packet("[[6]]"));
    main_vec.sort();
    let parsed_two = parse_packet("[[2]]");
    let parsed_six = parse_packet("[[6]]");
    let mut found_two = false;
    let mut index_of_two = 9999;
    let mut current_index = 0;
    for packet in main_vec.iter() {
        current_index += 1;
        if !found_two && *packet == parsed_two {
            found_two = true;
            index_of_two = current_index;
        } else if found_two && *packet == parsed_six {
            return current_index * index_of_two;
        }
    }
    panic!("I got through the vector and I didn't find the dividers.");
}

fn main() {
    let test_input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    println!("Part 1 test: {}", solve_part_1(test_input));
    let real_input = read_to_string("data/input13.txt").unwrap();
    println!("Part 1 solution: {}", solve_part_1(&real_input));
    println!("Part 2 test: {}", solve_part_2(test_input));
    println!("Part 2 solution: {}", solve_part_2(&real_input));
}
