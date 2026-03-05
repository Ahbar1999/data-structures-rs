pub mod linked_list {
    pub struct List {
        head: Option<Box<Node>> 
    }

    // type Link = Option<Box<Node>>;
    struct Node {
        val: isize,
        next: Option<Box<Node>>, 
    }    

    impl List {
        pub fn new() -> Self { 
            List{ head: None }
        }

        pub fn push(&mut self, val: isize) {
            let new_node= Box::new(Node{val: val, next: self.head.take()}); 
            
            self.head= Some(new_node);
        }

        pub fn peek(&self) -> Option<isize> {
            self.head.as_ref().map(|node| node.val)
        }

        pub fn pop(&mut self) -> Option<isize> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.val
            })    
        }

    }

    impl Drop for List {
        fn drop(&mut self) {
            let mut curr = self.head.take();
             
            while let Some(mut node) = curr {
                curr = node.next.take();
                // `node` is dropped here
            }
        }
    }
}
