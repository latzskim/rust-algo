const DEAFULT_CAPACITY: usize = 10;

#[derive(PartialEq, Debug)]
pub enum StateError {
    Empty,
    Full,
}

pub struct RingBuffer<T: Clone + Default> {
    buffer: Vec<T>,
    read: usize,
    write: usize,
}

impl<T: Clone + Default> RingBuffer<T> {
    pub fn new() -> Self {
        RingBuffer {
            buffer: vec![T::default(); DEAFULT_CAPACITY],
            read: 0,
            write: 0,
        }
    }

    pub fn new_with_cap(cap: usize) -> Self {
        RingBuffer {
            buffer: vec![T::default(); cap],
            read: 0,
            write: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), StateError> {
        if self.is_full() {
            return Err(StateError::Full);
        }

        let len = self.buffer.len();
        self.buffer[self.write % len] = value;
        self.write = self.write + 1;

        return Ok(());
    }

    pub fn get(&mut self) -> Result<T, StateError> {
        if self.is_empty() {
            return Err(StateError::Empty)
        }

        let len = self.buffer.len(); 
        let value = self.buffer[self.read % len].clone();
        self.read = self.read + 1;
        Ok(value)
    }

    fn is_full(&self) -> bool {
        self.write - self.read == self.buffer.len()
    }

    fn is_empty(&self) -> bool {
        self.write - self.read == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exceeded_capacity() {
        let mut rb = RingBuffer::new_with_cap(3);

        assert_eq!(Ok(()), rb.push(1));
        assert_eq!(Ok(()), rb.push(2));
        assert_eq!(Ok(()), rb.push(3));

        assert_eq!(Err(StateError::Full), rb.push(4));
    }

    #[test]
    fn test_get_from_empty() {
        let mut rb = RingBuffer::<i8>::new_with_cap(3);
        assert_eq!(Err(StateError::Empty), rb.get());
    }

    #[test]
    fn test_get() {
        let mut rb = RingBuffer::<i8>::new_with_cap(3);
        rb.push(1);
        rb.push(2);

        assert_eq!(Ok(1), rb.get());
        assert_eq!(Ok(2), rb.get());
        assert_eq!(Err(StateError::Empty), rb.get());
    }
}
