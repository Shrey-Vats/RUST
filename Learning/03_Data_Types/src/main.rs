fn main() {
    // Variables can be type annotated.
    let logical: bool = true;
    println!("bool {logical}");
    
    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation
    println!("float {a_float}, and integer {an_integer}");
    
    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    println!("float {default_float}, and integer {default_integer}");

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types 
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);

    array_vs_tuple();
}

fn array_vs_tuple() {
    // array
    let arr: [i32; 5] = [1,2,3,4,5];
    println!("array one is := {:?}", arr);

    // array 2 
    let arr2: [&str; 5] = ["shrey", "manu", "koko", "thala", "kampa"];
    println!("other array is ==> {:?}", arr2);

    // array do not able to add custom data
    // let custom_arr = ["shreyvats", 21, true, "vats"];

    let tuple: (&str, bool, i32, &str) = ("shrey", true, 221, "vats");
    println!("this is tuple one {:?}", tuple);

    let tuple2: (i32, &str, bool, f32) = (221, "shrey", true, 33.21);
    println!("this is another tuple {:?}", tuple2);
}
mod reserve;
mod array_Slices;