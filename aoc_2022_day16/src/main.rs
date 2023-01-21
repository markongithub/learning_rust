use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

type LabelBad = (char, char);
type Label = usize;

#[derive(Debug)]
struct ValveBad {
    flow_rate: usize,
    exits: Vec<LabelBad>,
}

struct Valve {
    flow_rate: usize,
    exits: Vec<Label>,
}

fn str_to_char_pair(two_char_string: &str) -> LabelBad {
    //    println!("The two character string is {}", two_char_string);
    let mut chars = two_char_string.chars();
    let first_char = chars.next().unwrap();
    let second_char = chars.next().unwrap();
    (first_char, second_char)
}

fn parse_valve(line: &str) -> (LabelBad, ValveBad) {
    // Valve BB has flow rate=13; tunnels lead to valves CC, AA
    // 012345678911234567892123456789312345678941234567895
    let l1: char = line.chars().nth(6).unwrap();
    let l2: char = line.chars().nth(7).unwrap();
    let semicolon_index = line.find(";").unwrap();
    let flow_rate = &line[23..semicolon_index].parse::<usize>().unwrap();
    let exits_index = if line.find("valves").is_some() {
        semicolon_index + 25
    } else {
        semicolon_index + 24
    };
    let exits_str = &line[exits_index..];

    let exit_pairs = exits_str
        .split(", ")
        .map(|s| str_to_char_pair(&s.to_string()))
        .collect();
    (
        (l1, l2),
        ValveBad {
            flow_rate: *flow_rate,
            exits: exit_pairs,
        },
    )
}

fn parse_input(input: &str) -> (Label, HashMap<Label, Valve>) {
    let valve_pairs = input.lines().map(parse_valve);
    let bad_valve_map: HashMap<LabelBad, ValveBad> = valve_pairs.collect();
    let mut label_index: HashMap<LabelBad, Label> = Default::default();
    let mut i: usize = 0;
    let mut all_labels: Vec<&LabelBad> = bad_valve_map.keys().collect();
    all_labels.sort();
    for bad_label in all_labels.iter() {
        label_index.insert(**bad_label, i);
        i += 1;
    }
    //    println!("Label index: {:?}", label_index);
    let mut output: HashMap<Label, Valve> = Default::default();
    for (k, v) in bad_valve_map.iter() {
        let mut better_exits: Vec<Label> = vec![];
        for exit in v.exits.iter() {
            better_exits.push(*label_index.get(exit).unwrap());
        }
        output.insert(
            *label_index.get(k).unwrap(),
            Valve {
                flow_rate: v.flow_rate,
                exits: better_exits,
            },
        );
    }
    let start_node = label_index.get(&('A', 'A')).unwrap();
    (*start_node, output)
}

fn floyd_warshall(graph: &HashMap<Label, Valve>) -> Vec<usize> {
    let mut dist: HashMap<(Label, Label), usize> = Default::default();
    for (label_k, valve_k) in graph.iter() {
        dist.insert((*label_k, *label_k), 0);
        for neighbor in valve_k.exits.iter() {
            dist.insert((*label_k, *neighbor), 1);
        }
    }
    for k in graph.keys() {
        for i in graph.keys() {
            for j in graph.keys() {
                if dist.contains_key(&(*i, *k)) && dist.contains_key(&(*k, *j)) {
                    let ikj_weight = dist.get(&(*i, *k)).unwrap() + dist.get(&(*k, *j)).unwrap();
                    if !dist.contains_key(&(*i, *j)) || (dist.get(&(*i, *j)).unwrap() > &ikj_weight)
                    {
                        dist.insert((*i, *j), ikj_weight);
                    }
                }
            }
        }
    }
    let num_vertices = graph.len();
    let mut output: Vec<usize> = vec![0; num_vertices * num_vertices];
    //    println!("I made a vec of length {}", output.len());
    for i in 0..num_vertices {
        for j in 0..num_vertices {
            output[(i * num_vertices) + j] = *dist.get(&(i, j)).unwrap();
        }
    }
    output
}

fn flow_rates_vec(graph: &HashMap<Label, Valve>) -> Vec<usize> {
    let mut output: Vec<usize> = vec![0; graph.len()];
    for (label_k, valve_k) in graph.iter() {
        output[*label_k] = valve_k.flow_rate;
    }
    output
}

fn cost_to_open_valve(
    cur_position: &Label,
    new_valve: &Label,
    total_vertices: &usize,
    weights: &Vec<usize>,
) -> usize {
    weights[(*cur_position * *total_vertices) + *new_valve] + 1
}

/*
fn try_ordering(
    valve_map: &HashMap<Label, Valve>,
    weights: &HashMap<(Label, Label), usize>,
    ordering: &Vec<Label>,
) -> (usize, usize) {
    // this should return what the total flow was, and how many valves we had opened
    let mut valves_open = 0;
    let mut total_flow = 0;
    let mut position = ('A', 'A');
    let mut time_left = 30;
    for valve in ordering {
        let this_cost = cost_to_open_valve(&position, &valve, weights);
        if this_cost >= time_left {
            return (total_flow, valves_open);
        }
        // now we do the move and the open
        time_left -= this_cost;
        valves_open += 1;
        total_flow += time_left * valve_map.get(&valve).unwrap().flow_rate;
        position = *valve;
    }
    return (total_flow, valves_open);
}
*/
fn try_permutations(
    flow_rates: &Vec<usize>,
    weights: &Vec<usize>,
    remaining_valves: &HashSet<Label>,
    time_left: usize,
    valves_open: usize,
    total_flow: usize,
    current_position: Label,
) -> (usize, usize) {
    let mut best_valves_open = valves_open;
    let mut best_total_flow = total_flow;
    let next_valves_open = valves_open + 1;

    /*    println!(
            "I am at {:?} with {} total flow, {} valves open, and {} time left.",
            current_position, total_flow, valves_open, time_left
        );
    */
    let total_vertices = flow_rates.len();
    for valve in remaining_valves.iter() {
        let this_cost = cost_to_open_valve(&current_position, &valve, &total_vertices, weights);
        if this_cost >= time_left {
            //            println!(
            //                "No point in visiting {:?} now, it will take too long.",
            //                valve
            //            );
            continue;
        }
        // now we do the move and the open
        let next_time_left = time_left - this_cost;
        let next_total_flow = total_flow + (next_time_left * flow_rates[*valve]);
        let next_position = *valve;
        let mut next_remaining_valves = remaining_valves.clone();
        next_remaining_valves.remove(&next_position);
        let (this_total_flow, this_valves_open) = try_permutations(
            flow_rates,
            weights,
            &next_remaining_valves,
            next_time_left,
            next_valves_open,
            next_total_flow,
            next_position,
        );
        if this_total_flow > best_total_flow {
            best_total_flow = this_total_flow;
            best_valves_open = this_valves_open;
        }
    }
    (best_total_flow, best_valves_open)
}

