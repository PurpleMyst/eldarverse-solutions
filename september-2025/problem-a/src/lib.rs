use std::fmt::Display;

pub fn solve() -> impl Display {
    include_str!("input.txt").lines().skip(1)
        .enumerate()
        .map(|(i, line)| {
            let mut mask = 0u32;
            for c in line.bytes() {
                mask |= 1 << (c.to_ascii_lowercase() - b'a')
            }

            format!("Case #{}: {}", i + 1, 100 - 5 * mask.count_ones())
        })
        .collect::<Vec<String>>()
        .join("\n")
}
