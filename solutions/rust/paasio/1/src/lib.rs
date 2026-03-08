use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R>(R, usize, usize);

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self(wrapped, 0, 0)
    }

    pub fn get_ref(&self) -> &R {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.1
    }

    pub fn reads(&self) -> usize {
        self.2
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.2 += 1;
        let actually_read = self.0.read(buf)?;
        self.1 += actually_read;
        Ok(actually_read)
    }
}

pub struct WriteStats<W>(W, usize, usize);

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self(wrapped, 0, 0)
    }

    pub fn get_ref(&self) -> &W {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.1
    }

    pub fn writes(&self) -> usize {
        self.2
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.2 += 1;
        let actually_written = self.0.write(buf)?;
        self.1 += actually_written;
        Ok(actually_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
