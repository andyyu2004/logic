use std::fmt::Display;

pub fn join<T>(ts: &[T], sep: &str) -> String
where
    T: Display,
{
    ts.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
}
