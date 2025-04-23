Standard wrapping:

Rc<RefCell<T>>

 - Rc lets you share the data. 
 - RefCell lets you mutate the shared data
   at runtime.

But, you can also have following

    struct Node {
        value: i32,
        parent: RefCell<Weak<Nodes>>,
        children: RefCell<Vec<Rc<Node>>>, 
        }

This pattern avoids **memory leaks** due to **reference cycles**:
-   Children own parents weakly.
-   Parents own children strongly.
### TL;DR Decision Tree
-   Need shared ownership? → `Rc`
-   Need interior mutability? → `RefCell`
-   Need shared + mutable? → `Rc<RefCell<T>>`
-   Want non-owning links (e.g., parent pointer)? → `Weak`
-   Avoid cycles? → `Weak` to break `Rc` chains

**Q: Generally Rc<RefCell<T>>. so why Refcell<Vec<Rc<Node>>**

    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,           // interior mutable weak ref
        children: RefCell<Vec<Rc<Node>>>,      // interior mutable vec of strong refs
    }

### What’s happening?
-   The whole `Node` is wrapped in an `Rc<Node>` when used.
-   Inside the `Node`, the `children` field is a `RefCell<Vec<Rc<Node>>>`.

> So the outer `Rc<Node>` allows shared ownership of a node, and the `RefCell` allows _interior mutability_ of the `children` list.

## Why not `Rc<RefCell<Vec<Node>>>`?
-   Because we want each **individual node** in the `Vec` to also be shared.
-   If we did `Vec<Node>`, you couldn’t share children with others or keep track of their references.
-   So each child must be an `Rc<Node>`.

Thus:
-   The **`Vec<Rc<Node>>`** means “a list of shared child nodes.”
-   The **`RefCell<...>`** means “I can mutate the list (add/remove children) even if `Node` is not mut.”
> So the field is: “a mutable list of shared child nodes”


----------
## 🧠 So how does this fit with `Rc<RefCell<T>>`?

You're absolutely right — the _general pattern_ is `Rc<RefCell<T>>`, meaning:
-   You share ownership (`Rc`)
-   You want to mutate the data (`RefCell`)

But when you're _inside_ a shared structure like a tree or graph, you often:
-   Put the whole `Node` in `Rc`
-   And put _interior mutability_ (like `children`) in `RefCell`

**Because `Rc` doesn't let you mutate — it only gives shared access.**