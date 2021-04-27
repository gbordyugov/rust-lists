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
