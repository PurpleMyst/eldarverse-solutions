use std::{io::Write, fmt::Display, num::NonZeroU8};

#[allow(unused_imports)]
use common::*;

const GEOLYMP: &str = "GEOLYMP";
const FAKE_GEOLYMP: &str = "0123456";

type Count = u32;
type Prefixes = [Count; GEOLYMP.len()];

use bumpalo::Bump;

fn update_prefixes(s: &[u8], mut prefixes: Prefixes) -> Prefixes  {
    for &b in s {
        let idx = (b - b'0') as usize;
        unsafe { std::hint::assert_unchecked(idx < GEOLYMP.len()); }
        prefixes[idx] += (idx == 0) as u32 * 1 + (idx != 0) as u32 * prefixes[idx.saturating_sub(1)];
    }
    prefixes
}

fn count_geolymp(s: &[u8]) -> u32 {
    let prefixes = update_prefixes(s, [0; GEOLYMP.len()]);
    prefixes[GEOLYMP.len() - 1]
}

#[derive(Debug)]
struct Searcher<'arena> {
    target: u32,
    arena: &'arena Bump,
}

#[derive(Debug, Clone, Copy)]
struct Node<'arena> {
    previous: Option<&'arena Node<'arena>>,
    suffix: NonZeroU8,
    prefixes: Prefixes,
    length: usize,
}

impl From<&Node<'_>> for String {
    fn from(this: &Node<'_>) -> String {
        let mut s = String::with_capacity(this.length);
        let mut current = Some(this);
        while let Some(node) = current {
            let suffix_str = &FAKE_GEOLYMP.as_bytes()[(node.suffix.get() - 1) as usize..];
            s.extend(suffix_str.iter().map(|&b| b as char).rev());
            current = node.previous;
        }
        s.chars().rev().collect()
    }
}

impl Display for Node<'_> {
    // same as above but with real GEOLYMP
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(self.length);
        let mut current = Some(self);
        while let Some(node) = current {
            let suffix_str = &GEOLYMP.as_bytes()[(node.suffix.get() - 1) as usize..];
            s.extend(suffix_str.iter().map(|&b| b as char).rev());
            current = node.previous;
        }
        let s: String = s.chars().rev().collect();
        write!(f, "{s}")
    }
}

impl<'arena> Searcher<'arena> {
    fn new(target: u32, arena: &'arena Bump) -> Self {
        Self { target, arena }
    }

    fn search(&self, current: Option<&'arena Node<'arena>>) -> Option<&'arena Node<'arena>> {
        for i in 0..GEOLYMP.len() {
            let suffix_str = &FAKE_GEOLYMP.as_bytes()[i..];
            let node = self.arena.alloc(Node {
                previous: current,
                suffix: NonZeroU8::new(i as u8 + 1).unwrap(),
                prefixes: update_prefixes(suffix_str, current.map_or([0; GEOLYMP.len()], |n| n.prefixes)),
                length: current.map_or(0, |n| n.length) + suffix_str.len(),
            });
            if node.prefixes[GEOLYMP.len() - 1] <= self.target && node.length <= 1000 {
                if node.prefixes[GEOLYMP.len() - 1] == self.target {
                    return Some(node);
                } else if let Some(found) = self.search(Some(node)) {
                    return Some(found);
                }
            }
        }
        None
    }
}

fn count_full_geolymp_at_start_of_string(s: &str) -> u32 {
    if s.starts_with(FAKE_GEOLYMP) {
        1 + count_full_geolymp_at_start_of_string(&s[FAKE_GEOLYMP.len()..])
    } else {
        0
    }
}


#[inline]
pub fn solve() -> impl Display {
    // let total = include_str!("input.txt")
    //     .lines().skip(1).count();
    // for (i, line) in include_str!("input.txt") 
    //     .lines().skip(1).enumerate() {
    //         let n = line.trim().parse::<u32>().unwrap();
    //         let arena = Bump::new();
    //         let searcher = Searcher::new(n, &arena);
    //         eprintln!("Searching for {n} ({}/{})", i + 1, total);
    //         let node = searcher.search(None).unwrap();
    //         assert_eq!(count_geolymp(&String::from(&*node).into_bytes()), n);
    //         println!("{node}");
    //     }

    let mut map = std::collections::BTreeMap::new();
    for target in 0..1<<16 {
        let arena = Bump::new();
        let searcher = Searcher::new(target, &arena);
        let node = searcher.search(None).expect("Should find a solution");
        let full = count_full_geolymp_at_start_of_string(&String::from(&*node));
        let max_with_full = map.entry(full).or_default();
        let new_value = std::cmp::max(*max_with_full, target);
        *max_with_full = new_value;
    }
    for (ful, max) in map.iter() {
        println!("{} -> {}", ful, max);
    }

    "TODO"
}
