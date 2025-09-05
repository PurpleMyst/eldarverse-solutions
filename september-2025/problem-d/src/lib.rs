use std::fmt::Display;

use common::*;

use num_modular::{ModularInteger, MontgomeryInt};

const M: u64 = 1_000_000_009;

#[inline]
pub fn solve() -> impl Display {
    cases(include_str!("input.txt").lines().map(|line| {
        let n = line.parse::<u64>().unwrap();

        let big_side = MontgomeryInt::new(2, &M).pow(&n);

        // Final formula derived from the problem analysis; I just started manually unrolling loops
        // and came up with this formula after a while.
        // area.square() + area + big_side * 2 * area
        //     - (area + big_side) * gauss_sum(big_side) * 2
        //     + gauss_sum(big_side).square();
        // Which then simplifies to the square of the gauss sum of big_side.
        let result = (big_side * (big_side + 1) / big_side.convert(2)).pow(&2);

        result.residue()
    }))
}
