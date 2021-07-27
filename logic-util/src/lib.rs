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

pub fn fmt_generic_args<'a, T: Debug, I: ?Sized>(ts: &'a I) -> String
where
    &'a I: IntoIterator<Item = T>,
{
    let vec = ts.into_iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>();
    if vec.is_empty() { String::new() } else { format!("<{}>", vec.join(", ")) }
}
