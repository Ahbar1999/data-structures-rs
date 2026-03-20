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
        pub front: Link<T>,
        pub back: Link<T>,
        pub len: usize,
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

        pub fn pop_front(&mut self) -> Option<T> {
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
        } 
    }
}

#[cfg(test)]
pub mod tests {
    use crate::ll_deque_final::ll_deque_final::*;
        
    #[test] 
    pub fn basics() {
        let mut list = List::<isize>::new();
        
        debug_assert!(list.pop_front() == None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        
        debug_assert!(list.len == 3); 

        debug_assert!(list.pop_front() == Some(3));
        debug_assert!(list.len == 2); 
        debug_assert!(list.pop_front() == Some(2));
        debug_assert!(list.pop_front() == Some(1));

        debug_assert!(list.len == 0); 

        debug_assert!(list.pop_front() == None);
    }
}

