use std::ptr;

// Definition of a node in the linked list
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Implementation of the linked list
impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    fn append(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));

        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }

        current.next = Some(new_node);
    }

    fn merge_sort(&mut self) {
        if self.next.is_none() {
            return;
        }

        let (left, right) = self.split_list();

        left.merge_sort();
        right.merge_sort();

        *self = Self::merge(left, right);
    }

    fn split_list(&mut self) -> (Box<Node<T>>, Box<Node<T>>) {
        let mut slow = self;
        let mut fast = self;
        let mut prev = self;

        while fast.next.is_some() && fast.next.as_ref().unwrap().next.is_some() {
            fast = fast.next.as_mut().unwrap().next.as_mut().unwrap();
            prev = slow;
            slow = slow.next.as_mut().unwrap();
        }

        prev.next = None;

        (Box::new(*self), Box::new(*slow))
    }

    fn merge(left: Box<Node<T>>, right: Box<Node<T>>) -> Box<Node<T>> {
        let mut merged = Box::new(Node::new(left.data));
        let mut left_ptr = left.next;
        let mut right_ptr = Some(right);

        let mut current = &mut *merged;

        while left_ptr.is_some() && right_ptr.is_some() {
            if left_ptr.as_ref().unwrap().data <= right_ptr.as_ref().unwrap().data {
                let new_node = Box::new(Node::new(left_ptr.as_ref().unwrap().data));
                current.next = Some(new_node);
                current = current.next.as_mut().unwrap();
                left_ptr = left_ptr.unwrap().next;
            } else {
                let new_node = Box::new(Node::new(right_ptr.as_ref().unwrap().data));
                current.next = Some(new_node);
                current = current.next.as_mut().unwrap();
                right_ptr = right_ptr.unwrap().next;
            }
        }

        if left_ptr.is_some() {
            current.next = left_ptr;
        }

        if right_ptr.is_some() {
            current.next = right_ptr;
        }

        merged.next
    }

    fn display(&self) {
        let mut current = self;

        while let Some(ref next) = current.next {
            print!("{} -> ", current.data);
            current = next;
        }

        println!("{}", current.data);
    }
}

// Example usage:
fn main() {
    let mut linked_list = Box::new(Node::new(4));
    linked_list.append(2);
    linked_list.append(6);
    linked_list.append(1);
    linked_list.append(3);
    linked_list.append(5);
    linked_list.append(7);

    println!("Original linked list:");
    linked_list.display();

    linked_list.merge_sort();

    println!("Sorted linked list:");
    linked_list.display();
}
