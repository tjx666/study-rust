struct Node {
    elem: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> List {
        List { head: None }
    }

    // 头插法
    pub fn push(&mut self, v: i32) {
        let new_node = Node {
            elem: v,
            next: self.head.take(),
        };
        // head 其实始终指向尾结点
        self.head = Some(Box::new(new_node));
    }

    // 返回 head 指向的尾结点
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

#[test]
fn long_list() {
    let mut list = List::new();
    for i in 0..100000 {
        list.push(i);
    }
    drop(list);
}
