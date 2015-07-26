
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

impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            //next: self.head
            // Steal the value out of self.head
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Steal the value of self.head
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            // Match the box
            Link::More(boxed_node) => {
                // move entire node value onto the stack
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
