struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> List {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, v: i32) {
        let new_node = Node {
            elem: v,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }

    fn pop() -> Option<i32> {
        unimplemented!()
    }
}
