pub struct Tree {
    ptr: Link,
}

enum Link {
    Nil,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    left: Link,
    right: Link,
}
