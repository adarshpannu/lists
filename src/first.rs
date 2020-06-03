#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

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

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

impl List {
    fn new() -> Self {
        List { head: Link::Empty }
    }

    fn push(&mut self, value: i32) {
        let new_node = Node {
            elem: value,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        let new_link = Link::More(Box::new(new_node));
        self.head = new_link;
    }

    fn pop(&mut self) -> Option<i32> {
        let result: Option<i32>;
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => result = None,
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }
        result
    }
}

mod test {
    #[test]
    fn run_test() {
        let mut l = super::List::new();
        l.push(45);
        l.push(56);
        assert_eq!(56, l.pop().unwrap());
        assert_eq!(45, l.pop().unwrap());
    }
}
