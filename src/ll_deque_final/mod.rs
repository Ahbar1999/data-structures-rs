pub mod ll_deque_final {
    use std::ptr::NonNull;
    use std::marker::PhantomData;

    type Link<T> = Option<NonNull<Node<T>>>;
    
    pub struct Node<T> {
        prev: Link<T>,
        next: Link<T>,
        val: T
    }

    impl<T> Node<T> {
        pub fn new(val: T) -> Self {
            Node{ prev: None, next: None, val } 
        }
    }

    pub struct List<T> {
        front: Link<T>,
        back: Link<T>,
        len: usize,
        _dummy: PhantomData<T>  // List owns the data is points, it needs to tell compiler that so the lifetimes of the data can be bound finitely 
    }

    impl<T> List<T> {   
        pub fn new() -> Self {
            List { 
                front: None, 
                back :None, 
                len: 0,
                _dummy: PhantomData // 0 sized type 
            }
        }
        
        pub fn len(&self) -> usize {
            self.len
        }

        pub fn push_front(&mut self, val: T) {
            unsafe {
                let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(val))));
                
                if let Some(old) = self.front {
                    (*old.as_ptr()).prev = Some(new);
                    (*new.as_ptr()).next = Some(old);
                } else {
                    debug_assert!(self.back.is_none());
                    debug_assert!(self.front.is_none());
                    debug_assert!(self.len == 0);
                    self.back = Some(new);
                }

                self.front = Some(new);
                self.len += 1;
            }
        }
    

        pub fn pop_back(&mut self) -> Option<T> {
            unsafe {
                self.back.map(|tail_ptr| {
                    let tail_boxed = Box::from_raw(tail_ptr.as_ptr());

                    if let Some(next_tail) = tail_boxed.prev {
                        (*next_tail.as_ptr()).next = None; 
                        self.back = Some(next_tail);
                    } else {
                        self.front = None;  // deinit head
                    }

                    self.len -= 1;

                    tail_boxed.val
                })
            }
        }

        pub fn pop_front(&mut self) -> Option<T> {
            /*
            unsafe {
                self.front.take().map(|old_head| {
                    if let Some(new_head) = (*old_head.as_ptr()).next {
                        (*new_head.as_ptr()).prev = None;
                        self.front = Some(new_head);
                    } else {
                        // no new head
                        debug_assert!(self.len == 1);
                        self.front = None;
                        self.back = None;
                    }

                    self.len -= 1;
                    // std::ptr::read() copies value regardless of wether T is copy
                    old_head.read().val
                })
            }
            */
            unsafe {
                self.front.map(|node| {
                    // this is a common pattern to brind the object back into existence sort of
                    // to facilitate moves(drops, transfer etc.)
                    let boxed_node = Box::from_raw(node.as_ptr());
                    let result = boxed_node.val;    // a move occurs here

                    self.front = boxed_node.next;
                    if let Some(new_front) = self.front {
                        (*new_front.as_ptr()).prev = None;
                    } else {
                        // list emptied
                        self.back = None;
                    }
                
                    self.len -= 1;
                    result

                    // boxed_node(uninitialized) gets dropped here 
                })
            }
        }
    
        pub fn front(&self) -> Option<&T> {
            unsafe {
                self.front.map(|nn_node_ptr|{
                    &((*nn_node_ptr.as_ptr()).val)
                })
            }
        }

        pub fn back(&self) -> Option<&T> {
            unsafe {
                self.back.map(|nn_node_ptr|{
                    &((*nn_node_ptr.as_ptr()).val)
                })
            }
        }

        pub fn front_mut(&self) -> Option<&mut T> {
            unsafe {
                self.front.map(|nn_node_ptr|{
                    &mut ((*nn_node_ptr.as_ptr()).val)
                })
            }
        }

        pub fn back_mut(&self) -> Option<&mut T> {
            unsafe {
                    self.back.map(|nn_node_ptr|{
                        &mut ((*nn_node_ptr.as_ptr()).val)
                    })
                }
        }

        pub fn iter(&self) -> Iter<T> {
            Iter {
                front: self.front,   // head of iterator
                back: self.back,    // tail of iterator
                len: self.len,      // number of elements left to iterator over; used for
                                    // ExactIterator
                _boo: PhantomData   // for bounding lifetimes
            }
        }
        
        /*
        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter { next: self }
        }
        */
        
        pub fn iter_mut(&mut self) -> IterMut<T> {
            IterMut { next: &mut self.front }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            while let Some(_) = self.front {
                self.pop_front();
            }
        } 
    } 

    impl<'a, T> IntoIterator for &'a List<T> {
        type IntoIter = Iter<'a, T>;    // for now return Iter impl; change it to IntoIter when
                                        // thats available 
        type Item= &'a T;

        fn into_iter(self) -> Self::IntoIter {
            self.iter() // this will be changed to self.into_iter()
        }
    }

    pub struct Iter<'a, T> {
        front: Link<T>,
        back: Link<T>,
        len: usize,
        _boo: PhantomData<&'a T>
    }
    
    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.len > 0 {
                self.front.map(|node| unsafe {
                    self.len -= 1;
                    self.front = (*node.as_ptr()).next;
                    &(*node.as_ptr()).val
                })
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.len, Some(self.len))
        }
    }
    
    impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.len != 0 {
                self.back.map(|node_ptr| unsafe {
                    self.len -= 1;
                    self.back = (*node_ptr.as_ptr()).prev;

                    &(*node_ptr.as_ptr()).val
                }) 
            } else {
                None
            }
        }
    }

    impl<'a, T> ExactSizeIterator for Iter<'a, T> {
        fn len(&self) -> usize {
            self.len
        }
    }

    pub struct IterMut<'a, T> { next: &'a mut Link<T> }
    // pub struct IntoIter<T> { next: List<T> }
    
    impl<T> Iterator for IntoIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> { 
            self.next.pop_front() 
        }
    } 

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                self.next.as_ref().map(|node_ptr| {
                    &mut (*node_ptr.as_ptr()).val 
                })
            }
        }
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                self.next.as_ref().map(|node_ptr| {
                    & (*node_ptr.as_ptr()).val 
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ll_deque_final::ll_deque_final::*;

    #[test]
    fn test_basic_front() {
        let mut list = List::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
