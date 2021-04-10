pub struct List {
    head: Link,
}


enum Link {
    Nil,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
