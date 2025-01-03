use std::env;

fn chaldean_lookupv1(v: &Vec<Vec<char>>, x: char) -> u32 {

    let mut idx: u32 = 0;

    for (v, i) in v.iter().enumerate() {
        let b = i.contains(&x);
        match b {
            /*true => { let r = i.binary_search(&x); match r {Ok(index) => {/*dbg!("matched {}", index);*/idx = v as u32; break}, Err(insert_pos) => println!("error! not found")}  },*/ 
            true => { idx = v as u32}, 
            _ => {}, //println!("next vector please {}", v),    
        }
    }
    idx
}

fn chaldean_lookup(v: &Vec<Vec<char>>, x: char) -> u32 {

    let mut idx: u32 = 0;

    for (v, i) in v.iter().enumerate() {
        for j in i {
            //println!("lookup for {} {} {} {:?}", x, j, v, i);
            match j {
                val if *val == x => {println!("matched"); idx = v as u32; break},
                _ => println!("not in this vector {} {}", v, x),
            }
        }
    }
    idx
}
fn chaldean_sum(v: &Vec<Vec<char>>, name: &String) -> u32 {

    let mut tot = 0;
    let mut x: u32 = 0;

    for x in name.chars() {
    let idx = chaldean_lookupv1(&v, x);
    //println!("chaldean look up for {} {}", x, idx);
    tot += idx;
    }

    let digits: Vec<String> = tot.to_string().chars().map(|c| c.to_string()).collect();
    for d in digits {
        match d.parse::<u32>() {
            Ok(a) => x += a,
            Err(e) => {},
        }
    }
    return x;
}

fn main() {

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let mut chaldean  = vec![vec!['_'],
                         vec!['a', 'q', 'y', 'i', 'j'],
                         vec!['b', 'r', 'k'],
                         vec!['g', 'c', 'l', 's'],
                         vec!['d', 'm', 't'],
                         vec!['e', 'h', 'n', 'x'],
                         vec!['u', 'v', 'w'],
                         vec!['o', 'z'],
                         vec!['f', 'p'],
                        ];
    println!("welcome to chaldean's world of numbers!");

    //chaldean.sort();

    for (v,i) in chaldean.iter_mut().enumerate() {
        i.sort();
            print!("{:?} -> ", v);
        for j in i {
            print!(" {:?} ", j);
        }
        println!("");
    }

    let _chaldean  = vec!["",
                         "aqyij",
    ];
    print!(" {:?} ", _chaldean);

    let test = String::from("poomale");
    let name = &args[1];
    println!("chaldean totla for name {:?} => {}",&name, chaldean_sum(&chaldean, &name));

}
