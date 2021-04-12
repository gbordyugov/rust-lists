pub struct List<T> {
    head: Link<T>
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

    pub fn push(&mut self, elem: T) {
        let n = Node {
            elem: elem,
            next: self.head.take(),
        };

        self.head = Link::Some(Box::new(n))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();

        while let Some(mut node) = cur {
            cur = node.next.take()
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
}
