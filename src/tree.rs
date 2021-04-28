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
    pub fn empty() -> Self {
        Tree { head: None }
    }

    pub fn new(left: &Self, right: &Self, t: T) -> Self {
        let node = Node {
            elem: t,
            left: left.head.clone(),
            right: right.head.clone(),
        };

        Tree {
            head: Some(Rc::new(node))
        }
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

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|n| &n.elem)
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree::new()
    }
}
