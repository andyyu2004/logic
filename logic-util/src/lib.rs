use std::fmt::{Debug, Display};

pub fn join<T>(ts: &[T], sep: &str) -> String
where
    T: Display,
{
    ts.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
}

pub fn join_dbg<'a, T: Debug, I: ?Sized>(ts: &'a I, sep: &str) -> String
where
    &'a I: IntoIterator<Item = T>,
{
    ts.into_iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(sep)
}
