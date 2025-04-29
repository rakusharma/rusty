use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
#[derive(Debug)]
#[allow(dead_code)]
pub struct PermNode {
    x: u32,
    parent: RefCell<Weak<PermNode>>,
    child: RefCell<Vec<Rc<PermNode>>>,
    child_list: Vec<u32>,
}

impl PermNode {
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

        dbg!(&self);
    }
}

fn main() {
    let list: Vec<_> = vec![1, 2];
    let root: Rc<_> = Rc::new(PermNode {
        x: 0,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
        child_list: list.clone(),
    });
    root.generate_root();
}
