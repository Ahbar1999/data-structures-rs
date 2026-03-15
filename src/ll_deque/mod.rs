pub mod ll_deque {
    use std::ptr;

    type Link<T> = Option<Box<Node<T>>>;   

    pub struct List<T> {
        head: Link<T>,
        tail: *mut Node<T>
    } 

    pub struct Node<T> {
        val: T,
        next: Link<T>,
    }

    impl<T> Node<T> {
        pub fn new(val: T) -> Node<T> {
            Node{val, next: None}
        }
    }

    impl<T> List<T> {
        pub fn new() -> List<T> {
            List{head: None, tail: ptr::null_mut() }
        }
        
        // forward storing references, two nodes in sequence: a -> b; 
        // b cannot be destroyed as long as a has a reference to it
        pub fn push(&mut self, val: T) {
            let mut new_tail = Box::new(Node::new(val));
            let raw_tail: *mut _ = &mut *new_tail; 
            
            if !self.tail.is_null() {
                unsafe {
                    (*self.tail).next = Some(new_tail);  
                }
            } else {
                // empty list
                self.head = Some(new_tail);
            }
           
            self.tail = raw_tail;
        }
        
        // pop from front
        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                self.head = old_head.next;

                if self.head.is_none() {
                    // update self.tail also
                    self.tail = ptr::null_mut();
                }

                old_head.val
            })
        } 
    }
}

#[cfg(test)]
mod test {
    use crate::ll_deque::ll_deque::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        // assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);   // the scond time you cannot create a mutable borrow as there exists a
                        // immutable reference in the list itself which peresists for as long as
                        // the list 

        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
