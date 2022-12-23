use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Eq, PartialEq)]
struct Edge<T> {
    dest: T,
    w: u8,
}

impl<T: Eq + Ord> PartialOrd for Edge<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + Ord> Ord for Edge<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .w
            .cmp(&self.w)
            .then_with(|| self.dest.cmp(&other.dest))
    }
}

fn main() {
    let edges: HashMap<char, Vec<Edge<char>>> = HashMap::from([
        (
            'a',
            vec![
                Edge { dest: 'b', w: 1 },
                Edge { dest: 'c', w: 9 },
                Edge { dest: 'e', w: 9 },
            ],
        ),
        ('b', vec![Edge { dest: 'c', w: 1 }]),
        (
            'c',
            vec![Edge { dest: 'd', w: 1 }, Edge { dest: 'e', w: 9 }],
        ),
        (
            'd',
            vec![Edge { dest: 'e', w: 1 }, Edge { dest: 'f', w: 9 }],
        ),
        ('e', vec![Edge { dest: 'f', w: 1 }]),
    ]);
    let output = dijkstra(edges, 'a', 'f');
    println!("{0:?}", traceback(output, 'a', 'f'));
    println! {"That's the name of the game."}
}

fn traceback<T: Eq + Hash + Copy>(distances: HashMap<T, (u8, T)>, source: T, dest: T) -> Vec<T> {
    let mut output: Vec<T> = vec![];
    let mut current: T = dest;
    let mut _dist: u8;
    while current != source {
        output.push(current);
        (_dist, current) = *distances.get(&current).unwrap();
    }
    output.push(source);
    output.reverse();
    return output;
}

fn dijkstra<T: Eq + Ord + Hash + Display + Copy>(
    edges: HashMap<T, Vec<Edge<T>>>,
    source: T,
    goal: T,
) -> HashMap<T, (u8, T)> {
    let mut distances: HashMap<T, (u8, T)> = Default::default();
    let mut queue: BinaryHeap<Edge<T>> = BinaryHeap::new();
    let mut visited: HashSet<T> = Default::default();

    let mut current: Edge<T>;
    queue.push(Edge { dest: source, w: 0 });
    while !queue.is_empty() {
        current = queue.pop().unwrap();
        if current.dest == goal {
            break;
        }
        visited.insert(current.dest);
        //let mut neighbor: Edge;
        for neighbor in edges.get(&current.dest).unwrap() {
            if visited.contains(&neighbor.dest) {
                println!(
                    "We will skip {0} this time because we visited it already.",
                    neighbor.dest
                );
                continue;
            }
            let alt: u8 = current.w + neighbor.w;
            let is_better: bool = match distances.get(&neighbor.dest) {
                Some((distance, _pred)) => alt < *distance,
                None => true,
            };
            if is_better {
                println!(
                    "I found a better route to {0} through {1}",
                    neighbor.dest, current.dest
                );

                distances.insert(neighbor.dest, (alt, current.dest));
                queue.push(Edge {
                    dest: neighbor.dest,
                    w: alt,
                });
            }
        }
    }
    return distances;
}
