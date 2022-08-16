use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { next: None, value }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(PartialEq, Debug)]
pub enum LinkedListError {
    IndexError(usize),
    Empty,
    Unknown,
}


impl<T> Display for LinkedList<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut elements_in_order = String::new();
        let mut counter = 0;
        loop {
            if let Ok(r) = self.get_at(counter) {
                if elements_in_order.is_empty() {
                    elements_in_order.push_str(&format!("{:?}", r));
                } else {
                    elements_in_order.push_str(&format!(" -> {:?}", r));
                }
            } else {
                break;
            }
            counter += 1;
        }

        write!(f, "{}", elements_in_order)
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn push(&mut self, value: T) {
        let _ = self.add_at(self.len, value);
    }

    pub fn add_at(&mut self, index: usize, value: T) -> Result<(), LinkedListError> {
        if index > self.len {
            return Err(LinkedListError::IndexError(index));
        }

        let mut iter_node = self.head.as_mut();
        self.len += 1;
        let mut new_node = Box::new(Node::new(value));

        if iter_node.is_none() {
            self.head = Some(new_node);
            return Ok(());
        }

        for _ in 0..index - 1 {
            iter_node = iter_node.map(|n| n.next.as_mut()).unwrap();
        }

        match iter_node {
            Some(node) => {
                new_node.next = node.next.take();
                node.next = Some(new_node);
                Ok(())
            }
            None => Err(LinkedListError::Unknown),
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Result<T, LinkedListError> {
        if self.len == 0 {
            return Err(LinkedListError::Empty);
        }

        if index >= self.len {
            return Err(LinkedListError::IndexError(index));
        }

        self.len -= 1;

        if index == 0 {
            let elem_to_remove = self.head.take().unwrap();
            self.head = elem_to_remove.next;
            return Ok(elem_to_remove.value);
        }

        let mut iter_element = self.head.as_mut();

        for _ in 0..index - 1 {
            iter_element = iter_element.map(|n| n.next.as_mut()).unwrap();
        }

        if let Some(element_before) = iter_element {
            let elem_to_remove = element_before.next.take().unwrap();
            element_before.next = elem_to_remove.next;
            return Ok(elem_to_remove.value);
        }

        Err(LinkedListError::Unknown)
    }

    pub fn pop(&mut self) -> Result<T, LinkedListError> {
        if self.len == 0 {
            return Err(LinkedListError::Empty);
        }
        self.remove_at(self.len - 1)
    }

    pub fn get_at(&self, index: usize) -> Result<&T, LinkedListError> {
        if index >= self.len {
            return Err(LinkedListError::IndexError(index));
        }

        let mut elem_iter = self.head.as_ref();

        for _ in 0..index {
            elem_iter = elem_iter.map(|n| n.next.as_ref()).unwrap();
        }

        match elem_iter {
            Some(n) => Ok(&n.value),
            None => Err(LinkedListError::Empty),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_new_linked_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.head == None);
    }

    #[test]
    fn add() {
        let mut list = LinkedList::new();

        list.push(31);
        list.push(66);
        list.push(420);

        let first = list.head.as_ref();
        assert_eq!(first.unwrap().as_ref().value, 31);

        let second = first.as_ref().unwrap().next.as_ref();
        assert_eq!(second.unwrap().value, 66);

        let third = second.as_ref().unwrap().next.as_ref();
        assert_eq!(third.unwrap().value, 420);

        assert_eq!(list.len, 3);
    }

    #[test]
    fn add_at() {
        let mut list = LinkedList::new();

        let res = list.add_at(0, 31);
        assert_eq!(res.is_ok(), true);
        assert_eq!(list.head, Some(Box::new(Node::new(31))));

        let res = list.add_at(1, 66);
        assert_eq!(res.is_ok(), true);
        assert_eq!(
            list.head,
            Some(Box::new(Node {
                next: Some(Box::new(Node::new(66))),
                value: 31
            }))
        );

        let res = list.add_at(1, 420);
        assert_eq!(res.is_ok(), true);
        assert_eq!(
            list.head,
            Some(Box::new(Node {
                value: 31,
                next: Some(Box::new(Node {
                    value: 420,
                    next: Some(Box::new(Node {
                        value: 66,
                        next: None,
                    }))
                }))
            }))
        );

        assert_eq!(list.len, 3);
    }

    #[test]
    fn index_out_of_range() {
        let mut list = LinkedList::new();
        let res = list.add_at(100, 123);

        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err(), LinkedListError::IndexError(100))
    }

    #[test]
    fn add_at_len() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(3);

        let res = list.add_at(list.len, 5);
        assert_eq!(res.is_ok(), true);

        let first = list.head.as_ref();
        assert_eq!(first.unwrap().as_ref().value, 1);

        let second = first.as_ref().unwrap().next.as_ref();
        assert_eq!(second.unwrap().value, 3);

        let third = second.as_ref().unwrap().next.as_ref();
        assert_eq!(third.unwrap().value, 5);

        assert_eq!(list.len, 3);
    }

    #[test]
    fn remove_at() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len, 3);
        let res = list.remove_at(1);
        assert_eq!(list.len, 2);

        assert_eq!(res, Ok(2));
        assert_eq!(
            list.head,
            Some(Box::new(Node {
                value: 1,
                next: Some(Box::new(Node::new(3)))
            }))
        );
    }

    #[test]
    fn remove_at_head_and_tail() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let head_res = list.remove_at(0);
        assert_eq!(head_res, Ok(1));
        assert_eq!(
            list.head,
            Some(Box::new(Node {
                value: 2,
                next: Some(Box::new(Node::new(3)))
            }))
        );

        assert_eq!(list.len, 2);
        let tail_res = list.remove_at(1);
        assert_eq!(list.len, 1);
        assert_eq!(tail_res, Ok(3));
        assert_eq!(list.head, Some(Box::new(Node::new(2))))
    }

    #[test]
    fn remove_at_len_index_out_of_range() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let res = list.remove_at(list.len);
        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err(), LinkedListError::IndexError(list.len));
    }

    #[test]
    fn remove_from_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        let res = list.pop();
        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err(), LinkedListError::Empty);
    }

    #[test]
    fn get_at() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let result = list.get_at(1);
        assert_eq!(result, Ok(&2));
    }

    #[test]
    fn get_at_head_and_tail() {
        let mut list = LinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let res = list.get_at(0);
        assert_eq!(res, Ok(&1));

        let res = list.get_at(list.len - 1);
        assert_eq!(res, Ok(&3));
    }

    #[test]
    fn get_element_at_index_out_of_range() {
        let list: LinkedList<i32> = LinkedList::new();

        let res = list.get_at(100);
        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err(), LinkedListError::IndexError(100));
    }

    #[test]
    fn get_at_len_index_out_of_range() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let res = list.get_at(list.len);
        assert_eq!(res.is_err(), true);
        assert_eq!(res.unwrap_err(), LinkedListError::IndexError(list.len));
    }

    #[test]
    fn display_trait_list() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(format!("{}", list), "1 -> 2 -> 3");
    }

    #[test]
    fn display_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(format!("{}", list), "");
    }
}
