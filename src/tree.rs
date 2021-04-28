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
        Tree::empty()
    }
}


#[cfg(test)]
mod test {
    use super::Tree;

    #[test]
    fn empty() {
        let empty = Tree::<i32>::empty();

        assert_eq!(empty.peek(), None);
    }

    #[test]
    fn singleton() {
        let empty = Tree::empty();
        let singleton = Tree::new(&empty, &empty, 3);

        assert_eq!(singleton.peek(), Some(&3));
        assert_eq!(singleton.left().peek(), None);
        assert_eq!(singleton.right().peek(), None);
    }

    #[test]
    fn list() {
        let empty = Tree::empty();
        let tree = Tree::new(&empty, &empty, 3);
        let tree = Tree::new(&tree, &empty, 2);
        let tree = Tree::new(&tree, &empty, 1);

        assert_eq!(tree.peek(), Some(&1));

        assert_eq!(tree.left().peek(), Some(&2));
        assert_eq!(tree.right().peek(), None);

        assert_eq!(tree.left().left().peek(), Some(&3));
        assert_eq!(tree.left().right().peek(), None);

        assert_eq!(tree.left().left().left().peek(), None);
        assert_eq!(tree.left().left().right().peek(), None);
    }
}
