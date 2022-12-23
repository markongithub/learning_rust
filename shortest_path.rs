use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, PartialEq)]
struct Edge {
    dest: char,
    w: u8,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
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
    let edges: HashMap<char, Vec<Edge>> = HashMap::from([
        (
            'a',
            vec![
                Edge { dest: 'b', w: 1 },
                Edge { dest: 'c', w: 1 },
                Edge { dest: 'e', w: 1 },
            ],
        ),
        ('b', vec![Edge { dest: 'c', w: 1 }]),
        (
            'c',
            vec![Edge { dest: 'd', w: 1 }, Edge { dest: 'e', w: 1 }],
        ),
        (
            'd',
            vec![Edge { dest: 'e', w: 1 }, Edge { dest: 'f', w: 1 }],
        ),
        ('e', vec![Edge { dest: 'f', w: 1 }]),
    ]);
    let output = dijkstra(edges, 'a', 'f');
    println!("{0:?}", traceback(output, 'a', 'f'));
    println! {"That's the name of the game."}
}

fn traceback(distances: HashMap<char, (u8, char)>, source: char, dest: char) -> Vec<char> {
    let mut output: Vec<char> = vec![];
    let mut current: char = dest;
    let mut _dist: u8;
    while current != source {
        output.push(current);
        (_dist, current) = *distances.get(&current).unwrap();
    }
    output.push(source);
    output.reverse();
    return output;
}

fn dijkstra(
    edges: HashMap<char, Vec<Edge>>,
    source: char,
    goal: char,
) -> HashMap<char, (u8, char)> {
    let mut distances: HashMap<char, (u8, char)> = Default::default();
    let mut queue: BinaryHeap<Edge> = BinaryHeap::new();
    let mut visited: HashSet<char> = Default::default();

    let mut current: Edge;
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
