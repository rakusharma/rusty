/* two function code duplication*/
fn gt_i32(list : &[i32]) -> &i32 {

    let mut num = &list[0];

    for number in list {
        if number > num {
            num  = number;
        }
    }

    num
}

fn gt_char(list : &[char]) -> &char {

    let mut num = &list[0];

    for number in list {
        if number > num {
            num  = number;
        }
    }

    num
}


/*trait*/

fn gt<T: std::cmp::PartialOrd>(list: &[T]) ->&T {
let mut num = &list[0];

    for number in list {
        if number > num {
            num  = number;
        }
    }

    return num;

}

fn main() {
    println!("trait example TraitImpl<T>");

    /* Generic way to implement larget number treasure hunting*/
    let list = vec![10, 11, 12, 14, 89, 1];
    let mut num = &list[0];
    dbg!(num);
    //println!("{:?}", type_name::<typeof(num)>());

    for number in &list  {
        if number > num {
            num =  number;
        }

    }
    let list = vec![10, 11, 12, 14, 89, 1];
    let num = gt_i32(&list);
    println!("the largest {num}");
   
    let num = gt(&list);
    println!("the largest using trait {num}");
 
    let list = vec!['a', 'c', 'o', 'p'];
    let num = gt_char(&list);
    println!("the largest char {num}");

    let list = vec!['a', 'c', 'o', 'p'];
    let num = gt(&list);
    println!("the largest using trait char {num}");
 
   
}


