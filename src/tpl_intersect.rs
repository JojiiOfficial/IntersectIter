use std::{cmp::Ordering, iter::Peekable};

/// Intersect over Tuples where the first tuple item serves the order
pub struct TupleIntersect<A, B, K, T>
where
    A: Iterator<Item = (K, T)>,
    B: Iterator<Item = (K, T)>,
{
    a: Peekable<A>,
    b: Peekable<B>,
}

impl<A, B, K, T> TupleIntersect<A, B, K, T>
where
    A: Iterator<Item = (K, T)>,
    B: Iterator<Item = (K, T)>,
{
    #[inline]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a: a.peekable(),
            b: b.peekable(),
        }
    }
}

impl<A, B, K, T> Iterator for TupleIntersect<A, B, K, T>
where
    A: Iterator<Item = (K, T)>,
    B: Iterator<Item = (K, T)>,
    K: Ord,
{
    type Item = (K, T, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let a_peek = self.a.peek()?;
            let b_peek = self.b.peek()?;

            match a_peek.0.cmp(&b_peek.0) {
                Ordering::Less => {
                    self.a.next()?;
                }
                Ordering::Greater => {
                    self.b.next()?;
                }
                Ordering::Equal => {
                    let (dim, value_a) = unsafe { self.a.next().unwrap_unchecked() };
                    let (_, value_b) = unsafe { self.b.next().unwrap_unchecked() };
                    return Some((dim, value_a, value_b));
                }
            }
        }
    }
}
