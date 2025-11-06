pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, new_number: i32) {
        let new_node = Node {
            elem: new_number,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        let new_link = Link::More(Box::new(new_node));
        self.head = new_link;
    }

    pub fn pop(&mut self) -> Option<i32> {}
}
