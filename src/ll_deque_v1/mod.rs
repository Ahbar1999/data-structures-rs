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
                val: val,
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
                    node.borrow_mut().prev = Some(new_node);
                },
                None => {
                    // Brand new list
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
        }
    }
}
