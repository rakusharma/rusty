/*Find the kth largest element in a number stream
    Design a class to efficiently find the kth  largest element in a stream of numbers. 
    The class should have the following two things
    1.The constructor of the class should accept an integer array (nums) containing initial numbers from the stream and an integer kth. 
    2.The class should expose a function add(int value) which will add the number value to the stream of numbers, and will return the kth largest number in the stream seen so far.
    Example:
    Input: [4, 1, 3, 12, 7, 14], k = 3
    Calling add(6) should return '7'.
    Calling add(13) should return '12'.
    Calling add(4) should still return '12'.
*/

#[derive(Debug)]
struct Heap {
    list: Vec<u32>,
    clone: Vec<u32>,
    k: u32,
}

impl Heap {
    fn new(arr: &[u32], k: u32) ->  Heap {
        let mut c = arr.to_vec();
        c.sort();
        Heap {
        list: arr.to_vec(),
        clone:c,
        k,
        }
    }

    fn add(&mut self, val: u32) -> u32 {
        let mut kth = 0;
        self.list.push(val);
        for (i, &x) in self.clone.iter().enumerate()  {
            if val < x {
                println!(" x {} > val {} @ {}", x, val, i);
                self.clone.insert(i, val);
                break;
            }
        }
        let mut iter = self.k;

        for x in self.clone.iter().rev() {
            if iter == 1 {
                kth = *x as u32;
                break;
            }
            iter-=1;
        }
        kth
    }
}

fn main() {
    println!("kth problem!");

    let arr:[u32; 6] = [4, 1, 3, 12, 7, 14];
    let mut v: Heap = Heap::new(&arr, 3);
    println!("{:? }", v);
    println!("add and return {}", v.add(6));
    println!("{:? }", v);
    println!("add and return {}", v.add(13));
    println!("{:? }", v);
    println!("add and return {}", v.add(4));
    println!("{:? }", v);
}
