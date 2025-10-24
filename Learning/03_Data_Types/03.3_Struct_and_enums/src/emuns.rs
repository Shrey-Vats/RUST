
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn enmuns () {
    let player_move = Direction::Down;

    match player_move {
        Direction::Down => println!("Going to toop"),
        Direction::Left => println!("Going to left"),
        Direction::Right => println!("Going to right"),
        Direction::Up => println!("Going to uo")
    }
}

//2. Enums with Data

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: u32, y:u32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn mess() {
    let mess1 = Message::Quit;
    let mess2 = Message::ChangeColor(255, 0, 49);
    let mess3 = Message::Move { x: 134, y: 98 };
    let mess4 = Message::Write(String::from("Hellow"));

    println!("the message 1 to 4, {:?}, {:?}, {:?}, {:?}", mess1, mess2, mess3, mess4);
}

fn process_message(msg: Message) {
    match msg {
        Message::ChangeColor(a,b ,c ) => println!("color is {}, {}, {}", a,b,c),
        Message::Quit => println!("Has been quit"),
        Message::Move { x, y } => println!("The move is x: {}, y:{}", x, y),
        Message::Write(s) => println!("writing {}", s)
    }
}

//3. Enum Methods (Impl Block)

#[derive(Debug)]
 enum Shape {
    Circle(f64),
    Rectagnle(f64, f64)
 }

 impl Shape {
     fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectagnle(length, width ) => length * width
        }
     }
 }

 fn calculate_area () {
    let circle = Shape::Circle(66.0);
    let rectangke = Shape::Rectagnle(33.0, 49.0);

    println!("The area of cicle is {:?}", circle.area());
    println!("The area of rectangle is {:?}", rectangke.area());
 }


 //4. Real-World Concept — Option Enum

 enum Option<T>{
    Some(T),
    None
 }

 fn divide(a: u32, b: u32) -> Option<u32> {
    let after_divide = a / b;

    if after_divide <= 0 {
        None
    } 
 }