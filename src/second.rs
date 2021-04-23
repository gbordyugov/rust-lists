pub struct List<T> {
    head: Link<T>,
}


type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, t: T) {
        let new_node = Node {
            elem: t,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(
            |node| {
                self.head = node.next;
                node.elem
            }
        )
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(
            |node| {
                &node.elem
            }
        )
    }
}


impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();

        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}


/*
 * Implementing T Iterator.
 */

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}


/*
 * Implementing &T Iterator.
 */

pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(
            |node| {
                self.next = node.next.as_deref();
                &node.elem
            }
        )
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            next: self.head.as_deref()
        }
    }
}


/*
 * Implementing &T Iterator.
 */

pub struct ListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}


impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.next.take().map(
            |node| {
                self.next = node.next.as_deref_mut();
                &mut node.elem
            }
        )
    }
}


impl<T> List<T> {
    pub fn iter_mut(&mut self) -> ListIterMut<T> {
        ListIterMut {
            next: self.head.as_deref_mut()
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn pop_from_empty_list() {
        let mut list = List::<i32>::new();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_from_non_empty_list() {
        let mut list = List::new();
        for i in 1..4 {
            list.push(i)
        }
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_into_empty_list() {
        let list = List::<i32>::new();
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn peek_into_non_empty_list() {
        let mut list = List::new();
        list.push(1);

        assert_eq!(list.peek(), Some(&1));
    }

    #[test]
    fn iterator() {
        let mut list = List::new();
        for i in 1..4 {
            list.push(i)
        }

        assert_eq!(list.next(), Some(3));
        assert_eq!(list.next(), Some(2));
        assert_eq!(list.next(), Some(1));
        assert_eq!(list.next(), None);
        assert_eq!(list.next(), None);
    }

    #[test]
    fn ref_iterator() {
        let mut list = List::new();
        for i in 1..4 {
            list.push(i)
        }

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ref_mut_iterator() {
        let mut list = List::new();
        for i in 1..4 {
            list.push(i)
        }

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
