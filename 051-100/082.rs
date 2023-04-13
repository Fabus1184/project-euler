// cargo-deps: file-fetcher = "0.1.4", priority-queue = "1.3.1"

use std::io::Read;
use std::collections::HashMap;
use priority_queue::PriorityQueue;

let matrix = file_fetcher::open_str("https://projecteuler.net/project/resources/p082_matrix.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .lines()
    .map(|line| {
        line.split(',')
            .map(|x| x.parse::<i32>().unwrap() as i64)
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

fn dijkstra<T: std::cmp::Eq + std::hash::Hash + Copy>(
    start: T,
    neighbors: &dyn Fn(T) -> Vec<T>,
    cost: &dyn Fn(T, T) -> i64,
    end: &dyn Fn(T) -> bool
) -> Option<Vec<T>> {
    let mut map: HashMap<T, (T, i64)> = HashMap::from([(start, (start, 0))]);
    let mut queue = PriorityQueue::new();
    // attention: the priority queue returns the biggest element first
    queue.push(start, -0);
    while let Some((node, _)) = queue.pop() {
        if end(node) {
            let mut path = vec![node];
            let mut current = node;
            while let Some((parent, _)) = map.get(&current) {
                if *parent == current {
                    break;
                }

                path.push(*parent);
                current = *parent;
            }
            path.reverse();
            return Some(path);
        }
        for neighbor in neighbors(node) {
            let new_cost = map[&node].1 + cost(node, neighbor);
            if !map.contains_key(&neighbor) || map[&neighbor].1 > new_cost {
                map.insert(neighbor, (node, new_cost));
                queue.push(neighbor, -new_cost);
            }
        }
    }
    None
}

println!("{:?}", 
    (0..matrix.len())
        .map(|i| dijkstra((i, 0),
            &|(row, col)| {
                let mut v = vec![];
                if row > 0 {
                    v.push((row - 1, col));
                }
                if row < matrix.len() - 1 {
                    v.push((row + 1, col));
                }
                if col < matrix.len() - 1 {
                    v.push((row, col + 1));
                }
                v
            },
            &|_, (row, col)| matrix[row][col],
            &|(_, col)| col == matrix.len() - 1)
            .map(|x| x.iter().map(|(row, col)| matrix[*row][*col]).sum::<i64>())
            .unwrap_or(0)
        )
        .min()
        .unwrap()
);
