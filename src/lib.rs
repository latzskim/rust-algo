pub mod lists;
pub mod sorting;

#[cfg(test)]
mod tests {
    use crate::lists::{RingBuffer, StateError};


    #[test]
    fn test_ring_buffer_scenario() {
        let mut rb = RingBuffer::new_with_cap(3);

        assert_eq!(Err(StateError::Empty), rb.get());
        assert_eq!(Ok(()), rb.push(1));
        assert_eq!(Ok(()), rb.push(2));
        assert_eq!(Ok(()), rb.push(3));
        
        assert_eq!(Ok(1), rb.get());
        assert_eq!(Ok(2), rb.get());

        assert_eq!(Ok(()), rb.push(4));
        assert_eq!(Ok(()), rb.push(5));

        assert_eq!(Err(StateError::Full), rb.push(6));

        assert_eq!(Ok(3), rb.get());
        assert_eq!(Ok(4), rb.get());
        assert_eq!(Ok(5), rb.get());
        assert_eq!(Err(StateError::Empty), rb.get());

    }
}
