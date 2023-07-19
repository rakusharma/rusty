//#[derive(Debug)]
fn main() {

    let mut v: Vec<u32> = Vec::new();
    v.push(9);
    v.push(10);
    println!("vector {:?}", v);

    let mut v_1 = vec![1, 2, 3];
    v_1.push(0);
    println!("another vector {:?}", v_1);
    let e = &v_1[0];
    println!("accessing element@index -> {e}");
    let _e: Option<&u32> = v_1.get(0);
    match _e {
        Some(_e) => println!("match the element@index is {_e}"),
        None => println!("error! index does not exist"),
    }
}
