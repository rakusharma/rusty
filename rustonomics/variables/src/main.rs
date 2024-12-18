fn main() {
    println!("--- scalar types ---");
    let x = 3;
    let mut y = 10;

    println!("{y}");

    y = 17;

    println!("{x} {y}");
    const FIRST_CONST: u32 = 89 * 87;
    println!("{FIRST_CONST}");
    let y: u128 = 0xfffffffffffff - 1;
    println!("{y}");
    let y: u32 = 1;
    let y = y + 1;
    let f: f32 = 1.32;
    let _f: bool = true;
    let _c: char = 'u';

    println!("{y} float {f} bool {_f} char {_c}");

    println!("--- compound types ----");

    let tup: (i32, u32, char) = (5, 4, 'c');
    let (x, y, z) = tup;
    println!("tuple {:?}, {x}:{y}:{z}", tup);
    println!("tup {}:{}:{}", tup.0, tup.1, tup.2);
    let a = [1, 2, 3, 4];
    println!("{:?}", a);
    let a: [u32; 4] = [1, 2, 3, 4];
    println!("{:?}", a);
    let a = [-1; 5];
    println!("{:?}", a);
}
