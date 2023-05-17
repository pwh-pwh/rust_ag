use std::iter::FromIterator;
use std::mem::take;
use std::rc::Rc;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize,
}

pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}
type Link<T> = Option<Box<Node<T>>>;

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let mut node = Node::new(element);
        node.next = None;
        match self.head.take() {
            Some(mut old_node) => {
                node.next = Some(old_node);
                self.head = Some(Box::new(node));
            }
            None => {
                self.head = Some(Box::new(node));
            }
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.len -= 1;
                self.head = node.next;
                Some(node.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => Some(&node.data),
            None => None,
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut v = vec![];
        while let Some(node) = self.pop() {
            v.push(node);
        }
        v.reverse();
        let mut list = SimpleLinkedList::new();
        for x in v {
            list.push(x);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for x in iter {
            list.push(x);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = vec![];
        while let Some(node) = linked_list.pop() {
            v.insert(0, node);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::LinkedList::SimpleLinkedList;

    #[test]
    fn test() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
        list.push(1);
        assert_eq!(list.len(), 1, "list's length must be 1");
        list.push(2);
        assert_eq!(list.len(), 2, "list's length must be 2");
    }

    #[test]
    fn test_from_slice() {
        let mut array = vec!["1", "2", "3", "4"];
        let mut list: SimpleLinkedList<_> = array.drain(..).collect();
    }
}
