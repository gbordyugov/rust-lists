use std::rc::Rc;

pub struct Tree<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { head: None }
    }

    pub fn left(&self) -> Self {
        Tree {
            head: self.head.as_ref().and_then(
                |node| node.left.clone()
            )
        }
    }

    pub fn right(&self) -> Self {
        Tree {
            head: self.head.as_ref().and_then(
                |node| node.right.clone()
            )
        }
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree::new()
    }
}
