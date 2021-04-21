pub struct List<T> {
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem: T,
    next: Link<T>,
}


impl<T> List<T> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn push(&mut self) {
        unimplemented!()
    }

    pub fn pop(&mut self, elem: T) -> Self {
        unimplemented!()
    }

    pub fn peek(&self) -> &T {
        unimplemented!()
    }
}
