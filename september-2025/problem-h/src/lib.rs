use std::fmt::{Display, Write};

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

use common::*;

const HOME: &str = "BATUMI";

#[derive(Debug, Clone)]
struct State {
    used_tickets: u128,
    position: u8,
    bought: Vec<(u8, u8)>,
}

impl State {
    fn new(position: u8) -> Self {
        Self {
            used_tickets: 0,
            position,
            bought: Vec::new(),
        }
    }

    fn push(&mut self, ticket: u8) {
        self.used_tickets |= 1 << ticket;
    }

    fn pop(&mut self, ticket: u8) {
        self.used_tickets &= !(1 << ticket);
    }

    fn has(&self, ticket: u8) -> bool {
        self.used_tickets & (1 << ticket) != 0
    }

    fn count(&self) -> u8 {
        self.used_tickets.count_ones() as _
    }
}

struct Searcher {
    nodes: Vec<&'static str>,
    best_bought_count: usize,
    best_bought: Vec<(u8, u8)>,

    ticket_count: u8,
    home: u8,
    state: State,
    seen: HashMap<(u8, u8), u8>,
}

impl Searcher {
    fn new(
        tickets: &[(&'static str, &'static str)],
    ) -> (Self, HashMap<u8, Vec<(u8, (u8, u8))>>) {
        let mut nodes = HashSet::default();
        nodes.insert(HOME);

        for ticket in tickets {
            nodes.insert(ticket.0);
            nodes.insert(ticket.1);
        }

        let mut nodes: Vec<&'static str> = nodes.into_iter().collect();
        nodes.sort();

        let node_map: HashMap<&'static str, u8> =
            nodes.iter().enumerate().map(|(i, n)| (*n, u8::try_from(i).expect("Too many nodes"))).collect();

        let home = node_map[HOME];

        let tickets: Vec<(u8, u8)> = tickets
            .into_iter()
            .map(|ticket| {
                let a = node_map[&ticket.0];
                let b = node_map[&ticket.1];
                if a < b { (a, b) } else { (b, a) }
            })
            .collect();

        let state = State::new(home);

        // Build candidate cache
        let candidate_cache = (0..nodes.len() as u8)
            .map(|node| {
                let mut candidates: Vec<(u8, (u8, u8))> = tickets
                    .iter()
                    .enumerate()
                    .map(|(i, &ticket)| (i as u8, ticket))
                    .collect();

                // Sort by whether the node is in the ticket (reverse order for priority)
                candidates.sort_by_key(|(_, ticket)| {
                    if ticket.0 == node || ticket.1 == node {
                        0
                    } else {
                        1
                    }
                });

                (node, candidates)
            })
            .collect();

        (
            Self {
                nodes,
                best_bought_count: usize::MAX,
                best_bought: Vec::new(),
                ticket_count: u8::try_from(tickets.len()).expect("Too many tickets"),
                home,
                state,
                seen: HashMap::default(),
            },
            candidate_cache,
        )
    }

    fn search(&mut self, candidate_cache: &HashMap<u8, Vec<(u8, (u8, u8))>>) {
        if self.state.count() == self.ticket_count {
            let must_return = self.state.position != self.home;
            let bought_count = self.state.bought.len() + if must_return { 1 } else { 0 };
            if bought_count < self.best_bought_count {
                eprintln!("\tNew best: {}", bought_count);
                self.best_bought = self.state.bought.clone();
                if must_return {
                    self.best_bought.push((self.state.position, self.home));
                }
                self.best_bought_count = bought_count;
            }
            return;
        }

        let key = (self.state.count() as u8, self.state.position);
        if self.seen.get(&key).copied().unwrap_or(u8::MAX) < self.state.bought.len() as u8 {
            return;
        }
        self.seen.insert(key, self.state.bought.len() as u8);

        if self.state.bought.len() >= self.best_bought_count {
            return;
        }

        let mut used_tickets: HashSet<(u8, u8)> = HashSet::default();
        for &(i, ticket) in &candidate_cache[&self.state.position] {
            if self.state.has(i) {
                continue;
            }
            if used_tickets.contains(&ticket) {
                continue; // dedup
            }
            used_tickets.insert(ticket);

            // Try to use ticket
            self.state.push(i);
            let prev_position = self.state.position;

            if self.state.position != ticket.0 && self.state.position != ticket.1 {
                for &destination in &[ticket.0, ticket.1] {
                    self.state.bought.push((self.state.position, destination));
                    self.state.position = destination;
                    self.search(candidate_cache);
                    self.state.bought.pop();
                }
            } else {
                self.state.position = if ticket.0 != self.state.position {
                    ticket.0
                } else {
                    ticket.1
                };
                self.search(candidate_cache);
            }

            self.state.position = prev_position;
            self.state.pop(i);
        }
    }
}

#[inline]
pub fn solve() -> impl Display {
    let data = include_str!("input.txt");
    let mut lines = data.lines();
    let num_cases: usize = lines.next().unwrap().parse().unwrap();

    cases((0..num_cases).map(|_| {
        let ticket_count: usize = lines.next().unwrap().parse().unwrap();
        let mut result = String::new();

        let mut tickets = Vec::new();
        for _ in 0..ticket_count {
            let line = lines.next().unwrap();
            let ticket = line.split_once(' ').unwrap();
            tickets.push(ticket);
        }

        eprintln!("Start: {:?}", tickets);

        let start = std::time::Instant::now();
        let (mut searcher, candidate_cache) = Searcher::new(&tickets);
        searcher.search(&candidate_cache);
        eprintln!("Time: {:?}", start.elapsed());

        writeln!(result, "{}", searcher.best_bought_count).unwrap();
        for &(src, dst) in &searcher.best_bought {
            writeln!(result, "{} {}", searcher.nodes[src as usize], searcher.nodes[dst as usize]).unwrap();
        }

        result
    }))
}
