use std::fmt::Debug;

pub struct List<T: Debug> {
    head: Option<Box<Node<T>>>,
}

// type Link<T> = Option<Box<Node<T>>>;
pub struct Node<T: Debug> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }  
    pub fn print_list(&self) {

        let mut curr = self.head.as_ref();
        while curr.is_some() {
            print!("{:?}",curr.unwrap().value);
            if curr.unwrap().next.is_some() {
                print!("->")
            }
            curr = curr.unwrap().next.as_ref();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_print() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.print_list();
    }
}
