use std::cmp::Ordering;

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
                println!("I got the number {} before a comma", number_str);
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
        match (&self, &other) {
            (Packet::ValueP(n1), Packet::ValueP(n2)) => n1.cmp(&n2),
            (Packet::ListP(v1), Packet::ListP(v2)) => {
                if v1.is_empty() && v2.is_empty() {
                    Ordering::Equal
                } else if v1.is_empty() {
                    Ordering::Less
                } else if v2.is_empty() {
                    Ordering::Greater
                } else {
                    // both vectors are unempty!
                    let _head1 = &v1[0];
                    let _head2 = &v2[0];
                    todo!("lol");
                }
            }
            (_foo, _bar) => todo!("lol"),
        }
    }
}
fn main() {
    println!("{:?}", parse_packet("[[1],[123,3,4]]"));
    println!("{:?}", parse_packet("[[4,4],4,4]"));
}
