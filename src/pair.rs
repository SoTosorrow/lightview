use std::{
    fmt::{Debug, Display},
    iter::FromIterator,
};

#[derive(Debug)]
pub struct Pair<T>(T, T);

impl<T: Debug> Display for Pair<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{:?})", self.0, self.1)
    }
}

impl<T> FromIterator<T> for Pair<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Pair<T> {
        let mut er = iter.into_iter().take(2);

        let n0 = er.next().unwrap();
        let n1 = er.next().unwrap();
        Pair(n0, n1)
    }
}
