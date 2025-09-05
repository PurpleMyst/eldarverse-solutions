use std::fmt::Display;

pub fn cases<I>(it: I) -> String 
where I: IntoIterator, I::Item: Display,
{
    it.into_iter().enumerate().map(|(i, s)| format!("Case #{}: {}", i + 1, s)).collect::<Vec<_>>().join("\n")
}
