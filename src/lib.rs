use std::{
    fmt::Display,
    io::{self, Read, Result, Write},
    ops::{Shl, Shr},
};

pub struct WriteStream<'r, F>
where
    F: Write,
{
    inner: &'r mut F,
    buffer: Vec<Vec<u8>>,
}

impl<'r, F> WriteStream<'r, F>
where
    F: Write,
{
    pub fn new(inner: &'r mut F) -> Self {
        WriteStream {
            inner,
            buffer: Vec::new(),
        }
    }
    #[inline(always)]
    pub fn finish(&mut self) -> Result<()> {
        self.inner.write_all(&self.buffer.concat())
    }
}
impl<'r, F, R> Shl<R> for WriteStream<'r, F>
where
    F: Write,
    R: Display,
{
    type Output = Self;
    fn shl(mut self, rhs: R) -> Self::Output {
        let mut buffer = Vec::new();
        write!(&mut buffer, "{}", rhs).unwrap();
        self.buffer.push(buffer);
        self
    }
}
pub struct ReadStream<'r, F>
where
    F: Read,
{
    inner: &'r mut F,
}
impl<'r, F> ReadStream<'r, F>
where
    F: Read,
{
    pub fn new(inner: &'r mut F) -> Self {
        ReadStream { inner }
    }
}
impl<'r, F, R> Shr<R> for ReadStream<'r, F>
where
    F: Read,
    R: Write,
{
    type Output = Result<u64>;
    fn shr(self, mut rhs: R) -> Self::Output {
        io::copy(self.inner, &mut rhs)
    }
}
