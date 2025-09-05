use std::fmt::Display;

use itertools::Itertools;

use common::*;
use rayon::prelude::*;

#[inline]
pub fn solve() -> impl Display {
    let ks: Vec<[_; 10]> = (2..=6).into_par_iter().map(|n| {
        (0..1 << (n * (n -1))).into_par_iter().map(|mut combo| {

            let mut scores = vec![0u8; n as usize];
            for i in 0..n {
                for j in 0..n {
                    if i != j {
                        if combo & 1 != 0 {
                            scores[i as usize] += 1;
                        } else {
                            scores[j as usize] += 1;
                        }
                        combo >>= 1;
                    }
                }
            }
            let (min, max) = scores.into_iter().minmax().into_option().unwrap();
            max - min
        }).fold(|| [0; 10], |mut acc, delta| {
            acc.iter_mut()
                .enumerate()
                .for_each(|(k, item)| if delta as usize > k { *item += 1 });

            acc
        })
        .reduce(|| [0; 10], |mut a, b| {
            a.iter_mut().zip(b).for_each(|(cell, n)| *cell += n);
            a
        })
    }).collect();

    cases(include_str!("input.txt").lines().map(|line| {
        let (n, k) = line.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();
        let k: usize = k.parse().unwrap();

        ks[n - 2][k]
    }).collect::<Vec<_>>())
}
