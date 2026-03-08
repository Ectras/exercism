pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize_with(capacity, || None);
        Self {
            buffer,
            start: 0,
            end: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer[self.end].is_some() {
            return Err(Error::FullBuffer);
        }
        self.buffer[self.end] = Some(element);
        self.end = (self.end + 1) % self.buffer.len();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer[self.start].is_none() {
            return Err(Error::EmptyBuffer);
        }
        let element = std::mem::take(&mut self.buffer[self.start]).unwrap();
        self.start = (self.start + 1) % self.buffer.len();
        Ok(element)
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|el| {
            std::mem::take(el);
        });
        self.start = 0;
        self.end = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buffer[self.end].is_some() {
            self.start = (self.start + 1) % self.buffer.len();
        }
        self.buffer[self.end] = Some(element);
        self.end = (self.end + 1) % self.buffer.len();
    }
}
