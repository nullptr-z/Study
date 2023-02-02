use std::{io::Read, *};

pub struct BufReader<R> {
    inner: R,
    buf: Box<u8>,
    pos: usize,
    cap: usize,
}

impl<R> BufReader<R> {
    pub fn capacity(&self) -> usize {
        0
    }

    pub fn buffer(&self) -> &[u8] {
        &[]
    }
}

impl<R: Read> BufReader<R> {
    pub fn new(inner: R) -> BufReader<R> {}

    pub fn with_capacity(capacity: usize, inner: R) -> BufReader<R> {}
}

impl<R> fmt::Debug for BufReader<R>
where
    R: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {}
}
