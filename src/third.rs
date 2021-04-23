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

    pub fn cons(&self, elem: T) -> Self {
        unimplemented!()
    }

    pub fn head(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn tail(&self) -> List<T> {
        unimplemented!()
    }
}


impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        unimplemented!()
    }
}
