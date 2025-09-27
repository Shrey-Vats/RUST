fn main() {
    // string
    let name_topper: &str = "shrey vats";
    let name_average = String::from("kING COBRA");

    println!("strings are {name_average} and {name_topper}");

    //number's

    // unsign :- means only positive
    let a: u8 = 1;
    let a1: u16 = 9983;
    let a2: u32 = 939949455; 
    println!("all unsign (positive integer):- {a}, {a1}, {a2}");
    
    // sign :- means nagative + positive
    let b: i8 = 1;
    let b1: i16 = -29833;
    let b2: i32 = -385758837;
    println!("all sign (positive + nagative integer):- {b}, {b1}, {b2}");

    // boolean's
    let age : u8 = 22;
    let gender: &str = "male";

    let is_you_are_real_male = age > 18;
    let is_you_are_female = gender == "female";

    println!("You are {is_you_are_female} and {is_you_are_real_male}");  
}
