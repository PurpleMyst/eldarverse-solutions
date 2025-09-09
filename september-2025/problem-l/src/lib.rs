use std::{fmt::Display, num::NonZeroU8};

use common::*;

/*
 * Built painstakingly by hand. Basically a DFS, made faster by various micro-opts combined with
 * trying to start from floor(log2(n)).
 *
 * Takes a few minutes, so go get a coffee or something.
 */

const GEOLYMP: &str = "GEOLYMP";
const FAKE_GEOLYMP: &str = "\x00\x01\x02\x03\x04\x05\x06";

const GEOLYMP_LEN: usize = GEOLYMP.len();

type Count = u32;
type Prefixes = [Count; GEOLYMP_LEN];

fn update_prefixes_generic(s: &[u8], mut prefixes: Prefixes) -> Prefixes {
    for &b in s {
        let idx = b as usize;
        unsafe {
            std::hint::assert_unchecked(idx < GEOLYMP_LEN);
        }
        prefixes[idx] +=
            (idx == 0) as u32 * 1 + (idx != 0) as u32 * prefixes[idx.saturating_sub(1)];
    }
    prefixes
}

/// Specialized version of `update_prefixes_generic` that unrolls the loop and
/// removes bounds checks.
fn add_suffix(i: usize, mut prefixes: Prefixes) -> Prefixes {
    match i {
        0 => {
            prefixes[0] += 1;
            prefixes[1] += prefixes[0];
            prefixes[2] += prefixes[1];
            prefixes[3] += prefixes[2];
            prefixes[4] += prefixes[3];
            prefixes[5] += prefixes[4];
            prefixes[6] += prefixes[5];
        }

        1 => {
            prefixes[1] += prefixes[0];
            prefixes[2] += prefixes[1];
            prefixes[3] += prefixes[2];
            prefixes[4] += prefixes[3];
            prefixes[5] += prefixes[4];
            prefixes[6] += prefixes[5];
        }

        2 => {
            prefixes[2] += prefixes[1];
            prefixes[3] += prefixes[2];
            prefixes[4] += prefixes[3];
            prefixes[5] += prefixes[4];
            prefixes[6] += prefixes[5];
        }

        3 => {
            prefixes[3] += prefixes[2];
            prefixes[4] += prefixes[3];
            prefixes[5] += prefixes[4];
            prefixes[6] += prefixes[5];
        }

        4 => {
            prefixes[4] += prefixes[3];
            prefixes[5] += prefixes[4];
            prefixes[6] += prefixes[5];
        }

        5 => {
            prefixes[5] += prefixes[4];
            prefixes[6] += prefixes[5];
        }

        6 => {
            prefixes[6] += prefixes[5];
        }

        _ => unsafe { std::hint::unreachable_unchecked() },
    }
    prefixes
}

fn count_geolymp(s: &[u8]) -> u32 {
    let prefixes = update_prefixes_generic(s, [0; GEOLYMP_LEN]);
    prefixes[GEOLYMP_LEN - 1]
}

#[derive(Debug)]
struct Searcher<'arena> {
    target: u32,
    arena: &'arena bumpalo::Bump,
}

#[derive(Debug, Clone, Copy)]
struct SearchStep<'arena> {
    previous: Option<&'arena SearchStep<'arena>>,
    suffix: NonZeroU8,
    prefixes: Prefixes,
    length: usize,
}

fn rgb_ansi(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m")
}

impl SearchStep<'_> {
    fn to_string_with_charset(&self, charset: &str) -> String {
        let mut s = String::with_capacity(self.length);
        let mut current = Some(self);
        while let Some(node) = current {
            let i = (node.suffix.get() - 1) as usize;
            let suffix_bytes = &charset.as_bytes()[i..];
            s.extend(suffix_bytes.iter().map(|&b| b as char).rev());
            current = node.previous;
        }
        s.chars().rev().collect()
    }

    fn to_pretty_string(&self) -> String {
        let mut s = Vec::new();
        let mut current = Some(self);
        while let Some(node) = current {
            let i = (node.suffix.get() - 1) as usize;
            let suffix_bytes = &GEOLYMP.as_bytes()[i..];
            let color = match i {
                0 => rgb_ansi(255, 0, 0),     // G - Red
                1 => rgb_ansi(255, 165, 0),   // E - Orange
                2 => rgb_ansi(255, 255, 0),   // O - Yellow
                3 => rgb_ansi(0, 128, 0),     // L - Green
                4 => rgb_ansi(0, 0, 255),     // Y - Blue
                5 => rgb_ansi(75, 0, 130),    // M - Indigo
                6 => rgb_ansi(238, 130, 238), // P - Violet
                7 => String::new(),           // Start node, no color
                _ => unreachable!(),
            };
            s.push(
                suffix_bytes
                    .iter()
                    .map(|&b| format!("{color}{}\x1b[0m", b as char))
                    .collect::<String>(),
            );

            current = node.previous;
        }
        s.into_iter().rev().collect::<String>() + "\x1b[0m"
    }
}

