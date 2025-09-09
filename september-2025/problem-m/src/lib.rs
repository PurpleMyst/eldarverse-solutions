use std::fmt::Display;

use itertools::Itertools;

use common::*;

struct Searcher {
    answer: f64,
}

impl Searcher {
    fn new() -> Self {
        Self {
            answer: 0.,
        }
    }

    fn search(&mut self, a: f64, b: f64, k: u8) {
        let area = a * b / 2.0;
        if area < self.answer {
            return;
        }
        if k == 0 {
            self.answer = self.answer.max(area);
            return;
        }

        let c = f64::hypot(a, b);

        let h = (a * b) / c;

        let l1 = (a * a - h * h).sqrt();
        self.search(h, l1, k - 1);

        let l2 = c - l1;
        self.search(h, l2, k - 1);
    }
}

#[inline]
pub fn solve() -> impl Display {
    cases(include_str!("input.txt").lines().skip(1).map(|line| {
        let (a, b, k) = line
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect_tuple()
            .unwrap();
        let mut searcher = Searcher::new();
        searcher.search(a as _, b as _, k);
        format!("{:.6}", searcher.answer)
    }))
}
