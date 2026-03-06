pub mod linked_list {
    pub struct List<T> {
        head: Option<Box<Node<T>>> 
    }

    // type Link = Option<Box<Node>>;
    struct Node<T> {
        val: T,
        next: Option<Box<Node<T>>>, 
    }

    impl<T> List<T> {
        pub fn new() -> Self { 
            List::<T>{ head: None }
        }

        pub fn push(&mut self, val: T) {
            let new_node= Box::new(Node::<T>{val: val, next: self.head.take()}); 
            
            self.head= Some(new_node);
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.val)
        }

        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.val
            })    
        }

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self) 
        }

        pub fn iter<'a>(&'a self) -> Iter<'a, T> {
            Iter(self.head.as_deref())
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut curr = self.head.take();
             
            while let Some(mut node) = curr {
                curr = node.next.take();
                // `node` is dropped here
            }
        }
    }
        
    // IntoItem: a tuple struct with List<T> as one of the fields
    // following syntax is same as `struct foo {bar: baz}`
    pub struct IntoIter<T>(List<T>);

    impl<T> Iterator for IntoIter<T> {
        type Item=T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }
    
    // field '0' doesnt have a name but it represents cursor of iterator 
    pub struct Iter<'a, T>(Option<&'a Node<T>>);

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        
        fn next(&mut self) -> Option<Self::Item> {
            self.0.map(|node| { 
                self.0 = node.next.as_deref(); 
                &node.val
            })
        }
    }
}
