use std::{cmp::Ordering, iter::Peekable};

pub struct Intersect<A, B, K>
where
    A: Iterator<Item = K>,
    B: Iterator<Item = K>,
{
    a: Peekable<A>,
    b: Peekable<B>,
}

impl<A, B, K> Intersect<A, B, K>
where
    A: Iterator<Item = K>,
    B: Iterator<Item = K>,
{
    #[inline]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a: a.peekable(),
            b: b.peekable(),
        }
    }
}

impl<A, B, K> Iterator for Intersect<A, B, K>
where
    A: Iterator<Item = K>,
    B: Iterator<Item = K>,
    K: Ord,
{
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let a_peek = self.a.peek()?;
            let b_peek = self.b.peek()?;

            match a_peek.cmp(b_peek) {
                Ordering::Less => {
                    self.a.next()?;
                }
                Ordering::Greater => {
                    self.b.next()?;
                }
                Ordering::Equal => {
                    let _ = unsafe { self.b.next().unwrap_unchecked() };
                    return self.a.next();
                }
            }
        }
    }
}
