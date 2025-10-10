
// there is three types of struck :- 

//normal struct

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    money: f64
}

//unit struct
struct Unit;

// a tuple struct
struct Pair(i32, f64);

// struct with 2 fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

// struct inside struct
#[derive(Debug)]
struct Rectagnle {
    top: Point,
    bottom: Point
}

fn struct_learning() {
    let name = String::from("Shrey");
    let age: u8 = 22;
    let money = 22948859.2;
    let shrey: Person = Person { name: (name), age: (age), money: (money) };

    println!("Shrey bio is {:?}", shrey);

    let point_top: Point = Point { x: (22.0), y: (32.9) };
    let point_bottom: Point = Point { x: (23.0), y: (39.9) };
    let another_point: Point = Point { x: (21.0), y: (59.9) };

    let rectangle1 : Rectagnle = Rectagnle { top: (point_top), bottom: (point_bottom) };
    println!("rectangle is {:?}", rectangle1);
    // strut update syntax;
    let point_update: Point = Point { x: 22.54 , ..another_point }; 
    println!("new style obj {:?}", point_update);
}

