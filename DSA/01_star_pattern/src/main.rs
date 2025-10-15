// 1) Star pattens questions ---> loops inside loops;
fn main() {
}
// type 1
fn star_pattern(rows: u8, cols: u8) {
    for _ in 1..=rows {
        let mut star = String::from("");
        for _ in 1..=cols {
            star.push_str("*");
        }
        println!("{}", star);
    }
}

fn star_pattern2(rows: u8) {
    for i in 0..=rows {
        let star = "*".repeat(i as usize);
        println!("{}", star);
    }
}
fn star_pattern3(rows: u8) {
    for i in (1..=rows).rev() {
        let star = "*".repect(i as usize);
        println!("{}", star);
    }
}

fn star_pattern4(rows: u8) {
    for i in 1..=rows {
        // add space
        let mut star = String::from("");
        for _ in 1..=(i - 1 )/ 2 {
            star.push_str(" ");
        }
        for _ in 1..=i {
            star.push_str("*");
        }
        println!("{}", star);
    }
}

fn star_pattern5(till: u32) {
    for i in (0..=till).rev() {
        let mut str = String::from("");
        for _ in 0..=i {
            str.push_str("*");
        }
        println!("{}", str);
    }
}

fn star_pattern6(till: u32) {
    for i in 1..=till {
        let mut str = String::from("");

        for _ in 1..=till - i {
            str.push_str(" ");
        }
        for _ in 1..=(i * 2) - 1 {
            str.push_str("*");
        }

        println!("{}", str)
    }
}

fn star_pattern7(till: u32) {
    for i in (1..=till).rev() {
        let mut str = String::from("");
        // print space
        for _ in 1..=till - i {
            str.push_str(" ");
        }
        for _ in 1..=(i * 2) - 1 {
            str.push_str("*");
        }
        println!("{}", str);
    }
}

fn star_pattern8(till: u32) {
    for i in 1..=till {
        let mut str = String::from("");
        for _ in 1..=till - i {
            str.push_str(" ");
        }
        for _ in 1..=(i * 2) - 1 {
            str.push_str("*");
        }
        println!("{}", str);
    }
    for a in (1..=till).rev() {
        let mut str2 = String::from("");
        for _ in 1..=till - a {
            str2.push_str(" ");
        }
        for _ in 1..=(a * 2) - 1 {
            str2.push_str("*");
        }
        println!("{}", str2);
    }
}
//
fn star_pattern9(till: u32){
    for i in 1..=till {
        let stars = "*".repeat(i as usize);
        println!("{}", stars)
    }
    for a in (1..=till-1).rev() {
        let stars = "*".repeat(a as usize);
        println!("{}", stars);
    }
}