/**
 *
 *enums and pattern matching
 *
 */
#[derive(Debug)]
enum days {
    MON,
    TUE,
    WED,
    SUN,
}
#[derive(Debug)]
struct Planner {
    day: days,
    date_type: String,
}

enum Ip_addr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}
//Depending on the enum Message type, call will call on the object. Amazing concept with out too
//much of fuss
impl Message {
    fn call(&self) {
        dbg!(self);
    }
}
#[derive(Debug)]
enum Import {
    USA,
    India,
    Ireland
}
enum Grocery {
    Rice,
    Wheat,
    Pasta,
    Sauce,
    Meat,
    //enum that bind a value
    Beer(Import)
    }



fn main() {

    let day = days::MON;
    let weekend = days::SUN;
    let plan = Planner {
        day: days::MON,
        date_type: String::from("weekend"),
    };

    print_day(&day);
    print_day(&weekend);
    println!("the planner says {:?}", plan);

    let home = Ip_addr::V4(192, 168, 1, 3);
    let loop_back = Ip_addr::V4(127, 0, 0, 1);
    let loop_back_1 = Ip_addr::V6(String::from("::1"));
    //Below enum is amazing. look at implementation
    let m = Message::Write(String::from("example"));
    m.call();
    let m = Message::ChangeColor(10, 10, 10);
    m.call();


    //Rust does not habe NULL. instead it has 'None'
    let n: Option<i32> = None;

    dbg!(n);
    //<T> is generic type parameter
    let some_num = Some(5);
    let some_char= Some('e');
    //this wont compile
    //let i: i32 = 5 + some_num;
    
    //match control flow
    let rice_value = value(Grocery::Rice);
    dbg!(rice_value);
    let beer = value(Grocery::Beer(Import::India));
    dbg!(beer);
    //Option<T>
    let six = Some(6);
    let seven = plus_one(six);
    let null = plus_one(None);
    println!("some {:?}", seven);

    //Catch all pattern and _ placeholder
    let hno = 1;
    match hno {
        1 => one(),
        2 => two(),
        other => unknown()
    }

    let hno = 8;
    match hno {
        1 => one(),
        2 => two(),
        _ => unknown(),
    }

    let hno = 8;
    match hno {
        1 => one(),
        2 => two(),
        _ => (),
    }

    //if let

    let max_cfg = Some(3u8);
    if let Some(max) = max_cfg {
        println!("Maximum is configured as {}", max);
    }

}



fn one() {}
fn two() {}
fn unknown() {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

fn value(item: Grocery) -> f32 {
    match item {
        Grocery::Rice => { 
            println!("Rice will make you fat!");
            3.0
        }
        Grocery::Wheat => 13.0,
        Grocery::Pasta => 30.0,
        Grocery::Sauce => 2.3,
        Grocery::Meat => 3.1,
        //pattern that bind to value
        Grocery::Beer(country) => {
            println!("imported from {:?}", country);
            5.0
        }
    }
}

fn print_day(day: &days) {
    println!("this day is  {:?}", day);
}
