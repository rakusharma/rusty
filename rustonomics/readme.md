use std::rc::Rc;

let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);
println!("a = {}, b = {}", a, b);
