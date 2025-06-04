use std::collections::linked_list;

fn permutations(list: Vec<u32>) {
    let mut stack: Vec<Vec<u32>> = Vec::new();
    let mut path: Vec<Vec<u32>> = vec![];

    stack.push(vec![]);

    while !stack.is_empty() {
        if let Some(temp) = stack.pop() {
            if temp.len() == list.len() {
                path.push(temp.clone());
                continue;
            }
            for i in &list {
                if temp.contains(&i) {
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
    println!("{:?}", path);
}
fn main() {
    let list: Vec<u32> = vec![1, 2, 3];
    permutations(list);
}
