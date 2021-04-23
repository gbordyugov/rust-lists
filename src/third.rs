use std::rc::Rc;


pub struct List<T> {
    head: Link<T>,
}


type Link<T> = Option<Rc<Node<T>>>;


struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn cons(&self, t: T) -> Self {
        let node = Node {
            elem: t,
            next: self.head.clone()
        };

        List { head: Some(Rc::new(node)) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(
            |node| &node.elem
        )
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(
                |node| node.next.clone()
            )
        }
    }
}


impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();

        while let Some(node) = cur {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                cur = node.next.take();
            } else {
                break;
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();

        assert_eq!(list.head(), None);

        let list = list.cons(1).cons(2).cons(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
