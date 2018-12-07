#![feature(vec_remove_item)]

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct DependencyList {
    nodes: HashMap<char, Vec<char>>,
}

impl DependencyList {
    fn new() -> Self {
        DependencyList {
            nodes: HashMap::new()
        }
    }
    
    fn add_node(&mut self, node: char) {
        self.nodes.entry(node).or_insert(Vec::new());
    }

    fn add_dependency(&mut self, node: char, dep: char) {
        let deps = self.nodes.entry(node).or_insert(Vec::new());

        deps.push(dep);
    }

    fn next_steps(&self) -> Vec<char> {
        let mut next: Vec<char> = self.nodes
            .iter()
            .filter(|(k, v)| v.len() == 0)
            .map(|(k, _)| *k)
            .collect();

        next.sort();
        next
    }

    fn finish_step(&mut self) -> Option<char> {
        let finishable_deps = self.next_steps();
        let fst = finishable_deps.first()?;

        self.mark_finished(*fst);

        Some(*fst)
    }

    fn mark_finished(&mut self, node: char) {
        self.nodes.remove(&node);
        self.nodes.values_mut().for_each(|item| {
            item.remove_item(&node);
        });
    }
}

fn node_cost(node: char) -> u32 {
    (node as u32 - 'A' as u32) + 1
}

fn main() {
    let input: Vec<String> = shared::input::read_stdin_lines()
        .expect("could not lock stdin");

    let mut deplist = input.iter().filter_map(|line| {
        let mut spl = line.split(' ');

        Some((
            spl.nth(1)?.as_bytes()[0] as char,
            spl.nth(5)?.as_bytes()[0] as char
        ))
    }).fold(DependencyList::new(), |mut map, (dep, stp)| {
        map.add_node(dep);
        map.add_dependency(stp, dep);
        map
    });

    let mut deplist_p2 = deplist.clone();

    let mut part1 = String::new();

    while let Some(chr) = deplist.finish_step() {
        part1.push(chr);
    }
    
    println!("Part 1: {}", part1);

    // Part 2
    const WORKERS: usize = 5;
    const CONSTANT_OFFSET: u32 = 60;

    let mut workers: [(Option<char>, u32); WORKERS] = [(None, 0); WORKERS];
    let mut pending = HashSet::new();

    let mut next_steps = deplist_p2.next_steps();
    let mut res = String::new();

    for t in 0.. {
        for (held, tleft) in &mut workers {
            // Working on an item? Clock is ticking.
            if held.is_some() && *tleft > 0 {
                *tleft -= 1;

                if *tleft == 0 {
                    let n = held.take().unwrap();

                    res.push(n);

                    // Update the graph
                    deplist_p2.mark_finished(n);

                    // Calculate next steps, ignoring any still being
                    // worked on for now.
                    next_steps = deplist_p2.next_steps();
                    next_steps.retain(|n| !pending.contains(n));
                }
            }

            // If any more are available for this worker or any being
            // tried after the previous one dropped an item, start
            // working
            if held.is_none() && next_steps.len() > 0 {
                let n = next_steps.remove(0);

                held.replace(n);
                pending.insert(n);

                *tleft = CONSTANT_OFFSET + node_cost(n);
            }
        }

        if workers.iter().all(|(p, _)| p.is_none()) {
            println!("Part 2: {} after {} steps", res, t);
            break;
        }
    }
}