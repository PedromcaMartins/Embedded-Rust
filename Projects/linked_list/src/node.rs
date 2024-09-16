use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct Node {
    next: Option<Box<Node>>,
    value: i32
}

impl Node {
    pub fn new_root() -> Node {
        Node::new(0)
    }

    fn new(value: i32) -> Node {
        Node {
            next: None,
            value,
        }
    }

    pub fn add(&mut self, value: i32) {
        let mut last_node = self;
        while let Some(ref mut next_node) = last_node.next {
            last_node = next_node; // traverse the linked list
        }
        last_node.next = Some(Box::new(Node::new(value)));
    }

    pub fn remove(&mut self, value: i32) {
        let mut last_node = self;
        while let Some(ref mut next_node) = last_node.next {
            if next_node.value == value {
                let new_next_node = next_node.next;
                break;
            }
            last_node = next_node; // traverse the linked list
        }
    }

    pub fn insert(&mut self, position_in_linked_list: i32, value: i32) {
        todo!();
    }
}