use std::fmt::Display;

use rayon::prelude::*;

use common::*;

#[inline]
pub fn solve() -> impl Display {
    cases(
        include_str!("input.txt")
            .par_lines()
            .map(|line| {
                let n: u8 = line.parse().unwrap();

                let (best_d, perm) = (1..n)
                    .into_par_iter()
                    .find_map_last(|best_d| {
                        let perm = (1..=n)
                            .find_map(|m| {
                                let mut perm = vec![m.to_string()];
                                let mut used: u128 = 1 << m;
                                let mut last = m;

                                for _ in 1..n {
                                    let next = (1..=n).find(|&x| {
                                        used & (1 << x) == 0 && last.abs_diff(x) >= best_d
                                    })?;
                                    perm.push(next.to_string());
                                    used |= 1 << next;
                                    last = next;
                                }

                                Some(perm)
                            })?
                            .join(" ");

                        Some((best_d, perm))
                    })
                    .unwrap();

                format!("{best_d}\n{perm}")
            })
            .collect::<Vec<_>>(),
    )
}
