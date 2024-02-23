#!/usr/bin/env rust-script

// cargo-deps: primal="0.3.2"

fn is_concateable(a: u64, b: u64) -> bool {
    if !primal::is_prime(format!("{}{}", a, b).parse().unwrap()) {
        return false;
    }
    if !primal::is_prime(format!("{}{}", b, a).parse().unwrap()) {
        return false;
    }
    true
}

#[derive(Debug)]
enum Node {
    Root(Vec<Node>),
    Leaf(u64, Vec<Node>),
}

fn insert_prime(prime: u64, node: &mut Node) -> Option<Vec<u64>> {
    match node {
        Node::Root(children) => {
            Some(children
                .iter_mut()
                .filter_map(|child| insert_prime(prime, child))
                .max_by_key(|s| s.len())
                .unwrap_or_else(|| {
                    children.push(Node::Leaf(prime, vec![]));
                    vec![prime]
                }))
        },
        Node::Leaf(p, children) => {
            if !is_concateable(prime, *p) {
                return None;
            }
            
            if let Some(mut v) = children
                .iter_mut()
                .filter_map(|child| insert_prime(prime, child))
                .max_by_key(|s| s.len())
            {
                v.push(*p);
                Some(v)
            } else {
                children.push(Node::Leaf(prime, vec![]));
                Some(vec![prime, *p])
            }
        }
    }
}

let mut tree = Node::Root(vec![]);

for p in primal::Primes::all() {
    let v = insert_prime(p as u64, &mut tree).unwrap();
    if v.len() == 5 {
        println!("{}", v.iter().sum::<u64>());
        return Ok(());
    }
}