fn solve_part_1(input: &str) -> usize {
    let (start_node, mut valve_map) = parse_input(input);
    let weights = floyd_warshall(&valve_map);
    let flow_rates = flow_rates_vec(&valve_map);
    valve_map.retain(|_, v| v.flow_rate > 0);
    let (best_flow, _) = try_permutations(
        &flow_rates,
        &weights,
        &valve_map.keys().cloned().collect(),
        30,
        0,
        0,
        start_node,
    );
    best_flow
}

fn try_permutations2(
    flow_rates: &Vec<usize>,
    weights: &Vec<usize>,
    remaining_valves: &HashSet<Label>,
    time_left: [usize; 2],
    total_flow: usize,
    current_position: [Label; 2],
    mut best_total_flow: usize,
) -> usize {
    //    println!(
    //        "I am at {:?} with {} total flow, {} best_total_flow, and {:?} time left.",
    //        current_position, total_flow, best_total_flow, time_left
    //    );

    if total_flow > best_total_flow {
        best_total_flow = total_flow;
    }
    let mut theoretical_max = total_flow;
    let mut theoretical_time_left = time_left[0] + time_left[1];

    for valve in remaining_valves.iter() {
        if theoretical_time_left < 2 {
            break;
        }
        theoretical_time_left -= 2;
        theoretical_max += theoretical_time_left * flow_rates[*valve];
    }
    if theoretical_max <= best_total_flow {
        //       println!(
        //          "My theoretical max is {} so I will never be as good as {}. Now I get to skip {} factorial permutations.",
        //         theoretical_max, best_total_flow, remaining_valves.len()
        //    );
        return theoretical_max;
    }

    //    if theoretical_max <
    let mut remaining_valves_sorted = remaining_valves.into_iter().collect::<Vec<&Label>>();
    remaining_valves_sorted.sort_by_key(|label| flow_rates[**label]);
    remaining_valves_sorted.reverse();
    let total_vertices = flow_rates.len();
    for valve in remaining_valves_sorted.iter() {
        for player in 0..=1 {
            //            println!(
            //                "Let's think about if player {} went to {:?}...",
            //               player, valve,
            //            );
            let this_cost =
                cost_to_open_valve(&current_position[player], &valve, &total_vertices, weights);
            if this_cost >= time_left[player] {
                //                println!(
                //                    "No point in visiting {:?} now, it will take too long.",
                //                    valve
                //               );
                continue;
            }
            // now we do the move and the open
            let mut next_time_left: [usize; 2] = [0, 0]; // seems dumb
            next_time_left.copy_from_slice(&time_left);
            next_time_left[player] = time_left[player] - this_cost;
            let next_total_flow = total_flow + (next_time_left[player] * flow_rates[**valve]);
            let mut next_position: [Label; 2] = [69, 69];
            next_position.copy_from_slice(&current_position);
            next_position[player] = **valve;
            let mut next_remaining_valves = remaining_valves.clone();
            next_remaining_valves.remove(&next_position[player]);
            let this_total_flow = try_permutations2(
                flow_rates,
                weights,
                &next_remaining_valves,
                next_time_left,
                next_total_flow,
                next_position,
                best_total_flow,
            );
            //          println!("recursing returned {}", this_total_flow);
            if this_total_flow > best_total_flow {
                //            println!(
                //               "{} is better than my previous best of {}",
                //              this_total_flow, best_total_flow
                //         );
                best_total_flow = this_total_flow;
            } else {
                //       println!(
                //          "Apparently {} is less than than my previous best of {}",
                //          this_total_flow, best_total_flow
                //      );
            }
        }
    }
    //println!("I am about to return {}", best_total_flow);
    best_total_flow
}

fn solve_part_2(input: &str) -> usize {
    let (start_node, mut valve_map) = parse_input(input);
    let weights = floyd_warshall(&valve_map);
    let flow_rates = flow_rates_vec(&valve_map);
    valve_map.retain(|_, v| v.flow_rate > 0);
    let best_flow = try_permutations2(
        &flow_rates,
        &weights,
        &valve_map.keys().cloned().collect(),
        [26, 26],
        0,
        [start_node, start_node],
        0,
    );
    best_flow
}

fn main() {
    let test_input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    println!("Part 1 test: {}", solve_part_1(test_input));
    let real_input = read_to_string("../data/input16.txt").unwrap();
    println!("Part 1 solution: {}", solve_part_1(&real_input));
    println!("Part 2 test: {}", solve_part_2(test_input));
    println!("Starting part 2 solution. See you in a minute...");
    println!("Part 2 solution: {}", solve_part_2(&real_input));
}
