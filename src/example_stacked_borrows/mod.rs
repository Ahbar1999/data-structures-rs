pub mod example_stacked_borrows {
    pub fn snippet_err() {
        unsafe {
            let mut data = 10;
            
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;
            let ref3 = &mut *ptr2;
            let ptr4 = ref3 as *mut _;
            
            *ref1 *= 1; // invalidates rest of the pointers/references

            *ptr4 *= 4;
            *ref3 *= 3;
            *ptr2 *= 2;

            println!("{}", data);
        }
    }

    pub fn snippet_ok() { 
        unsafe {
            let mut data = 10;
            
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;
            let ref3 = &mut *ptr2;
            let ptr4 = ref3 as *mut _;
            
            *ptr4 *= 4;
            *ref3 *= 3;
            *ptr2 *= 2;
            *ref1 *= 1; 

            println!("{}", data);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::example_stacked_borrows::example_stacked_borrows::*;

    #[test]
    fn test_ok() {
        snippet_ok();
    }
    
    #[test]
    fn test_err() {
        snippet_err();
    }
}
