use std::io;
fn main() {
    // //imutable
    // let a = "shrey";
    // let b = "rahul";
    // let c = "aaman";
    // let d = "sahil";
    // let e = "prateek";

    // println!("Hello, world! {}, {}, {}, {}, {}", a, b, c, d, e);

    // //mutable
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // // constant
    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    // constants();

    // shadowing();

    shadowing_vs_let();
}

/*

fn constants(){
    const THREE_IN_HOUR: u32 = 60 * 60 * 3;

    println!("Constansts {THREE_IN_HOUR}");
}

fn shadowing(){

    let x = 3;
    let x = x + 1;
    {
        let x = x * 2;

        println!("The value of x after multiplying by 2 :- {x}");
    };
    println!("The value of x is:- {x}");
}
*/

fn shadowing_vs_let(){
    let mut space = String::from("Shrey");

    io::stdin().read_line(&mut space).expect("Error");

    let space = space.len();

    println!("{space}, the value setup complete");
}