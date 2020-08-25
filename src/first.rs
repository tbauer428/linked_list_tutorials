use std::mem;

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
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
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

mod test {
    use super::*;

    #[test]
    fn new_list_empty() {
        let mut bbs_list: List = List::new();

        assert_eq!(bbs_list.pop(), None)
    }

    #[test]
    fn pop() {
        let mut bbs_list: List = List::new();

        bbs_list.push(9001);
        bbs_list.push(12);
        bbs_list.push(42);

        assert_eq!(bbs_list.pop(), Some(42));
        assert_eq!(bbs_list.pop(), Some(12))
    }

    #[test]
    fn push() {
        let mut bbs_list: List = List::new();

        bbs_list.push(9001);
        bbs_list.push(12);
        bbs_list.push(42);

        assert_eq!(bbs_list.pop(), Some(42));
        assert_eq!(bbs_list.pop(), Some(12))
    }


}