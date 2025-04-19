use std::cell::RefCell;
use std::rc::Rc;

//TODO: Cell, Weak, structure where elements point to child and root (tree)

#[derive(Debug)]
struct Node {
    parent: Vec<Node>,
    child: Vec<Node>,
}

fn smartptr() {
    /*
     * Illegal code : Below is not allowed in rust due to ownership and borrow rules
    let mut root: Vec<Node> = Vec::new();

    root.push(Node {
        parent: Vec::new(),
        child: Vec::new(),
    });

    //add one node in child

    root.child.push(Node {
        parent: root, <-------We are moving ownership here. RED_FLAG: Not allowed. Also you cannot do parent: &root. borrowing and lifetime rules. This will lead to self referential structs and borrow check and lifetime     issues.Better to move to Rc or RefCell.
        child: Vec::new(),
    });
    */

    dbg!(":?", root);
}
fn main() {
    //On stack
    let y: u32 = 5;
    //On heap & non-mutable reference
    let x = Rc::new(5);
    let u = Rc::clone(&x);
    let v = Rc::clone(&x);
    //below do the ownership tranferring, so not allowed
    //dbg!(x);
    //allowed
    dbg!(&x);
    dbg!(Rc::strong_count(&x));

    //Now lets have a  rc with mutable
    // first new Rc pointer will be created to track the Vec using reference count
    //Read only data immutable
    let x: Rc<Vec<u32>> = Rc::new(vec![1, 2, 0]);
    let read_only = Rc::clone(&x);

    //let's make vector mutable
    let immutable_rc: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    let ex1 = immutable_rc.clone();
    dbg!(&ex1);
    //Mutate the vector
    immutable_rc.borrow_mut().push(4);
    dbg!(&immutable_rc);
    dbg!(&ex1);
    //Hurray below will panic!!!
    let test = immutable_rc.borrow_mut();
    //uncomment below to panic
    //dbg!(immutable_rc.borrow());

    let a = 45;
    let b = 78;
    let max = |a, b| {
        if a > b {
            a
        } else {
            b
        }
    };
    /*wow nested closures*/
    let test_max = |a, b| println!("max : {:?}", max(a, b));

    test_max(a, b);

    let t: Vec<u32> = vec![1, 2, 3, 4];
    //turbofish
    let tt = Vec::<u32>::new();
    /*wtf is this?*/
    //let ttt = Vec<u32>::new();   -> wrong in rusty

    //let nody = Node::new(Node { x: &t, y: &t });
    //dbg!(&nody);
    //
    smartptr();
}
