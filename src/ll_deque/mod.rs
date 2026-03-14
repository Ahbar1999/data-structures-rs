pub mod ll_deque {
    type Link<T> = Option<Box<Node<T>>>;   

    pub struct List<'a, T> {
        head: Link<T>,
        tail: Option<&'a mut Node<T>>,
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

    impl<'a, T> List<'a, T> {
        pub fn new() -> List<'a, T> {
            List{head: None, tail: None}
        }
        
        pub fn push(&'a mut self, val: T) {
            let new_tail = Box::new(Node::new(val));
            
            // add the new node at the end of list
            self.tail = match self.tail.take() {
                Some(old_tail) => {
                    // associate(move) newly created to a field of List 
                    old_tail.next = Some(new_tail);
                    
                    // return the reference to the that field(that reference now lives as long as
                    // the container) 
                    old_tail.next.as_deref_mut() 
                },
                None => {
                    self.head = Some(new_tail);
                    
                    self.head.as_deref_mut()
                }
            }
        }
    }
}
