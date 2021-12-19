use std::boxed::Box;
use std::fmt::Display;
use std::iter::Iterator;

struct Node<T>
where
    T: Display,
{
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Display,
{
    pub fn new(v: T) -> Node<T> {
        Node { val: v, next: None }
    }
}

pub struct SinglyLinkedList<T>
where
    T: Display,
{
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T>
where
    T: Display,
{
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            Some(_) => false,
            None => true,
        }
    }

    pub fn iter(&self) -> SllIterator<T> {
        SllIterator {
            node: self.head.as_ref(),
        }
    }

    pub fn push(&mut self, val: T) {
        let mut new_node = Node::new(val);
        new_node.next = self.head.take();
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.val),
            None => None,
        }
    }

    pub fn print(&self) {
        let mut output = String::new();
        for val in self.iter() {
            let s = val.to_string();
            let s = format!("{} -> ", s);
            output.push_str(&s[..]);
        }
        println!("{}", output);
    }
}

pub struct SllIterator<'a, T>
where
    T: Display,
{
    node: Option<&'a Box<Node<T>>>,
}

impl<'a, T> Iterator for SllIterator<'a, T>
where
    T: Display,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node.take() {
            Some(node_ref) => {
                let val_ref = &node_ref.val;
                self.node = node_ref.next.as_ref();
                Some(val_ref)
            }
            None => None,
        }
    }
}
