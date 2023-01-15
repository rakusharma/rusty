fn main() {
    println!("common programming concepts");
    //variables and mutability
    let x = 10;
    println!("the value of x is {x}");
    //x = 5 will not work due to immutability
    let mut x = 10;
    x = 5;
    println!("mutability example: the value of x is {x}");
    //constants
    const CONST_EXAMPLE:u32 = 60*60;
    //shadowing
    let x = 10;
    let x = x + 1;
    {
        let x = x + 2;
        println!("the value of x inside braces {x}");
    }
    println!("the value of x outside braces {x}");
    let spaces = "  ";
    let spaces = spaces.len();
    //let mut spaces = " ";
    //spaces = spaces.len();
    //Above will lead to error where we are mixing data types. Rust don't allow it!
    
    /*
     * Data types
     * Scalar and Compounds
     */

    /*
     * Scalar : represents single value, four primary type -> integer, floating-point, boolean and
     *          characters. New type as compare to C is 128 bit (i128, u128) and arch (iszie, usize)
     * integer literals Dec 98_22, Hex 0xff, Octal 0o77, Binary 0b1111_0000, Byte(unsigned only, u8) b'A'
     * integer type default to i32
     * In case of integer overflow, rust will panic in debug mode. Im release mode, it will wrap
     * around.
     * floating typoe default shall be f64( on modern cpu it is roughly same speed as f32 but more
     * precision
     */
    //default as f64
    int x = 1.0;
    int x:f32 = 1.1;
    //boolean
    let t = true;
    let t: bool =  false;
    //character
    let c = 'c';
    let c: char = 'Z';

    /*
     * Compound types  - Group multiple values into one type. Rust has two types - Tuples  and
     * arrays
     */
    let tup: (i32, f64, u8) = (500, 1.1, 'c');
    //pattern matching to destructure a tuple
    let (x, y, z) = tup;
    println!("The value  of z is {z}");
    let a = tup.2;
    println!("the value of {a}");
    //arrays: arrays has fixed length in rust
    let a = [1, 2, 3, 4, 5];
    let days = ["mon", "tues", "wed"];
    //type and size of array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //initial value; size of array
    let a = [3; 5];
    let v = a[0];
    //functions
    foo();
    foo_param(10, 'c');
    let y = val(10);
    //expression -> added a semicolon and it will conver to statement
    let y = {
        let x = 3;
        //expression
        y = x + 1
    };
    //control flow
    if y < 10 {
        println!("y is less than 10");
    } else {
        println!("y is gt than 10");
    }
    //rust needs to know at compile time what type variable is. Below is error
    //let y = if t { 7 } else { "seven" };
    //loops
    let mut counter = 0;
    let result = loop {
        counter = counter + 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    //multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    //while loop
    let mut num = 3;
    while num != 0 {
        println!("number {num}");
        num -= 1;
    }
    let a = [10, 20, 30, 40];
    for ele in a {
        println!("value is {ele}");
    }
    //for loop is most used in rust because of saftery and conciseness
    for num in (1..4).rev() {
        println!("reverse -> {num}");
    }
}

fn foo() {
    prinln!("foo calling...");
}
fn foo_param(x: u32, c: char) {
    println!("foo with param {x} {c}");
}

fn val(x: u32) -> u32 {
    x + 1
}

