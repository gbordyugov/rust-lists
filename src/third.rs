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

    pub fn append1(&self, elem: T) -> List<T> {
        let node = Node {
            elem,
            next: self.head.as_ref().map(
                |node| Rc::clone(node)
            ),
        };

        List {
            head: Some(Rc::new(node))
        }
    }

    pub fn append(&self, elem: T) -> List<T> {
        let node = Node {
            elem,
            next: self.head.clone(),
        };

        List {
            head: Some(Rc::new(node))
        }
    }


    pub fn tail(&self) -> List<T> {
        List {
            // Option.and_then() is just a flatMap().
            head: self.head.as_ref().and_then(
                |node| {
                    node.next.clone()
                }
            )
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}
