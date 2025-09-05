use std::fmt::Display;

use itertools::Itertools;

use common::*;
use rayon::prelude::*;

#[inline]
pub fn solve() -> impl Display {
    let ks: Vec<[_; 10]> = (2..=6)
        .into_par_iter()
        .map(|n| {
            let game_map: Vec<_> = (0..n)
                .flat_map(|i| (0..n).map(move |j| (i, j)))
                .filter(|(i, j)| i != j)
                .map(|(i, j)| (i as usize, j as usize))
                .collect();

            (0..1 << (n * (n - 1)))
                .into_par_iter()
                .map(|mut combo: u32| {
                    let mut scores = [0; 6];
                    for &(i, j) in &game_map {
                        scores[i as usize] += combo & 1;
                        scores[j as usize] += 1 - (combo & 1);
                        combo >>= 1;
                    }
                    let (min, max) = scores.into_iter().take(n).minmax().into_option().unwrap();
                    max - min
                })
                .fold(
                    || [0u32; 10],
                    |mut acc, delta| {
                        acc.iter_mut()
                            .take(delta as usize)
                            .for_each(|count| *count += 1);

                        acc
                    },
                )
                .reduce(
                    || [0; 10],
                    |mut a, b| {
                        a.iter_mut().zip(b).for_each(|(cell, n)| *cell += n);
                        a
                    },
                )
        })
        .collect();

    cases(
        include_str!("input.txt")
            .lines()
            .map(|line| {
                let (n, k) = line.split_once(' ').unwrap();
                let n: usize = n.parse().unwrap();
                let k: usize = k.parse().unwrap();

                ks[n - 2][k]
            })
            .collect::<Vec<_>>(),
    )
}
