fn main() {
    let words = String::from("Shrey vats is living in the universe where everyone is not like him");
    let len = count_word(&words);
    println!("Number of cherecter: {}", len);
}

fn count_word(value: &String)-> usize {
    let arr: String =  value.split(" ").collect();
    let len = arr.len();
    len
}

fn move_borrow() {
    let name = String::from("shrey vats");

    // wrong because here we already give the name value to consume_string function then name no longer exits
    // consume_string(name);
    // borrow_string(&name);


    borrow_string(&name);
    consume_string(name);
}

// here we consume the string and after it the value in it orginal place no loger exits 
fn consume_string(name: String) {
    println!("consumeing Name is : {name}");

}

// here we borrow a referance not acture value
fn borrow_string(name: &String) {
    println!("borrowing name : {name}")
}

/*
cli based
*/