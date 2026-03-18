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

    pub fn snippet_diff_borrows() { // gets caught as UB in miri
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];           // Reference to 0th element
            let ptr2_at_0 = ref1_at_0 as *mut i32;  // Ptr to 0th element
            

            let ptr3_at_0 = ptr2_at_0.add(1);       // Ptr to 1st element            
            // problem: ref1_at_0 only has borrow to the first element of data;
            // we need two separate mutable borrows to a single allocation of data; and we
            // cant just do &mut data[i]  
            // solution: use split_at_mut() on a slice
            
            // even with stacked access, its an issue
            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;

            // Should be [20, 0, 0, ...]
            println!("{:?}", &data[..]);
        }
    }

    pub fn snippet_same_borrrows() {
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];            // Reference to 0th element
            let ptr2_at_0 = ref1_at_0 as *mut i32;   // Ptr to 0th element
            let ptr3_at_0 = ptr2_at_0;               // Ptr to 0th element
            let ptr4_at_0 = ptr2_at_0.add(0);        // Ptr to 0th element
            let ptr5_at_0 = ptr3_at_0.add(1).sub(1); // Ptr to 0th element

            // An absolute jumbled hash of ptr usages
            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ptr4_at_0 += 4;
            *ptr5_at_0 += 5;
            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;

            // Should be [20, 0, 0, ...]
            println!("{:?}", &data[..]);
        }
    }
        

    pub fn snippet_split_borrows() {
        unsafe {
            let mut data =  [0; 10];

            let slice1 = &mut data[..]; // create a slice of data

            let (slice2_at_0, slice3_at_1) = slice1.split_at_mut(1);

            let ref4_at_0 = &mut slice2_at_0[0];
            let ref5_at_1 = &mut slice3_at_1[0];
            let ptr6_at_0 = ref4_at_0 as *mut i32;
            let ptr7_at_1 = ref5_at_1 as *mut i32;
            
            // still need to follow the stack order
            *ptr7_at_1 += 7;
            *ptr6_at_0 += 6;
            *ref5_at_1 += 5;
            *ref4_at_0 += 4;

            println!("{:?}", &data[..]);
       }
    }

    pub fn snippet_range_borrows() {
        unsafe {
            let mut data = [0; 10];

            let slice1_all = &mut data[..];         // Slice for the entire array
            let ptr2_all = slice1_all.as_mut_ptr(); // Pointer for the entire array
            
            let ptr3_at_0 = ptr2_all;               // Pointer to 0th elem (the same)
            // now this is okay because ptr2_all is a pointer to the whole data not just data[0] 
            let ptr4_at_1 = ptr2_all.add(1);        // Pointer to 1th elem
            let ref5_at_0 = &mut *ptr3_at_0;        // Reference to 0th elem
            let ref6_at_1 = &mut *ptr4_at_1;        // Reference to 1th elem

            *ref6_at_1 += 6;
            *ref5_at_0 += 5;
            *ptr4_at_1 += 4;
            *ptr3_at_0 += 3;

            // Just for fun, modify all the elements in a loop
            // (Could use any of the raw pointers for this, they share a borrow!)
            for idx in 0..10 {
                *ptr2_all.add(idx) += idx;
            }

            // Safe version of this same code for fun
            for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
                *elem_ref += idx; 
            }

            // Should be [8, 12, 4, 6, 8, 10, 12, 14, 16, 18]
            println!("{:?}", &data[..]);
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
   
    /*
    #[test]
    fn test_err() {
        snippet_err();
    }
    */
    
    #[test]
    fn test_same_borrow () {
        snippet_same_borrrows();
    }
    
    /*
    #[test]
    fn test_different_borrow () {
        snippet_diff_borrows();
    }
    */
}