impl<'arena> Searcher<'arena> {
    fn new(target: u32, arena: &'arena bumpalo::Bump) -> Self {
        Self { target, arena }
    }

    fn search(
        &mut self,
        current: Option<&'arena SearchStep<'arena>>,
    ) -> Option<&'arena SearchStep<'arena>> {
        let start_prefixes = [0; GEOLYMP_LEN];
        let start_length = 0;

        for i in 0..GEOLYMP_LEN {
            let node = self.arena.alloc(SearchStep {
                previous: current,
                suffix: NonZeroU8::new(i as u8 + 1).unwrap(),
                prefixes: add_suffix(i, current.map_or(start_prefixes, |n| n.prefixes)),
                length: current.map_or(start_length, |n| n.length) + (GEOLYMP_LEN - i),
            });
            if node.prefixes[GEOLYMP_LEN - 1] <= self.target && node.length <= 1000 {
                if node.prefixes[GEOLYMP_LEN - 1] == self.target {
                    return Some(&*node);
                } else if let Some(new_found) = self.search(Some(node)) {
                    return Some(new_found);
                }
            }
        }
        None
    }

    fn search_with_prefix(&mut self, prefix: &str) -> Option<&'arena SearchStep<'arena>> {
        debug_assert!(
            prefix.bytes().all(|b| b <= GEOLYMP_LEN as u8),
            "Prefix contains invalid characters"
        );
        let prefix_bytes = prefix.as_bytes();
        let prefixes = update_prefixes_generic(prefix_bytes, [0; GEOLYMP_LEN]);
        let length = prefix_bytes.len();
        let start_node = self.arena.alloc(SearchStep {
            previous: None,
            suffix: NonZeroU8::new(GEOLYMP_LEN as u8 + 1).unwrap(),
            prefixes,
            length,
        });
        if prefixes[GEOLYMP_LEN - 1] == self.target {
            return Some(start_node);
        }
        self.search(Some(start_node))
    }
}

#[inline]
pub fn solve() -> impl Display {
    let input = include_str!("input.txt");
    let total = input.lines().skip(1).count();
    cases(input.lines().skip(1).enumerate().map(|(i, line)| {
        let n = line.trim().parse::<u32>().unwrap();
        let arena = bumpalo::Bump::new();
        let mut searcher = Searcher::new(n, &arena);

        let mut m = n;
        while m != 0 {
            let prefix = build_prefix(m);

            let Some(node) = searcher.search_with_prefix(&prefix) else {
                m >>= 1;
                continue;
            };

            debug_assert_eq!(
                count_geolymp(
                    format!("{prefix}{}", node.to_string_with_charset(FAKE_GEOLYMP)).as_bytes()
                ),
                n
            );
            let ps = node.to_pretty_string();
            let l = node.to_string_with_charset(GEOLYMP).len();
            let disp_prefix = prefix
                .bytes()
                .map(|b| GEOLYMP.as_bytes()[b as usize] as char)
                .collect::<String>();
            eprintln!(
                "[{:2}/{:2}] {n:11} => {disp_prefix}{}{pad} ({:4})",
                i + 1,
                total,
                ps,
                l,
                pad = " ".repeat((64usize).saturating_sub(l + prefix.len()))
            );

            return format!("{disp_prefix}{}", node.to_string_with_charset(GEOLYMP));
        }
        panic!();
    }))
}

fn build_prefix(n: u32) -> String {
    let msb = n.ilog2() as usize;
    let base = msb / 7;
    let mut prefix = String::new();
    for c in FAKE_GEOLYMP.chars() {
        for _ in 0..1 << base {
            prefix.push(c);
        }
    }
    let kitty = update_prefixes_generic(prefix.as_bytes(), [0; GEOLYMP_LEN]);
    let foo = kitty[GEOLYMP_LEN - 2];
    let mut have = kitty[GEOLYMP_LEN - 1];
    let mut need = (1 << msb) - have;
    if have == 0 {
        prefix.push_str(FAKE_GEOLYMP);
        need -= 1;
        have = 1;
    }
    let per_p = if have == 1 { 1 } else { foo };
    prefix.push_str(
        FAKE_GEOLYMP[GEOLYMP_LEN - 1..]
            .repeat(need as usize / per_p as usize)
            .as_str(),
    );

    debug_assert_eq!(count_geolymp(prefix.as_bytes()), 1 << msb);
    prefix
}
