pub struct List {
    head: Link,
}

enum Link {
    Nil,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

use std::mem;

impl List {
    pub fn new() -> Self {
        List { head: Link::Nil }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Nil),
        };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_popping_from_empty_list() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_singleton_list() {
        let mut list = List::new();

        list.push(1);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_doublet_list() {
        let mut list = List::new();

        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_triplet_list() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
