# 🦀 Rust Ownership and Borrowing: Full Cheat Sheet

This cheat sheet covers **all major cases of ownership and borrowing in Rust**, with simple examples and key rules.

---

## ✅ 1. Ownership (Move Semantics)

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2, s1 is no longer valid
```

- Moves ownership from one variable to another.
- Only one owner at a time.

---

## ✅ 2. Immutable Borrowing

```rust
let s = String::from("hello");
let len = calculate_length(&s);

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- Uses `&T` to borrow immutably.
- Allows **multiple** borrows.
- Cannot modify the borrowed value.

---

## ✅ 3. Mutable Borrowing

```rust
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}
```

- Uses `&mut T` to borrow mutably.
- **Only one** mutable borrow allowed at a time.

---

## ✅ 4. Borrowing Rules

| Borrow Type       | Can Modify? | Multiple Allowed? |
|-------------------|-------------|--------------------|
| Immutable (`&T`)  | ❌ No       | ✅ Yes             |
| Mutable (`&mut T`)| ✅ Yes      | ❌ No              |

You cannot mix mutable and immutable borrows simultaneously.

---

## ✅ 5. Slices (Special Case of Borrowing)

```rust
let s = String::from("hello");
let slice = &s[0..2]; // slice = "he"
```

- Borrow part of a collection.
- Immutable by default.
- Follows the same borrowing rules.

---

## ✅ 6. Copy Trait

```rust
let x = 5;
let y = x; // x is still valid
```

- Types like integers, booleans, and chars implement `Copy`.
- Copies the value instead of moving.

---

## ✅ 7. Ownership in Functions

```rust
fn take_ownership(s: String) {
    println!("{}", s);
}

let s = String::from("hello");
take_ownership(s); // s is moved and no longer valid here
```

Returning ownership:
```rust
fn give_ownership() -> String {
    String::from("hello")
}

let s = give_ownership(); // s takes ownership of the returned value
```

---

## ✅ 8. Returning and Keeping Ownership

```rust
fn process(s: String) -> String {
    s
}

let s = String::from("hi");
let s = process(s); // ownership moved and returned
```

---

## ✅ 9. Structs with Ownership

```rust
struct Person {
    name: String,
}

let p = Person { name: String::from("Alice") };
let n = p.name; // ownership moved out of p
```

---

## ✅ 10. Structs with Borrowed References

```rust
struct Person<'a> {
    name: &'a str,
}
```

- Use lifetimes (`'a`) to track reference validity in structs.

---

## ✅ 11. Smart Pointers

### Box – Heap Allocation

```rust
let b = Box::new(5);
```

- Single ownership.
- Data stored on the heap.

### Rc – Reference Counting (Shared Ownership)

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a); // shared ownership
```

- Allows multiple owners.
- Immutable by default.

---

## ✅ 12. Closures and Borrowing

Immutable borrow:
```rust
let s = String::from("hello");
let closure = || println!("{}", s);
closure();
```

Mutable borrow:
```rust
let mut s = String::from("hi");
let mut change = || s.push_str(" there");
change();
```

Move ownership:
```rust
let s = String::from("yo");
let closure = move || println!("{}", s); // s is moved into the closure
```

---

## ✅ Summary Table

| Concept            | Can Modify? | Can Have Multiple? | Example Syntax           |
|--------------------|-------------|---------------------|---------------------------|
| Ownership (Move)   | ✅ Yes       | ❌ No               | `let s2 = s1`             |
| Immutable Borrow   | ❌ No        | ✅ Yes              | `&s` or `fn(&T)`          |
| Mutable Borrow     | ✅ Yes       | ❌ No               | `&mut s` or `fn(&mut T)`  |
| Slice              | ❌ No        | ✅ Yes              | `&s[0..2]`                |
| Copy               | ❌ No (copied) | ✅ Yes            | `let y = x`              |
| Rc (Smart Ptr)     | ❌ No        | ✅ Yes              | `Rc::clone(&a)`           |
| Box (Smart Ptr)    | ✅ Yes       | ❌ No               | `Box::new(value)`         |

---

## 📚 Resources

- [Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Book - Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

💡 Use this as a reference while learning or building Rust projects!


# 🦀 Rust Ownership and Borrowing Cheat Sheet

This cheat sheet summarizes the key concepts in Rust's ownership and borrowing system.

| Concept               | Can Modify? | Can Have Multiple? | Example Syntax              |
|-----------------------|-------------|---------------------|-----------------------------|
| **Ownership (Move)**  | ✅ Yes       | ❌ No                | `let s2 = s1`               |
| **Immutable Borrow**  | ❌ No        | ✅ Yes               | `fn(&T)` or `&T`            |
| **Slice**             | ❌ No        | ✅ Yes               | `&s[0..2]`                  |
| **Copy**              | ❌ No *(copied)* | ✅ Yes         | `let y = x`                 |
| **Rc** (Shared Ptr)   | ❌ No        | ✅ Yes               | `Rc::clone(&a)`             |
| **Box** (Heap Owner)  | ✅ Yes       | ❌ No                | `Box::new(value)`           |

---

### 📌 Key Rules

- **Only one mutable reference** allowed at a time.
- **Multiple immutable references** allowed.
- You **cannot mix mutable and immutable** borrows at the same time.
- **Copy types** (`i32`, `bool`, `char`, etc.) are duplicated, not moved.
- **Smart pointers** like `Box` and `Rc` manage heap allocation and reference-counted ownership.

---

### 🧠 Useful for:

- Building safe and efficient systems without garbage collection.
- Preventing data races at compile time.
- Structuring ownership in complex data types like trees, graphs, etc.

---

### 📚 Related Rust Book Chapters:

- [Chapter 4: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Chapter 10.3: Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Chapter 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---
# 🦀 Rust Ownership and Borrowing: Full Cheat Sheet

This cheat sheet covers **all major cases of ownership and borrowing in Rust**, with simple examples and key rules.

---


