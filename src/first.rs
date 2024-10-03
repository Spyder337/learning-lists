use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) -> () {
        let new_node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

mod test {
    use super::List;

    #[test]
    fn push_empty() -> () {
        let mut list = List::new();
        list.push(32);
        assert_eq!(list.pop(), Some(32));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_value() -> () {
        let mut list = List::new();
        list.push(32);
        list.push(64);
        list.push(96);
        list.push(128);
        assert_eq!(list.pop(), Some(128));
        assert_eq!(list.pop(), Some(96));
        assert_eq!(list.pop(), Some(64));
        assert_eq!(list.pop(), Some(32));
        assert_eq!(list.pop(), None);
    }
}
