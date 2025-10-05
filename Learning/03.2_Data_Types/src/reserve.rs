fn reserve(pair: (i32, bool)) -> (bool, i32){
    let (first_value, last_value) = pair;

    (last_value, first_value)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


fn working(){
    
    
    let long_tuple = (233, 88789, 338, "ab", true, false);

    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = (("221", 234, 90), 90, true, "shrey", false, 89, "yt", 98, "ss", 09, 223, true);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);

    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reserve(pair));

    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

}