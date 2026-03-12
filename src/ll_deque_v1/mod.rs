pub mod ll_deque_v1 {
    use std::rc::Rc;
    use std::cell::RefCell;

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
                if let Some(new_head) = old_head.borrow_mut().next.take() {
                    new_head.borrow_mut().prev.take();   // deinitialize prev pointer 

                    self.head = Some(new_head);
                } else {
                    // single node case; deinit tail pointer
                    self.tail.take();
                }
                    
                Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
            }) 
        }
        
        // TODO:
        // pop_back(), peek_back(), peek_front(), push_back() 
        pub fn pop_back(&mut self)  -> Option<T> {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
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
    }
    
    // implement drop trait
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            while self.pop_front().is_some() {
                self.pop_front();
            }
        }
    }
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
    }
}
