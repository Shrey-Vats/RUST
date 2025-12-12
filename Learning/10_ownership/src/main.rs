
fn main() {
    let s = String::from("shrey vats");
    println!("The value is {}", s);
    println!("{}", s);
    let t = s;
    println!("Value:{}", t);
    ownership_taken_give_overview();
}

fn take_ownership(s: String) {
    println!("Ownership of {} has been taken", s);
}

fn give_ownership() -> String {
    String::from("shrey vats")
}

fn ownership_taken_give_overview() {
    let s = String::from("1");
    take_ownership(s);
    // println!("The value of s : {}", s);

    let a = give_ownership();
    println!("Given ownership: {}",a);
}

fn borrowing() {
    let mut s = String::from("The world");

    let r1 = &s;
    let r2 = &s;
    
    println!("the referances: {} and {}", r1, r2);
    println!("again: {}, and {}", r1, r2);
    
    let r3 = &mut s;

    r3.push_str("is bad");

    println!("s value is: {}", s);

    borrowing_modify(&mut s);
    borrowing_simple(&s);
}

fn borrowing_simple(s: &String) {
    println!("What's up brother");
    println!("value: {}", s)
}

fn borrowing_modify(s: &mut String) {
    s.push_str("I will kill myself");
}