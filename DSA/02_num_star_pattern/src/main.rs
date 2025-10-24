fn main() {
    // num_star_pattern(10);
    // num_star_oattern2(10);
    // num_star_pattern3(10);
    // star_pattern2(5);
    num_star_pattern10(5);
   
}

fn num_star_pattern(till: u32){
    for i in 1..=till {
        let mut str = String::from("");

        for a in 1..i + 1{
            str.push_str(&a.to_string());
        }

        println!("{}", str);
    }
}
fn num_star_oattern2(till: u32){
    for i in 0..=till {
        let mut arr = String::from("");
        for _ in 0..i {
            arr.push_str(&i.to_string());
        }
        println!("{}", arr)
    }
}

fn num_star_pattern3(till: u32){
    for i in (1..=till).rev() {
        let mut str = String::from("");
        for a in 1..=i {
            str.push_str(&a.to_string());
        }
        println!("{}", str)
    }
}

fn num_star_pattern4(till: u32){
    for i in 1..=till {
        let mut num = String::from("");
        let mut start = if i % 2 == 0 {1} else {0};

        for _ in 1..=i {
            start = match start {
                1 => 0,
                _ => 1
            };

            num.push_str(&start.to_string());
        }
        println!("{}", num)
    }
}

fn num_star_pattern5(till: u32){
    for i in 1..=till {
        let mut str = String::new();
        for a in 1..=i {
            str.push_str(&a.to_string());
        }
        for _ in 1..=(till - i) * 2 {
            str.push_str(" ");
        }
        for b in (1..=i).rev() {
            str.push_str(&b.to_string());
        }
        println!("{}", str)
    }
}

fn num_star_pattern6(till: u32){
    let mut num = 0;
    for i in 1..=till {
        let mut pattern = String::new();
        for _ in 1..=i {
            num = num + 1;
            pattern.push_str(&num.to_string());
            pattern.push_str(" ");
        };
        println!("{}", pattern);
    }
}

fn num_star_pattern7(till: u32){
    for i in 1..=till {
        let mut variable: char = 'A';
        let mut str = String::new();

        for _ in 1..=i {
            str.push_str(&variable.to_string());
            variable = ((variable as u8) + 1) as char;
        }
        println!("{}", str)
    }
}

fn num_star_pattern8(till: u32){
    for i in (1..=till).rev() {
        let mut variable: char = 'A';
        let mut str = String::new();

        for _ in 1..=i {
            str.push_str(&variable.to_string());
            variable = ((variable as u8) + 1) as char;
        }

        println!("{}", str);
    }
}

fn num_star_pattern9(till: u32){

    let mut variable: char = 'A';
    for i in 1..=till{
        let mut str = String::new();

        for _ in 0..i {
            str.push_str(&variable.to_string());
        }
        println!("{}", str);
        variable= ((variable as u8) + 1) as char;
    }
}

fn num_star_pattern10(till: u32){
    for i in 1..=till {
        let mut str = String::new();
        let mut alphabet = 'A';
        for _ in 1..=till-i {
            str.push_str(" ");
        }
        for _ in 1..=i {
            str.push_str(&alphabet.to_string());
            alphabet = ((alphabet as u8 ) + 1 ) as char;
        };
         alphabet = ((alphabet as u8) - 1) as char;
        for _ in 1..=i -1  {
            alphabet = ((alphabet as u8) - 1) as char;
            str.push_str(&alphabet.to_string());
        }
        println!("{}", str)
    }
}


