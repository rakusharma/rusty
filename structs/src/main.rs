struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_cnt: u64,
    }

//Note the below directive. It will let {:?} work
#[derive(Debug)]    
struct Rect {
    x: u32,
    y: u32,
}
 
/*
 *defining Structs, Tuple structs, Unit-like Structs (without any field), Method
 *
 *
 */

fn main() {

    /*
     * struct
     * By default immutable
     * Rust does not any selective fields to be mutable.
     * Entire structure need to be mutable
     */

   let user1 = User {
        email: String::from("someone@foo.com"),
        username: String::from("faa"),
        active: true,
        sign_in_cnt: 1,
    };
    let mut user2 = User {
        email: String::from("everyone@spam.com"),
        username: String::from("spammer"),
        active: true,
        sign_in_cnt:  99999,
    };

    /*shorthand for copy data from user1 except email address*/
    let user3 = User {
        email: String::from("user3@email.com"),
        username: String::from("copier"),
        ..user1

    };

    user2.sign_in_cnt += 111;

    let user3 = return_struct(String::from("info@spam.com"), String::from("lost"));
    let user4 = return_struct_shorthand(String::from("why@spam.com"), String::from("questo"));
    println!("user4 {} {}", user4.email, user4.username);

    //Tuple structs
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("color tuple {}", black.0);

    //Unit-like structs without any field
    //
    struct Nomembers;

    let orphan = Nomembers;

    //example
    //Calcualte area of rectange
    let r = Rect {
        x: 10,
        y: 10
    };

    println!("dimension of rect using #debug directive {:?} {:#?}", r, r);
    //use reference if you want to use r 
    let area = cal_area_reference(&r);

    println!("area of rectange {}", area);
    //another debug macro
    dbg!(&r);

    //ownership changes
    let area = cal_area(r);
    //Below will have error since r moved
    //println!("dimensions {} {}", r.x, r.y);



    println!("area of rectange {}", area);

    //another example of dbg
    let scale = 1;
    let rr = Rect {
        x: dbg!(30 * scale),
        y: 20,
    };

    dbg!(&rr);

    //Method 
    //Methods are defined under the context of struct
    #[derive(Debug)]
    struct Circle {
        rad: u32,
    }

    impl Circle {
        //Note the difference between 'ordinary' function and method 
        fn area(&self) -> u32 {
            self.rad * self.rad
        }
        fn rad(&self) -> u32 {
            self.rad
        }
        fn is_same(&self, other: &Circle) -> bool {
            self.rad == other.rad
        }
    }

    let c = Circle {
        rad: 100,
    };

    let d = Circle {
        rad: 120,
    };

    dbg!(&c);
    dbg!(&d);
    
    println!("area of circle {}", c.area());
    println!("width of circle {}", c.rad());
    println!("is it same?  {}", c.is_same(&d));

    //Associate functions
    //methods which as often used as constructors
    //
    #[derive(Debug)]
    struct Rectangle {
        w: u32,
        h: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                w: size,
                h: size,

            }
        }
    }

    //Calling associate function 
    let y = Rectangle::square(3); 

    dbg!(&y);







}

fn cal_area(r: Rect) -> u32 {
    r.x * r.y
}

fn cal_area_reference(r: &Rect) -> u32 {
    r.x * r.y
}

fn return_struct(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_cnt: 1,
    }
}
fn return_struct_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_cnt: 1,
    }
}
