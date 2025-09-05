use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Write};

use common::*;

#[inline]
pub fn solve() -> impl Display {
    let mut lines = include_str!("input.txt").lines();
    let mut outputs = Vec::new();
    for _ in 0..lines.next().unwrap().parse::<usize>().unwrap() {
        let mut output = "\n".to_string();
        let (n, m) = lines.next().unwrap().split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();
        let m: usize = m.parse().unwrap();
        let usernames: HashSet<_> = lines.by_ref().take(n).collect();
        let mut friends: HashMap<&str, HashSet<&str>> = HashMap::default();
        for operation in lines.by_ref().take(m) {
            let (opcode, args) = operation.split_once(' ').unwrap();
            match opcode {
                "ADD" => {
                    let (alice, bob) = args.split_once(' ').unwrap();
                    friends.entry(alice).or_default().insert(bob);
                    friends.entry(bob).or_default().insert(alice);
                }

                "REMOVE" => {
                    let (alice, bob) = args.split_once(' ').unwrap();
                    friends.entry(alice).or_default().remove(bob);
                    friends.entry(bob).or_default().remove(alice);
                }

                "SUGGEST" => {
                    let alice = args;
                    let Some(friends_of_alice) = friends.get(alice) else {
                        writeln!(
                            output,
                            "{}",
                            usernames.iter().filter(|&&bob| bob != alice).min().unwrap()
                        )
                        .unwrap();
                        continue;
                    };
                    let candidates = usernames
                        .difference(friends_of_alice)
                        .copied()
                        .filter(|&x| x != alice);
                    let new_friend = candidates
                        .max_by_key(|&bob| {
                            let Some(friends_of_bob) = friends.get(bob) else {
                                return (0, Reverse(bob));
                            };
                            (
                                friends_of_alice.intersection(friends_of_bob).count(),
                                Reverse(bob),
                            )
                        })
                        .unwrap();
                    writeln!(output, "{}", new_friend).unwrap();
                }

                _ => unreachable!("unknown opcode {opcode:?}"),
            }
        }
        outputs.push(output);
    }
    cases(outputs)
}
