fn main() {
    println!("ownership in rust");
    /*
     * Ownership rule:
     * Each value in Rust has an owner.
     * There can only be one owner at a time.
     * When the owner goes out of scope, the value will be dropped.
     *
     */

    //ownership changes if there is a move. 
    let mut s1 = String::from("ownership");
    /*move example*/
    //s1 become invalid and cannot be used further since copy trait is not implemented. s2 point to
    //memory pointed  by s1
    let s2 = s1;
    //println!("s1 is {}", s1); <- Leads to compilation error 
   
    //deep copy
    let s3 = s1.clone();
    println!("s2 {} s3 {}", s2, s3);
    
    /*
     * Rust does allow annotate a type with Copy if that type implements Drop trait
     * types that implement Copy:
     * all integers
     * boolean types
     * all floating types
     * character
     * tuples, if any of the type does not implement drop e.g (u32, u32), invalid (u32, String)
     */

    let s4 = String::from("test");
    take_ownership(s4);
    //s4 is invalid from now onwards
    let x = 10;
    make_copy(x);
    //x is still valid
    
    //return value and scope
    let x1 = give_ownership();

    let x2 = String::from("take");
    //x2 is moved and return value is moved to x3
    let x3 = take_and_give_back(x2);

    //function return multiple values
    let x4 = String::from("multiple return");
    let (x5, len)  = cal_len(x4);
    println!("the lenght of str {} is {}", x5, len);




     
     
}

fn cal_len(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

fn take_ownership(x: String){
    println!("string {}", x);

}

fn make_copy(x: i32) {
    println!("variable {}", x)
}
fn give_ownership() -> String {
    let x = String::from("give");
    x
}
fn take_and_give_back(s: String) -> String {
    s
}



