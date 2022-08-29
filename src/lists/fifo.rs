use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq)]
struct Node<T> {
    next: Rc<RefCell<Option<Node<T>>>>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            next: Rc::new(RefCell::new(None)),
            value,
        }
    }
}

pub struct Fifo<T> {
    head: Rc<RefCell<Option<Node<T>>>>,
    tail: Rc<RefCell<Option<Node<T>>>>,
    len: usize,
}

impl<T> Fifo<T> {
    fn new() -> Self {
        Fifo {
            head: Rc::new(RefCell::new(None)),
            tail: Rc::new(RefCell::new(None)),
            len: 0,
        }
    }

    fn push(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Some(Node::new(value))));
        if self.tail.borrow().is_some() {
            self.tail.borrow_mut().as_mut().unwrap().next = Rc::clone(&new_node);
            self.tail = Rc::clone(&new_node);
        } else {
            self.head = Rc::clone(&new_node);
            self.tail = Rc::clone(&new_node);
        }
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.head.borrow().is_none() {
            return None;
        }

        self.len -= 1;
        let to_ret = self.head.borrow_mut().take();
        match to_ret {
            Some(node) => {
                self.head = node.next;
                return Some(node.value);
            }
            None => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_element() {
        let mut fifo = Fifo::new();
        fifo.push(1);
        assert_eq!(fifo.len, 1);
        assert_eq!(fifo.head.borrow().as_ref().unwrap().value, 1);
        assert_eq!(fifo.tail.borrow().as_ref().unwrap().value, 1);
    }

    #[test]
    fn add_more_elements() {
        let mut fifo = Fifo::new();
        fifo.push(1);
        fifo.push(2);
        assert_eq!(fifo.len, 2);
        assert_eq!(fifo.head.borrow().as_ref().unwrap().value, 1);
        assert_eq!(fifo.tail.borrow().as_ref().unwrap().value, 2);
    }

    #[test]
    fn remove_last_element_from_empty_fifo() {
        let mut fifo: Fifo<i32> = Fifo::new();
        let _ = fifo.pop();
        assert_eq!(fifo.len, 0);
        assert_eq!(fifo.head.borrow().is_none(), true);
        assert_eq!(fifo.tail.borrow().is_none(), true);
    }

    #[test]
    fn remove_last_element_from_non_empty_fifo() {
        let mut fifo = Fifo::new();
        fifo.push(1);
        let element = fifo.pop();
        assert_eq!(element, Some(1));
        assert_eq!(fifo.len, 0);
        assert_eq!(fifo.head.borrow().is_none(), true);
        assert_eq!(fifo.tail.borrow().is_none(), true);
    }

    #[test]
    fn remove_element_from_fifo() {
        let mut fifo = Fifo::new();
        fifo.push(1);
        fifo.push(2);
        fifo.push(3);
        let element = fifo.pop();
        assert_eq!(element, Some(1));
        assert_eq!(fifo.len, 2);
        assert_eq!(fifo.head.borrow().as_ref().unwrap().value, 2);
        assert_eq!(fifo.tail.borrow().as_ref().unwrap().value, 3);

        let element = fifo.pop();
        assert_eq!(element, Some(2));
        assert_eq!(fifo.len, 1);
        assert_eq!(fifo.head.borrow().as_ref().unwrap().value, 3);
        assert_eq!(fifo.tail.borrow().as_ref().unwrap().value, 3);

        let element = fifo.pop();
        assert_eq!(element, Some(3));
        assert_eq!(fifo.len, 0);
        assert_eq!(fifo.head.borrow().is_none(), true);
        assert_eq!(fifo.tail.borrow().is_none(), true);
    }
}
