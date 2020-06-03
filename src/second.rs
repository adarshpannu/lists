#![allow(dead_code)]
#![allow(unused_variables)]

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    fn peek0(&self) -> Option<&T> {
        //&self.head.map(|node| { node.elem })
        match &self.head {
            None => None,
            Some(link) => Some(&link.elem),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
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

        l.push(55);
        assert_eq!(&55, l.peek().unwrap());

    }
}
