
//API
fn b_sort(v: &mut Vec<u32>) {

    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }

}



fn main() {
    let mut v: Vec<u32> = vec![99,3,2,5,33,89,4];
    b_sort(&mut v);
/*
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }

            //let a = v[i];
            //println!("vector {a}"); 
        }
    }
  */

    println!("vector {:?}", v); 

}
