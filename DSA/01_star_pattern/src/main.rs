// 1) Star pattens questions ---> loops inside loops;
fn main(){
    loop4(11);
}

// type 1
fn loop1(rows: u8,cols: u8){
    for _ in 1..=rows {
        let mut star = String::from("");
        for _ in 1..=cols {
            star.push_str("*");
        };
        println!("{}", star);
    }
}

fn loop2(rows: u8){
    for i in 0..=rows{
        let mut star = String::from("");
        for _ in 1..i{
            star.push_str("*");
        };
        println!("{}", star);
    };
}

fn loop3(rows: u8){
    for i in (1..=rows).rev() {
        let mut star = String::from("");
        for _ in 1..=i {
            star.push_str("*");
        };
        println!("{}", star);
    };
}

fn loop4(rows: u8){
    for i in 1..=rows {
        // add space
        let mut star = String::from("");
        for _ in 1..=i-1/2 {
            star.push_str(" ");
        };
        for _ in 1..=i {
            star.push_str("*");
        };
        println!("{}", star);
    };
}

/*
9/2 

*/