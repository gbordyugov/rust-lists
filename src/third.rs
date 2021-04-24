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
            if let Ok(node) = Rc::try_unwrap(node) {
                cur = node.next;
            } else {
                break;
            }
        }
    }
}


/*
 * Implementing iterator.
 */

pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}


impl<T> List<T> {
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            next: self.head.as_deref()
        }
    }
}


impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(
            |node| {
                self.next = node.next.as_deref();
                &node.elem
            }
        )
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

    #[test]
    fn iter() {
        let list = List::new().cons(1).cons(2).cons(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
