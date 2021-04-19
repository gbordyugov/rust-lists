pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIterList<T>(List<T>);

impl<T> Iterator for IntoIterList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map( |node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    pub fn into_iter_list(self) -> IntoIterList<T> {
        IntoIterList(self)
    }

    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let n = Node {
            elem,
            next: self.head.take()
        };
        self.head = Some(Box::new(n))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map( |node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
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
        while let Some(node) = cur {
            cur = node.next
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_popping_from_empty_list() {
        let mut list = List::<i32>::new();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_singleton_list() {
        let mut list = List::new();

        list.push(1);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_doublet_list() {
        let mut list = List::new();

        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_triplet_list() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek_empty() {
        let list = List::<i32>::new();

        assert_eq!(list.peek(), None);

    }

    #[test]
    fn test_peek_nonempty() {
        let mut list = List::new();
        list.push(1);

        assert_eq!(list.peek(), Some(&1));
    }

    #[test]
    fn test_peek_mut_empty() {
        let mut list = List::<i32>::new();

        assert_eq!(list.peek_mut(), None);

    }

    #[test]
    fn test_peek_mut_nonempty() {
        let mut list = List::new();
        list.push(1);

        assert_eq!(list.peek_mut(), Some(&mut 1));
    }

    #[test]
    fn test_peek_mut_nonempty_mutate() {
        let mut list = List::new();
        list.push(1);

        assert_eq!(list.peek(), Some(&1));

        list.peek_mut().map(|val| {
            *val = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn test_into_iter_list() {
        let mut list = List::new();

        for i in 1..4 {
            list.push(i)
        }

        let mut iter = list.into_iter_list();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
