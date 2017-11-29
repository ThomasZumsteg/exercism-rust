#[derive(Debug, PartialEq)]
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer
}

impl <T: Clone> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer{ 
            buffer: Vec::with_capacity(size),
            capacity: size,
        }
    }

    pub fn read(&mut self) -> Result<T,Error> { 
        if self.buffer.is_empty() { return Err(Error::EmptyBuffer) }
        Ok(self.buffer.remove(0))
    }

    pub fn write(&mut self, item: T) -> Result<(),Error> {
        if self.buffer.len() >= self.capacity { return Err(Error::FullBuffer) }
        self.buffer.push(item);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, item: T) {
        if self.buffer.len() >= self.capacity { self.read(); }
        self.write(item);
    }
}

