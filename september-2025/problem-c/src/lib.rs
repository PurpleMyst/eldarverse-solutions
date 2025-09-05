use std::fmt::{Display, Write};

#[inline]
pub fn solve() -> impl Display {
    let mut lines = include_str!("input.txt").lines();
    let keywords = lines
        .by_ref()
        .take_while(|&line| line != "=====")
        .collect::<Vec<_>>();
    let actions = lines.next().unwrap().trim().bytes();

    let mut needle = String::new();
    let mut output = String::new();
    writeln!(output, "Case #1:").unwrap();

    for action in actions {
        match action {
            b'a'..=b'z' => {
                needle.push(action as char);
            }

            b'<' => {
                let _ = needle.pop();
            }

            _ => unreachable!("unexpected item in bagging area: {action:?}"),
        }

        if needle.len() >= 3 {
            writeln!(
                output,
                "{}",
                keywords.iter().filter(|kw| kw.starts_with(&needle)).count()
            )
            .unwrap();
        }
    }

    output
}
