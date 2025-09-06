use std::{
    cmp::Reverse,
    fmt::{Display, Write},
};

use rayon::prelude::*;

use common::*;

type Point = (usize, usize);
type Row = u64;

fn euclidian_distance_sq((x1, y1): Point, (x2, y2): Point) -> usize {
    x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2)
}

#[derive(Debug)]
struct Problem {
    width: usize,
    buildable: Vec<Row>,
    enemy: Point,
}

impl Problem {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            buildable: vec![0; height],
            enemy: (usize::MAX, usize::MAX),
        }
    }

    fn can_build(&self, (x, y): Point) -> bool {
        (self.buildable[y] & (1 << x)) != 0
    }

    fn set_buildable(&mut self, (x, y): Point) {
        self.buildable[y] |= 1 << x;
    }

    fn set_enemy(&mut self, (x, y): Point) {
        self.enemy = (x, y);
    }

    fn buildable_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.buildable
            .iter()
            .enumerate()
            .flat_map(move |(y, &row)| {
                (0..self.width)
                    .filter(move |&x| (row & (1 << x)) != 0)
                    .map(move |x| (x, y))
            })
    }

    fn fuel_cost(&self, point: Point) -> usize {
        euclidian_distance_sq(point, self.enemy)
    }
}

#[inline]
pub fn solve() -> impl Display {
    let mut lines = include_str!("input.txt").lines();

    let test_cases: usize = lines.next().unwrap().parse().unwrap();
    cases(
        (0..test_cases )
            .map(|_| {
                let (height, width, buildings): (usize, usize, usize) = {
                    let mut parts = lines.next().unwrap().split_whitespace();
                    (
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().parse().unwrap(),
                    )
                };

                let mut problem = Problem::new(width, height);
                for (y, line) in lines.by_ref().take(height).enumerate() {
                    for (x, c) in line.chars().enumerate() {
                        match c {
                            '.' => problem.set_buildable((x, y)),
                            'M' => problem.set_enemy((x, y)),
                            _ => {}
                        }
                    }
                }

                (problem, buildings)
            })
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|(problem, buildings)| {
                let mut candidate_points: Vec<Point> = problem.buildable_points().collect();
                candidate_points.sort_by_key(|&point| Reverse(problem.fuel_cost(point)));
                candidate_points.truncate(buildings);

                let candidate_costs: Vec<usize> = candidate_points.iter()
                        .map(|&p| problem.fuel_cost(p))
                        .collect();

                let total = candidate_costs.iter().sum::<usize>();
                let target = total / 2;
                
                // subset sum dp
                let mut dp = vec![vec![false; target + 1]; buildings + 1];
                let mut parent = vec![vec![None; target + 1]; buildings + 1];
                dp[0][0] = true;

                for i in 1..=buildings {
                    let a = candidate_costs[i-1];
                    for s in 0..=target {
                        if dp[i - 1][s] {
                            dp[i][s] = true;
                            parent[i][s] = Some(s);
                        } 
                        if s >= a && dp[i - 1][s - a] {
                            dp[i][s] = true;
                            parent[i][s] = Some(s-a);
                        }
                    }
                }

                let best = (0..=target).rev().find(|&s| dp[buildings][s]).unwrap();

                let mut plants = Vec::new();
                let mut i = buildings;
                let mut current = best;
                loop {
                    let Some(next) = parent[i][current] else { break; };
                    if next != current {
                        plants.push(candidate_points[i - 1]);
                    }
                    current = next;
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }

                let bases: Vec<(usize, usize)> = candidate_points.iter().filter(|&p| !plants.contains(p))
                    .copied()
                    .collect();

                let mut output = String::new();
                writeln!(output, "{}", best).unwrap();
                for y in 0..problem.buildable.len() {
                    for x in 0..problem.width {
                        if (x, y) == problem.enemy {
                            write!(output, "M").unwrap();
                        } else if plants.contains(&(x, y)) {
                            write!(output, "E").unwrap();
                        } else if bases.contains(&(x, y)) {
                            write!(output, "B").unwrap();
                        } else if problem.can_build((x, y)) {
                            write!(output, ".").unwrap();
                        } else {
                            write!(output, "X").unwrap();
                        }
                    }
                    writeln!(output).unwrap();
                }
                output.pop(); // remove last \n

                output
            })
            .collect::<Vec<_>>(),
    )
}
