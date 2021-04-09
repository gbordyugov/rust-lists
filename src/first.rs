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

impl List {
    pub fn new() -> Self {
        List {
            ptr: Link::Nil
        }
    }
}
