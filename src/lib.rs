pub mod linked_list; 

#[cfg(test)]
mod tests {
    use crate::linked_list;
    
    #[test]
    fn linked_list_v1_basics() {
        let mut head = linked_list::linked_list::List::new();

        assert!(head.peek() == None);

        for val in 0..10 {
            head.push(val);
            assert!(head.peek() == Some(&val));
        } 

        for i in 0..10 {
            assert!(head.pop() == Some(9 - i));
        }

        assert!(head.peek()== None);

        for val in 0..10 {
            head.push(val);
        }
        
        {
            let mut i = 9;
            for val in head.into_iter() {
                assert!(val == i);
                i -= 1;
            }
        }
    }

    #[test]
    fn linked_list_v1_iter() {
        let mut list = linked_list::linked_list::List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn linked_list_v1_iter_mut() {
        let mut list = linked_list::linked_list::List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
