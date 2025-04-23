use std::cell::{RefCell, RefMut, Cell};
use std::rc::{Rc, Weak};

//TODO: Cell, Weak, structure where elements point to child and root (tree)

#[derive(Debug)]
struct Node {
    parent: Vec<Node>,
    child: Vec<Node>,
}

//Tree node with parent and child
#[derive(Debug)]
struct Nodev1 {
    x: u32,
    parent: RefCell<Weak<Nodev1>>,
    child: RefCell<Vec<Rc<Nodev1>>>
}

//tree nodes where parent point to child and child point to parent. interesting family :)
fn pointy_tree(){

    let root: Rc<_> =   Rc::new(Nodev1 {
        x: 1,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]), 

    });

    let c1: Rc<_> = Rc::new(Nodev1{
        x: 2,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });

    root.child.borrow_mut().push(Rc::clone(&c1));

    let c2: Rc<_> = Rc::new(Nodev1{
        x: 3,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),

    });

    root.child.borrow_mut().push(Rc::clone(&c2));
    *c1.parent.borrow_mut() = Rc::downgrade(&root);
    *c2.parent.borrow_mut() = Rc::downgrade(&root);

    dbg!(&root.parent.borrow());
    dbg!(&root.child.borrow());

    match(&root){
    tmp => {
        std::eprintln!("[{}:{}:{}] {} = {:#?}",std::file!(),std::line!(),std::column!(),std::stringify!((&root)), &tmp);
        tmp
    }
    };

    let tmp = c1.parent.borrow().upgrade();
    if let Some(parent) = tmp {
        dbg!(&parent);
    } else {
        println!("Child has not parent")
    }
    

}

//weak: mut reference -> interior mutability
fn weak_strong(){

    let strong = Rc::new(5);
    let weak = Rc::downgrade(&strong);
    let again_strong = Weak::upgrade(&weak);

    dbg!(&strong);
    dbg!(&weak);
    dbg!(&again_strong);

    //take a weak reference. upgrade and modify
    //Take RefCell and take Rc of that RefCell
    let strong_1 = Rc::new(RefCell::new(10));
    let weak_1:Weak<RefCell<_>>  =  Rc::downgrade(&strong_1);

    dbg!(&strong_1);
    dbg!(&weak_1);

    if let Some(rc) = weak_1.upgrade() {
        *rc.borrow_mut() += 1;
        println!("updated value {:?}", rc.borrow());
    } else {
        println!("Rc has been dropped");
    }
}
fn smartptr() {
    // * Illegal code : Below is not allowed in rust due to ownership and borrow rules
    // let mut root: Vec<Node> = Vec::new();
    //
    // root.push(Node {
    // parent: Vec::new(),
    // child: Vec::new(),
    // });
    //
    // //add one node in child
    //
    // root.child.push(Node {
    // parent: root, <-------We are moving ownership here. RED_FLAG: Not allowed. Also you cannot do parent: &root. borrowing and lifetime rules. This will lead to self referential structs and borrow check and lifetime     issues.Better to move to Rc or RefCell.
    // child: Vec::new(),
    // });

    let shared_val: Rc<RefCell<_>> = Rc::new(RefCell::new(5));
   // RefMut
    let mut x: RefMut<'_, _> = shared_val.borrow_mut();
    /*vector*/
    let shared_vector: Rc<RefCell<_>> =  Rc::new(RefCell::new(Vec::new()));
    let mut y : RefMut<'_, _> = shared_vector.borrow_mut();

    y.push(100);

    // note the Asterix
    *x = 100;
    dbg!(":?i {:?}", x, y);

    //Cell
    //interesting: var is not explicitly as mut but can be modified
    let var: Cell<_> = Cell::new(5);
    var.set(100);

    dbg!("{:?}", var);
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
    weak_strong();
    pointy_tree();
}