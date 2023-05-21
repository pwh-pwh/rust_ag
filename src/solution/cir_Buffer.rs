use std::mem;

pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    arr: Vec<T>,
    write_index: usize,
    read_index: usize,
    len: usize,
    max: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
where
    T: Clone + Default,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            arr: vec![T::default(); capacity],
            write_index: 0,
            read_index: 0,
            len: 0,
            max: capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.len == self.max {
            return Err(Error::FullBuffer);
        }
        self.arr[self.write_index] = element;
        self.write_index = (self.write_index + 1) % self.max;
        self.len += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer);
        }
        let r = mem::replace(&mut self.arr[self.read_index], T::default());
        self.len -= 1;
        self.read_index = (self.read_index + self.max + 1) % self.max;
        Ok(r)
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.write_index = 0;
        self.read_index = 0;
        for i in 0..self.max {
            let _ = mem::replace(&mut self.arr[i], T::default());
        }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.len != self.max {
            self.write(element);
        } else {
            let r = mem::replace(&mut self.arr[self.read_index], element);
            self.read_index = (self.read_index + self.max + 1) % self.max;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::cir_Buffer::CircularBuffer;

    #[test]
    fn test() {
        let mut buffer = CircularBuffer::new(2);
        assert!(buffer.write(1).is_ok());
        assert!(buffer.write(2).is_ok());
        assert_eq!(Ok(1), buffer.read());
        assert!(buffer.write(-1).is_ok());
    }
}
