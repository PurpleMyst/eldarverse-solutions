use std::fmt::Display;

use common::cases;

pub fn solve() -> impl Display {
    cases(include_str!("input.txt").lines().skip(1)
        .map(|line| {
            let mut mask = 0u32;
            for c in line.bytes() {
                mask |= 1 << (c.to_ascii_lowercase() - b'a');
            }

            100 - 5 * mask.count_ones()
        }))
}
