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
        List {
            head: Link::Nil
        }
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
