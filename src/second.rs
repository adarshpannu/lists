#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    fn new() -> List {
        List { head: None }
    }

    fn push(&mut self, elem: i32) {
        match &self.head {
            None => {
                let node = Node { elem, next: None };
                self.head = Some(Box::new(node));
            }
            Some(node) => {
                let node = Node {
                    elem,
                    next: std::mem::replace(&mut self.head, None),
                };
                self.head = Some(Box::new(node));
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
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
