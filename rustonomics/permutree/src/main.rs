// Problem: Generate all permutations of a set of numbers.

// Unconventional approach: Convert the sequence of numbers into an n-ary tree.
// Then perform a depth-first traversal from root to leaf, recording each path as a permutation.

// Implementation Details:
// - Depth-First Search (DFS) is used to traverse from root to leaves.
// - Each complete path from root to leaf is stored in a vector of vectors.
// - Smart pointers and interior mutability are used to manage tree structure and shared ownership.

// Tree Node Structure:
// - `parent: RefCell<Weak<PermNode>>`
//   Each node holds a weak reference to its parent, wrapped in a `RefCell` to allow mutation even when shared via `Rc`.
//   `Weak` is used to avoid reference cycles and memory leaks (does not increase ref count).

// - `child: RefCell<Vec<Rc<PermNode>>>`
//   Each node owns a list of children via `Rc` (shared ownership), allowing multiple references to child nodes if needed.
//   `RefCell` provides interior mutability so the child list can be modified at runtime (e.g., push/pop).

// Smart Pointer Usage:
// - `Rc` (Reference Counted): Enables multiple owners of a node, used for children.
// - `Weak`: Non-owning pointer, used for parent to prevent circular references.
// - `RefCell`: Allows mutable access to fields even when the node is shared via `Rc`.

// TL;DR:
// - `parent: RefCell<Weak<PermNode>>`: Weak reference to parent, mutable even under shared ownership.
// - `child: RefCell<Vec<Rc<PermNode>>>`: List of owned children, with interior mutability for updates.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::format;
use std::rc::{Rc, Weak};
#[derive(Debug)]
#[allow(dead_code)]
pub struct PermNode {
    x: u32,
    parent: RefCell<Weak<PermNode>>,
    child: RefCell<Vec<Rc<PermNode>>>,
    child_list: Vec<u32>,
}
#[derive(Debug, Clone)]
pub struct Stack {
    l: Rc<PermNode>,
}
impl PermNode {
    fn permutation(self: &Rc<Self>) {
        let mut stack: Vec<Stack> = Vec::new();
        let stack_item = Stack { l: Rc::clone(self) };
        let mut paths: Vec<Vec<u32>> = Vec::new();
        let mut res: Vec<Vec<u32>> = Vec::new();

        stack.push(stack_item.clone());
        paths.push(vec![stack_item.l.x]);

        while stack.len() != 0 {
            //
            if let Some(stack_item) = stack.pop() {
                if let Some(mut path) = paths.pop() {
                    if path.len() == 1 && path[0] == 0 {
                        path.clear();
                    }
                    if stack_item.l.child_list.is_empty() {
                        res.push(path.clone());
                    } else {
                        let pn = &stack_item.l.child;
                        for c in pn.borrow().iter() {
                            let mut temp = path.clone();
                            stack.push(Stack { l: Rc::clone(c) });
                            temp.push(c.clone().x);
                            paths.push(temp);
                        }
                    }
                }
            }
        }
        let string = res
            .iter()
            .map(|inner| {
                let inner_str = inner
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("[{}]", inner_str)
            })
            .collect::<Vec<_>>()
            .join(", ");
        let pnumber = format!("[{}]", string);
        println!("{} {} ", pnumber, res.len());
    }
    fn generate_root(self: &Rc<Self>) {
        let mut q: VecDeque<Rc<PermNode>> = VecDeque::new();
        //add items as child from list to root
        for &child_id in &self.child_list {
            let node = Rc::new(PermNode {
                x: child_id,
                parent: RefCell::new(Weak::new()),
                child: RefCell::new(vec![]),
                child_list: self
                    .child_list
                    .iter()
                    .cloned()
                    .filter(|x| *x != child_id)
                    .collect(),
            });
            *node.parent.borrow_mut() = Rc::downgrade(self);
            self.child.borrow_mut().push(Rc::clone(&node));
            q.push_back(Rc::clone(&node));
        }

        //Add all possible child nodes
        while q.len() != 0 {
            if let Some(node) = q.pop_front() {
                //
                for &k in &node.child_list {
                    let n: Rc<PermNode> = Rc::new(PermNode {
                        x: k,
                        parent: RefCell::new(Weak::new()),
                        child: RefCell::new(vec![]),
                        child_list: node
                            .child_list
                            .iter()
                            .cloned()
                            .filter(|x| *x != k)
                            .collect(),
                    });
                    *n.parent.borrow_mut() = Rc::downgrade(&node);
                    node.child.borrow_mut().push(Rc::clone(&n));
                    q.push_back(Rc::clone(&n));
                }
            }
        }
    }
}

fn main() {
    let list: Vec<_> = vec![1, 2, 3, 4, 5];
    let root: Rc<_> = Rc::new(PermNode {
        x: 0,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
        child_list: list.clone(),
    });
    root.generate_root();
    root.permutation();
}
