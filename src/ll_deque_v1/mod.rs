pub mod ll_deque_v1 {
    use std::rc::Rc;
    use std::cell::{RefCell, Ref, RefMut};

    type Link<T> = Option<Rc<RefCell<Node<T>>>>;
        
    pub struct List<T> {
        head: Link<T>,
        tail: Link<T> 
    }

    struct Node<T> {
        val: T,
        prev: Link<T>,
        next: Link<T> 
    }

    impl<T> Node<T> {
        // returns Rc<RefCell<Node<T>>>
        fn new(val: T) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node {
                val,
                prev: None,
                next: None
            })
        )}
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None, tail: None }
        }
        
        pub fn push_front(&mut self, val: T) {
            let new_node = Node::new(val);

            match self.head.take() {
                Some(node) => {
                    // set the head the new node  
                    self.head = Some(new_node.clone());
                    // set the next of new head to the old head(which was taken out) 
                    new_node.borrow_mut().next = Some(node.clone());
                    // set the prev of old head to new head
                    // p.s.: we could have also clone new_node here but since there are no more
                    // uses to it further down this block we can safely move this value 
                    node.borrow_mut().prev = Some(new_node);
                },
                None => {
                    // Brand new list
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take(); // dinit; set prev of head to null; not
                                                           // adding this line would make
                                                           // Rc::try_unwrap() fail since there
                                                           // would be outstanding references
                        self.head = Some(new_head);
                    },
                    None => {
                        self.tail.take();
                    }
                }
                    
                Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
            }) 
        }
        
        pub fn pop_back(&mut self)  -> Option<T> {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                    },
                    None => {
                        // tail == head; single element list
                        // deinit head 
                        self.head.take(); 
                    }
                } 

                // just return tail value by taking inner value out of rc is there are no
                // outsanding references to it
                Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val 
            })
        }
        
        pub fn peek_front(&self) -> Option<Ref<T>> {
            self.head.as_ref().map(|head_ref| {
                Ref::map(head_ref.borrow(), |head_ref| { &head_ref.val }) 
            })
        }

        pub fn peek_back(&self) -> Option<Ref<T>> {
            self.tail.as_ref().map(|tail_ref| {
                Ref::map(tail_ref.borrow(), |tail_ref| { &tail_ref.val }) 
            })
        }
        
        pub fn push_back(&mut self, val: T) {
            let new_tail = Node::new(val);

            if let Some(old_tail) = self.tail.take() {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            } else {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        
        pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
            self.tail.as_ref().map(|tail_ref| {
                RefMut::map(tail_ref.borrow_mut(), |tail_ref: &mut Node<T>| {&mut tail_ref.val})
            })
        }

        pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
            self.head.as_ref().map(|head_ref| {
                RefMut::map(head_ref.borrow_mut(), |head_ref: &mut Node<T>| {&mut head_ref.val})
            })
        }

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)        
        }
        
        /*
        pub fn iter<'a>(&'a self) -> Iter<'a, T> {
            Iter(self.head.as_ref().map(|node| { node.borrow() }))
        }
        */
    }

    // implement drop trait
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            while self.pop_front().is_some() {
                self.pop_front();
            }
        }
    }
    
    // owns the List Object
    pub struct IntoIter<T>(List<T>);
    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop_front().take().map(|node| { node }) 
        }
    }
        
    impl<T> DoubleEndedIterator for IntoIter<T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.0.pop_back().take().map(|node| { node }) 
        }
    }
      
    /* NOTES:
     * this implementation doesnt work because the Ref<T> that we are trying to return is 
     * tied with the Ref<> of Node that we are iterating over 
     * */
    /*  
    pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);
    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = Ref<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.as_ref().map(|node_ref| {
                // in the line below you are trying to create and keep a reference(next_node.borrow()) while dropping its parent
                // reference(node_ref) at the end of scope
                self.0 = node_ref.next.as_ref().map(|next_node| { next_node.borrow() } );
                
                Ref::map(node_ref, |node_ref: &'a Node<T> | { &node_ref.val })
            })
        }
    }
    */
}

#[cfg(test)]
 mod test {
    use crate::ll_deque_v1::ll_deque_v1::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
        
    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }
        
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
