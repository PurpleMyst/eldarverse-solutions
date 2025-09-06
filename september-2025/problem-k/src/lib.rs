use std::fmt::Display;
use std::io::Write;
use std::iter::zip;

use rayon::prelude::*;

use common::*;

struct Searcher<'a> {
    /// Matrix where the rows are sockets and the columns are organizers;
    /// matrix[i][j] is the distance from socket i to organizer j.
    matrix: &'a [Vec<usize>],

    /// Available cable lengths.
    cables: &'a [usize],

    best_assigned: usize,

    best_organizers: u16,
    best_sockets: u16,
}

impl<'a> Searcher<'a> {
    fn new(matrix: &'a [Vec<usize>], cables: &'a [usize]) -> Self {
        Self {
            matrix,
            cables,

            best_assigned: 0,
            best_sockets: 0,
            best_organizers: 0,
        }
    }

    fn backtrack(&mut self, i: usize, used_sockets: u16, used_organizers: u16, used_cables: u16) {
        if i == self.matrix.len() {
            let assigned = used_organizers.count_ones() as usize;
            if assigned > self.best_assigned {
                self.best_assigned = assigned;
                self.best_sockets = used_sockets;
                self.best_organizers = used_organizers;
            }
            return;
        }

        let remaining = self.matrix.len() - i;
        if (used_organizers.count_ones() as usize + remaining) <= self.best_assigned {
            return;
        }

        for (j, &d) in self.matrix[i].iter().enumerate() {
            if used_organizers & (1 << j) != 0 {
                continue;
            }
            for (k, &l) in self.cables.iter().enumerate() {
                if used_cables & (1 << k) != 0 {
                    continue;
                }
                if l * l < d {
                    continue;
                }
                self.backtrack(
                    i + 1,
                    used_sockets | (1 << i),
                    used_organizers | (1 << j),
                    used_cables | (1 << k),
                );
            }
        }

        self.backtrack(i + 1, used_sockets, used_organizers, used_cables);
    }

    fn start(&mut self) {
        self.backtrack(0, 0, 0, 0);
    }
}

#[inline]
pub fn solve() -> impl Display {
    let mut lines = include_str!("input.txt").lines();
    let t = lines.next().unwrap().parse::<usize>().unwrap();

    cases(
        (0..t)
            .map(|_| {
                let (organizer_count, socket_count) = {
                    let mut it = lines.next().unwrap().split_whitespace();
                    (
                        it.next().unwrap().parse::<usize>().unwrap(),
                        it.next().unwrap().parse::<usize>().unwrap(),
                    )
                };
                let cable_lengths = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                let organizer_positions = lines
                    .by_ref()
                    .take(organizer_count)
                    .map(|line| {
                        let mut it = line.split_whitespace();
                        (
                            it.next().unwrap().parse::<usize>().unwrap(),
                            it.next().unwrap().parse::<usize>().unwrap(),
                        )
                    })
                    .collect::<Vec<_>>();
                let socket_positions = lines
                    .by_ref()
                    .take(socket_count)
                    .map(|line| {
                        let mut it = line.split_whitespace();
                        (
                            it.next().unwrap().parse::<usize>().unwrap(),
                            it.next().unwrap().parse::<usize>().unwrap(),
                        )
                    })
                    .collect::<Vec<_>>();

                let mut matrix = vec![vec![usize::MAX; organizer_count]; socket_count];

                for (&(u, v), row) in zip(socket_positions.iter(), matrix.iter_mut()) {
                    for (&(x, y), cell) in zip(organizer_positions.iter(), row.iter_mut()) {
                        let d = x.abs_diff(u).pow(2) + y.abs_diff(v).pow(2);
                        *cell = d;
                    }
                }

                (
                    matrix,
                    cable_lengths,
                    organizer_count,
                    organizer_positions.clone(),
                    socket_positions.clone(),
                )
            })
            .collect::<Vec<_>>()
            .into_par_iter()
            .enumerate()
            .map(
                |(
                    a,
                    (matrix, cable_lengths, organizer_count, organizer_positions, socket_positions),
                )| {
                    let mut searcher = Searcher::new(&matrix, &cable_lengths);
                    searcher.start();

                    {
                        let mut stderr = std::io::stderr().lock();
                        writeln!(stderr, "\x1b[31;1mCase #{a}\x1b[0m").unwrap();

                        writeln!(
                            stderr,
                            "\x1b[1mOrganizers ({}):\x1b[0m",
                            organizer_positions.len()
                        )
                        .unwrap();
                        for &(x, y) in &organizer_positions {
                            write!(stderr, "({x},{y}) ").unwrap();
                        }
                        writeln!(stderr).unwrap();

                        write!(
                            stderr,
                            "\x1b[1mSockets ({}):\x1b[0m ",
                            socket_positions.len()
                        )
                        .unwrap();
                        for &(u, v) in &socket_positions {
                            write!(stderr, "({u},{v}) ").unwrap();
                        }
                        writeln!(stderr).unwrap();

                        write!(stderr, "\x1b[1mDistance matrix:\x1b[0m\n").unwrap();
                        for row in &matrix {
                            for cell in row {
                                write!(stderr, "{:6.2} ", (*cell as f64).sqrt()).unwrap();
                            }
                            writeln!(stderr).unwrap();
                        }

                        writeln!(
                            stderr,
                            "\x1b[1mCable lengths ({}):\x1b[0m",
                            cable_lengths.len()
                        )
                        .unwrap();
                        for length in &cable_lengths {
                            write!(stderr, "{:4} ", length).unwrap();
                        }
                        writeln!(stderr).unwrap();

                        writeln!(stderr, "\x1b[1mBest assignment:\x1b[0m").unwrap();
                        writeln!(stderr, "Assigned = {}", searcher.best_assigned).unwrap();
                        writeln!(stderr, "Used organizers:").unwrap();
                        for (i, &(x, y)) in organizer_positions.iter().enumerate() {
                            if searcher.best_organizers & (1 << i) != 0 {
                                write!(stderr, "({x},{y}) ").unwrap();
                            }
                        }
                        writeln!(stderr, "\nUsed sockets:").unwrap();
                        for (j, &(u, v)) in socket_positions.iter().enumerate() {
                            if searcher.best_sockets & (1 << j) != 0 {
                                write!(stderr, "({u},{v}) ").unwrap();
                            }
                        }
                        writeln!(stderr).unwrap();
                    }

                    organizer_count - searcher.best_assigned
                },
            )
            .collect::<Vec<_>>(),
    )
}
