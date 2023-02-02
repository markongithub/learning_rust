use std::fs::read_to_string;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>, Vec<isize>) {
    // I need one index from IDs to values
    // then an index from IDs to positions
    let mut id_to_position: Vec<usize> = vec![];
    let mut position_to_id: Vec<usize> = vec![];
    let mut id_to_value: Vec<isize> = vec![];
    let mut index = 0;
    for line in input.lines() {
        let value = line.parse::<isize>().unwrap();
        id_to_value.push(value);
        id_to_position.push(index);
        position_to_id.push(index);
        index += 1;
    }
    (id_to_position, position_to_id, id_to_value)
}

fn get_new_index(original_index: usize, offset: isize, size: usize) -> usize {
    let after_cycles: usize = (offset.wrapping_abs() as usize) % (size - 1);
    if offset < 0 {
        if after_cycles > original_index {
            original_index + (size - 1) - after_cycles
        } else {
            original_index - after_cycles
        }
    } else {
        (original_index + after_cycles) % (size - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_new_index() {
        assert_eq!(get_new_index(0, -1, 10), 8);
        assert_eq!(get_new_index(1, -7, 10), 3);
        assert_eq!(get_new_index(1, 9, 10), 1);
        // -3 should go to 0, 6, 5
        // NO! It should to go 0, 5, 4. 6 is redundant.
        assert_eq!(get_new_index(1, -3, 7), 4);
    }
}

fn mix_one_value(
    id_to_position: &mut Vec<usize>,
    position_to_id: &mut Vec<usize>,
    id_to_value: &Vec<isize>,
    id_to_move: usize,
) {
    let offset = id_to_value[id_to_move];
    let original_index = id_to_position[id_to_move];
    let total_positions = id_to_value.len();
    let new_index = get_new_index(original_index, offset, total_positions);

    if new_index < original_index {
        for index in (new_index..original_index).rev() {
            /*
            original index was 5, new is 1
            so I set the contents of 5 as the contents of 4
            down to 2 is 1
            so I iterate from 1 to 4 rev and do N+1
            then 1 is offset

            */
            position_to_id[index + 1] = position_to_id[index];
            id_to_position[position_to_id[index + 1]] = index + 1;
        }
    } else {
        for index in original_index..new_index {
            /*
            original index was 1, new is 5
            I set 1 as the former 2
            up to 4 as the former 5
            */
            position_to_id[index] = position_to_id[index + 1];
            id_to_position[position_to_id[index]] = index;
        }
    }
    position_to_id[new_index] = id_to_move;
    id_to_position[id_to_move] = new_index;
}

fn values_in_order(position_to_id: &Vec<usize>, id_to_value: &Vec<isize>) -> Vec<isize> {
    let mut output = vec![];
    for id in position_to_id.iter() {
        output.push(id_to_value[*id]);
    }
    output
}

fn grove_values(
    id_to_position: &Vec<usize>,
    position_to_id: &Vec<usize>,
    id_to_value: &Vec<isize>,
) -> Vec<isize> {
    let mut id_of_zero = 0;
    loop {
        if id_to_value[id_of_zero] == 0 {
            break;
        } else {
            id_of_zero += 1;
        }
    }
    let position_of_zero = id_to_position[id_of_zero];
    let size = id_to_position.len();
    let mut output = vec![];
    output.push(id_to_value[position_to_id[(position_of_zero + 1000) % size]]);
    output.push(id_to_value[position_to_id[(position_of_zero + 2000) % size]]);
    output.push(id_to_value[position_to_id[(position_of_zero + 3000) % size]]);
    output
}

fn solve_part_1(input: &str) -> isize {
    let (mut id_to_position, mut position_to_id, id_to_value) = parse_input(&input);
    for i in 0..id_to_position.len() {
        mix_one_value(&mut id_to_position, &mut position_to_id, &id_to_value, i);
    }
    // println!("After mixing: {:?}", values_in_order(&position_to_id, &id_to_value));
    grove_values(&id_to_position, &position_to_id, &id_to_value)
        .iter()
        .sum()
}

fn main() {
    let test_input = "1
2
-3
3
-2
0
4";
    println!("Part 1 test: {}", solve_part_1(&test_input));
    let real_input = read_to_string("../data/input20.txt").unwrap();
    println!("Part 1 solution: {}", solve_part_1(&real_input));
}
