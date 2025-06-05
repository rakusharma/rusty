trait Permutation {
    fn permutations(&self, list: Vec<u32>) -> Vec<Vec<u32>>;
    fn permutations_recursive(&self, list: Vec<u32>, path: Vec<u32>);
}

struct NumberPermutations;

impl Permutation for NumberPermutations {
    fn permutations(&self, list: Vec<u32>) -> Vec<Vec<u32>> {
        let mut stack: Vec<Vec<u32>> = Vec::new();
        let mut path: Vec<Vec<u32>> = vec![];
        /*key is pushing items as vector and pop each and add items to it */

        stack.push(vec![]);

        while !stack.is_empty() {
            if let Some(temp) = stack.pop() {
                if temp.len() == list.len() {
                    path.push(temp.clone());
                    continue;
                }
                for i in list.iter().rev() {
                    if temp.contains(i) {
                        continue;
                    }
                    let mut new_path = if temp.is_empty() {
                        Vec::new()
                    } else {
                        temp.clone()
                    };
                    new_path.push(*i);
                    stack.push(new_path.clone());
                }
            }
        }
        path
    }
    fn permutations_recursive(&self, list: Vec<u32>, path: Vec<u32>) {}
}
fn main() {
    let perms = NumberPermutations;
    let list: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", perms.permutations(list));
}
