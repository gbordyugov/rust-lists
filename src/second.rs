pub struct List<T> {
    head: Link<T>,
}


type Link<T> = Option<Box<Node<T>>>;


struct Node<T>{
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

    pub fn peek(&self) -> &T {
        unimplemented!()
    }
}


impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn popping_from_empty_list() {
        let mut list = List::<i32>::new();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn popping_from_non_empty_list() {
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
}
