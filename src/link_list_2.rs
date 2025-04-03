use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

pub struct List<T: Debug + Clone> {
    head: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub struct Node<T: Debug>
where
    T: Clone,
{
    val: T,
    next: Link<T>,
}

impl<T: Debug> List<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(
            Node {
                val: value,
                next: self.head.take()
            }
        ));
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().and_then(|node| {
            let next = node.borrow().next.clone();
            self.head = next;
    
            let fallback_value = node.borrow().val.clone(); // Clone before moving

            Rc::try_unwrap(node)
                .ok()
                .map(|n| n.into_inner().val)
                .or_else(|| Some(fallback_value)) 
            })
    }

    pub fn pop_back(&mut self) {
        if self.head.is_none() {
            return; // Empty list, nothing to remove
        }
    
        // If there's only one node, remove it
        if self.head.as_ref().unwrap().borrow().next.is_none() {
            self.head = None;
            return;
        }
    
        // Traverse the list to find the second last node
        let mut curr = self.head.clone();
        let mut second_last = None;
    
        while let Some(node) = curr.clone() {
            if node.borrow().next.as_ref().unwrap().borrow().next.is_none() {
                second_last = Some(node);
                break;
            }
            curr = node.borrow().next.clone();
        }
    
        // Set `next` of `second_last` to `None`, effectively removing the last node
        if let Some(node) = second_last {
            node.borrow_mut().next = None;
        }
    }

    pub fn print_list(&self) {
        let mut curr = self.head.clone();
    
        while let Some(node) = curr {
            let node_ref = node.borrow();
            print!("{:?}", node_ref.val);
    
            curr = node_ref.next.clone();
            if curr.is_some() {
                print!(" -> ");
            }
        }
        println!(); // Add newline at the end for better readability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_print() {
        let mut list: List<i32> = List::new();
        list.push(9);
        list.push(5);
        list.push(4);
        list.push(1);
        list.push(6);
        list.push(3);
        list.push(2);
        list.push(8);
        list.push(7);

        list.print_list();

        // let x =  list.pop();
        // println!("\n{x:?}");
        list.pop_back();
        list.print_list();
        list.pop_back();
        list.print_list();

    }
}
