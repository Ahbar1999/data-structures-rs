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

        pub fn iter<'a>(&'a self) -> Iter<'a, T> {
            unsafe {
                Iter(self.head.as_ref())
            }
        }

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }

        pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
            unsafe {
                IterMut(self.head.as_mut())
            }
        }

        pub fn peek(&self) -> Option<&T> {
            unsafe {
                if self.head.is_null() {
                    None
                } else {
                    Some(&(*self.head).val)
                }
            }
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            unsafe {
                if self.head.is_null() {
                    None
                } else {
                    Some(&mut (*self.head).val)
                }
            }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            while let Some(_) = self.pop() { }
        }
    }

    pub struct Iter<'a, T>(Option<&'a Node<T>>);   // stores a reference to the pointer to the Node
    pub struct IntoIter<T>(List<T>);
    
    pub struct IterMut<'a, T>(Option<&'a mut Node<T>>); 
    
    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                self.0.take().map(|node_ref| {
                    self.0 = node_ref.next.as_mut();
                    &mut node_ref.val
                })
            }   
        }
    }


    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                /*
                if self.0.is_null() {
                    None
                } else {
                    let old_head = Some(&(*(*self.0)).val); 
                    self.0 = &(*(*self.0)).next;  
                    
                    old_head  
                }
                */
                self.0.map(|node_ptr| {
                    self.0 = node_ptr.next.as_ref();
                    &node_ptr.val 
                })
            }
        } 
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            /*
            unsafe {
                if self.0.head.is_null() {
                    None
                } else {
                    let old_head = Box::from_raw(self.0.head);
                    self.0.head = (*old_head).next;

                    Some((*old_head).val)
                }
            }
            */
            
            self.0.pop() 
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
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);   // the scond time you cannot create a mutable borrow as there exists a
                        // immutable reference in the list itself which peresists for as long as
                        // the list 

        list.push(3);
        
        // test peek
        assert_eq!(list.peek(), Some(&1));

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        assert_eq!(list.peek(), Some(&3));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);
        
        let mut_val = list.peek_mut().unwrap();
        *mut_val = 31;

        // Check normal removal
        assert_eq!(list.pop(), Some(31));
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

    #[test]
    fn ref_iter() { 
        let mut list = List::new();

        list.push(1);
        list.push(2);   
        list.push(3);
        list.push(4);
        
        let mut iter = list.iter();

        assert_eq!(Some(&1), iter.next()); 
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next()); 
        assert_eq!(Some(&4), iter.next());
    }

    #[test]
    fn owning_iter() { 
        let mut list = List::new();

        list.push(1);
        list.push(2);   
        list.push(3);
        list.push(4);
        
        let mut iter = list.into_iter();

        assert_eq!(Some(1), iter.next()); 
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(3), iter.next()); 
        assert_eq!(Some(4), iter.next());
    } 
}
