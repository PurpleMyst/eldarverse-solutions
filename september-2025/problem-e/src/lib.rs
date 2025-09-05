use std::fmt::Display;

use common::*;

#[inline]
pub fn solve() -> impl Display {
    cases(include_str!("input.txt").lines().map(|line| {
        let mut n = line.parse::<u64>().unwrap();

        if n <= 3 {
            return n;
        }

        let mut result = 0;

        let mut i = 1;
        let mut m = 3;
        while n != 0 {
            n -= m;
            result += i * m;
            i += 1;
            m += 2;
            m = m.min(n);
        }

        result
    }))
}
