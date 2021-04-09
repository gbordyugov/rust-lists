/*
 * That should be better called "A pointer to a list".
 */

pub struct List {
    ptr: Link,
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
            ptr: Link::Nil
        }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(
            Node {
                elem: elem,
                // This returns the value self.ptr and assigns
                // self.ptr to Link::Nil immediately afterwards.
                next: mem::replace(&mut self.ptr, Link::Nil),
            }
        );
        self.ptr = Link::More(new_node);
    }
}
