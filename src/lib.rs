pub mod linked_list; 

#[cfg(test)]
mod tests {
    use crate::linked_list;

    
    #[test]
    fn test_linked_list() {
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
}
