pub mod linked_list_v2 {
    use std::rc::Rc;

    pub struct List<T> {
        head: Option<Rc<Node<T>>> 
    }

    struct Node<T> {
        val: T,
        next: Option<Rc<Node<T>>>, 
    }

    impl<T> List<T> {
        pub fn new() -> Self { 
            List::<T>{ head: None }
        }

        pub fn head(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.val)
        }
    
        // add a node to the front of the list
        pub fn prepend(&self, val: T) -> List<T> {
            List { head: Some(Rc::new(Node{
                val: val, 
                next: self.head.clone()}
            ))} 
        }
        
        /*
        pub fn tail(&mut self) -> List<T> {
            List { 
                // .take() is better for non persistence 
                head: self.head.take().and_then(|node| {
                    node.next.clone() 
                }
            )} 
        }
        */
        
        // return a linked list wihout the head
        pub fn tail(&self) -> List<T> {
            List { 
                head: self.head.as_ref().and_then(|node| {
                    node.next.clone() 
                }
            )} 
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.val)
        }
        
        pub fn iter<'a>(&'a self) -> Iter<'a, T> {
            Iter(self.head.as_deref())
        }
    }
    
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut head = self.head.take();
            
            while let Some(node_rc) = head {
                if let Ok(mut node) = Rc::try_unwrap(node_rc) {
                    head = node.next.take(); 
                } else {
                    // Rc::try_unwrap() failed meaning 
                    // we cant drop this node yet
                    break;
                }
                // node is dropped here
            }
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

#[cfg(test)]
mod test {
    use crate::linked_list_v2::linked_list_v2::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
