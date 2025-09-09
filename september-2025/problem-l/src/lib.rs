use std::fmt::Display;

use indicatif::*;
use rayon::prelude::*;

use common::*;

/// Actual GEOLYMP string.
const GEOLYMP: &str = "GEOLYMP";

/// Fake GEOLYMP string which allows easier computation.
const FAKE_GEOLYMP: &str = "\x00\x01\x02\x03\x04\x05\x06";

const GEOLYMP_LEN: usize = GEOLYMP.len();

const MAX_LEN: usize = 1000;

type Count = u32;
type PrefixArray = [Count; GEOLYMP_LEN];

/// Compute the prefix array for a given byte slice.
/// The `i`-th element of the array represents the count of subsequences
/// that can be formed using the first `i+1` characters of the GEOLYMP string.
fn compute_prefix_array(s: &[u8]) -> PrefixArray {
    let mut prefix_arr = [0; GEOLYMP_LEN];
    for &b in s {
        let idx = b as usize;
        unsafe {
            std::hint::assert_unchecked(idx < GEOLYMP_LEN);
        }
        prefix_arr[idx] +=
            (idx == 0) as u32 + (idx != 0) as u32 * prefix_arr[idx.saturating_sub(1)];
    }
    prefix_arr
}

/// Push a suffix starting with the `i`-th character of GEOLYMP to the current prefixes.
fn push_suffix(i: usize, prefixes: &mut PrefixArray) {
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
}

/// Pop a suffix starting with the `i`-th character of GEOLYMP from the current prefixes.
fn pop_suffix(i: usize, prefixes: &mut PrefixArray) {
    match i {
        0 => {
            prefixes[6] -= prefixes[5];
            prefixes[5] -= prefixes[4];
            prefixes[4] -= prefixes[3];
            prefixes[3] -= prefixes[2];
            prefixes[2] -= prefixes[1];
            prefixes[1] -= prefixes[0];
            prefixes[0] -= 1;
        }

        1 => {
            prefixes[6] -= prefixes[5];
            prefixes[5] -= prefixes[4];
            prefixes[4] -= prefixes[3];
            prefixes[3] -= prefixes[2];
            prefixes[2] -= prefixes[1];
            prefixes[1] -= prefixes[0];
        }

        2 => {
            prefixes[6] -= prefixes[5];
            prefixes[5] -= prefixes[4];
            prefixes[4] -= prefixes[3];
            prefixes[3] -= prefixes[2];
            prefixes[2] -= prefixes[1];
        }

        3 => {
            prefixes[6] -= prefixes[5];
            prefixes[5] -= prefixes[4];
            prefixes[4] -= prefixes[3];
            prefixes[3] -= prefixes[2];
        }

        4 => {
            prefixes[6] -= prefixes[5];
            prefixes[5] -= prefixes[4];
            prefixes[4] -= prefixes[3];
        }

        5 => {
            prefixes[6] -= prefixes[5];
            prefixes[5] -= prefixes[4];
        }

        6 => {
            prefixes[6] -= prefixes[5];
        }

        _ => unsafe { std::hint::unreachable_unchecked() },
    }
}

#[derive(Debug)]
struct Searcher {
    /// Target count of subsequences to match.
    target: u32,

    /// Current suffixes being considered (as indices into GEOLYMP).
    suffixes: Vec<u8>,

    /// Current prefix array representing counts of subsequences.
    prefixes: PrefixArray,

    /// Current length of the constructed string.
    length: usize,
}

impl Searcher {
    /// Create a new Searcher with the given target.
    fn new(target: u32) -> Self {
        Self {
            target,
            suffixes: Vec::new(),
            prefixes: [0; GEOLYMP_LEN],
            length: 0,
        }
    }

    /// Recursively search for a valid sequence of suffixes that matches the target.
    /// Returns true if a valid sequence is found, false otherwise.
    fn search(&mut self) -> bool {
        for i in 0..GEOLYMP_LEN {
            self.length += GEOLYMP_LEN - i;
            push_suffix(i, &mut self.prefixes);
            self.suffixes.push(i as _);
            if self.prefixes[GEOLYMP_LEN - 1] <= self.target
                && self.length <= MAX_LEN
                && (self.prefixes[GEOLYMP_LEN - 1] == self.target || self.search())
            {
                return true;
            }
            self.suffixes.pop();
            pop_suffix(i, &mut self.prefixes);
            self.length -= GEOLYMP_LEN - i;
        }
        false
    }

    /// Initialize the searcher with a base string and start the search.
    fn search_with_base(&mut self, base: &str) -> bool {
        let base_bytes = base.as_bytes();
        let prefixes = compute_prefix_array(base_bytes);
        let length = base_bytes.len();
        self.prefixes = prefixes;
        self.length = length;
        if self.prefixes[GEOLYMP_LEN - 1] == self.target {
            return true;
        }
        self.search()
    }

    /// Convert the found suffixes into a string using the provided charset.
    fn to_string_with_charset(&self, charset: &str) -> String {
        let mut s = Vec::with_capacity(self.length);
        for &i in &self.suffixes {
            let suffix_bytes = &charset.as_bytes()[i as usize..];
            s.extend_from_slice(suffix_bytes);
        }
        unsafe { String::from_utf8_unchecked(s) }
    }
}

#[inline]
pub fn solve() -> impl Display {
    let input = include_str!("input.txt");
    let total = input.lines().count();
    cases(
        input
            .par_lines()
            .progress_count(total as u64)
            .map(|line| {
                let n = line.trim().parse::<u32>().unwrap();

                let mut m = n;
                while m != 0 {
                    // Start from a base string that corresponds to the highest power of two <= m
                    let base = build_base(m);
                    let mut searcher = Searcher::new(n);
                    if !searcher.search_with_base(&base) {
                        // If not found, reduce m to the next lower power of two and try again; no
                        // idea why this works.
                        m >>= 1;
                        continue;
                    };

                    let readable_base = base
                        .bytes()
                        .map(|b| GEOLYMP.as_bytes()[b as usize] as char)
                        .collect::<String>();
                    return format!(
                        "{readable_base}{}",
                        searcher.to_string_with_charset(GEOLYMP)
                    );
                }
                unreachable!();
            })
            .collect::<Vec<_>>(),
    )
}

/// Build a base string whose subsequence count is the largest power of two less than or equal to
/// `n`.
fn build_base(n: u32) -> String {
    let msb = n.ilog2() as usize;
    let base = msb / 7;
    let mut prefix = String::new();
    for c in FAKE_GEOLYMP.chars() {
        for _ in 0..1 << base {
            prefix.push(c);
        }
    }
    let prefixes_sofar = compute_prefix_array(prefix.as_bytes());
    let theoretical_per_p = prefixes_sofar[GEOLYMP_LEN - 2];
    let mut have = prefixes_sofar[GEOLYMP_LEN - 1];
    let mut need = (1 << msb) - have;
    if have == 0 {
        prefix.push_str(FAKE_GEOLYMP);
        need -= 1;
        have = 1;
    }
    let per_p = if have == 1 { 1 } else { theoretical_per_p };
    prefix.push_str(
        FAKE_GEOLYMP[GEOLYMP_LEN - 1..]
            .repeat(need as usize / per_p as usize)
            .as_str(),
    );

    prefix
}
