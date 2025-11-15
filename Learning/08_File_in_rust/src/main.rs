use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug)]
#[derive(Deserialize)]
struct Todo {
    todo: String,
    id: usize,
    done: bool
}
fn main () {
    let path = Path::new("info.json");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file
    };

    let mut todos = String::new();

    match file.read_to_string(&mut todos) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(_) => println!("{} contains \n{}", display, todos)
    }

    let s: Todo = serde_json::from_str(&todos).expect("file not not there");

    println!("{:?}", s)

    // let mut file = match File::create(path) {
    //     Err(why) => panic!("couldn't create {}: {}", display, why),
    //     Ok(file) => file 
    // };

    // match file.write_all(content.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfuly wrote to {}", display)
    // };
    
}