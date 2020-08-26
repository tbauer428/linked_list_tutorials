use std::mem;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn new_list_empty() {
        let mut bbs_list: List<String> = List::new();

        assert_eq!(bbs_list.pop(), None)
    }

    #[test]
    fn pop() {
        let mut bbs_list: List<i32> = List::new();

        bbs_list.push(9001);
        bbs_list.push(12);
        bbs_list.push(42);

        assert_eq!(bbs_list.pop(), Some(42));
        assert_eq!(bbs_list.pop(), Some(12))
    }

    #[test]
    fn push() {
        let mut bbs_list: List<i32> = List::new();

        bbs_list.push(9001);
        bbs_list.push(12);
        bbs_list.push(42);

        assert_eq!(bbs_list.pop(), Some(42));
        assert_eq!(bbs_list.pop(), Some(12))
    }

    #[test]
    fn drop() {

        let mut bbs_list: List<i32> = List::new();

        bbs_list.push(9001);
        bbs_list.push(12);
        bbs_list.push(42);

        std::mem::drop(bbs_list);

    }


}