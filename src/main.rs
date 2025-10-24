fn main() {
    // You can optionally experiment here.
    slice_out_of_array() 
}

#[cfg(test)]
// mod tests {
    #[test]
    fn slice_out_of_array() {
        let a:[&str; 5] = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        fn slice (arr: Array){
            let slice: (u32) = ();
            for i in 0..=arr.len() - 1 {
                if (i == 0 || i == 4){
                    continue;
                };
                slice.push(arr[i]);
            }
        }


        assert_eq!([2, 3, 4], nice_slice);
    }
}
// 
fn slice_out_of_array() {
         let a:[u32; 5] = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        fn slice (arr: Array){
            let slice: (u32, 3) = ();
            for i in 0..=arr.len() - 1 {
                if i == 0 || i == 4{
                    continue;
                };
                slice.push(arr[i]);
            }
        }

        let nice_slice = slice(a);
}