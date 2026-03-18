pub mod ll_deque {
    use std::ptr;

    // type Link<T> = Option<Box<Node<T>>>;
    type Link<T> = *mut Node<T>;    // revised version

    pub struct List<T> {
        head: Link<T>,
        tail: Link<T> 
    } 

    pub struct Node<T> {
        val: T,
        next: Link<T>,
    }

    impl<T> Node<T> {
        pub fn new(val: T) -> Node<T> {
            Node{val, next: ptr::null_mut()}
        }
    }

    impl<T> List<T> {
        pub fn new() -> List<T> {
            List{head: ptr::null_mut(), tail: ptr::null_mut() }
        }
        
        // Old implementation: forward storing references, two nodes in sequence: a -> b; 
        // b cannot be destroyed as long as a has a reference to it
        
        // New implmentation: just use raw pointers
        pub fn push(&mut self, val: T) {
            unsafe {
                // essentially a way to get raw pointer malloc style  
                let new_tail = Box::into_raw(Box::new(Node{
                    val,
                    next: ptr::null_mut(),
                }));

                if !self.tail.is_null() {
                    (*self.tail).next = new_tail;
                } else {
                    self.head = new_tail;
                }

                self.tail = new_tail;
            } 
        }
        
        // pop from front
        pub fn pop(&mut self) -> Option<T> {
            unsafe {
                if self.head.is_null() {
                    None 
                } else {
                    let old_head = Box::from_raw(self.head);
                    self.head = old_head.next;

                    if self.head.is_null() {
                        self.tail = ptr::null_mut();
                    }
                    
                    // move the val here
                    Some(old_head.val)
                }
            }

            // old_head dropped here
        } 
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            while let Some(_) = self.pop() { }
        }
    }
}

#[cfg(test)]
#[ignore]
mod test {
    use crate::ll_deque::ll_deque::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

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

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
