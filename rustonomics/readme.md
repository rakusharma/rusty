# 🧠 Rust Smart Pointers Cheat Sheet

Smart pointers in Rust are types that not only act like a pointer but also have additional metadata and capabilities. Here's a breakdown of the most common types.

---

## 📦 1. `Box<T>` – Heap Allocation

### Use:
- Store data on the heap.
- Useful for recursive data types like linked lists or trees.

### Example:
let b = Box::new(5);
println!("b = {}", b);


| Feature           | `Cell<T>`                        | `RefCell<T>`                     |
|------------------|----------------------------------|----------------------------------|
| Type requirement | `T: Copy` (usually)              | Any type                         |
| Access style     | `.get()` / `.set()`              | `.borrow()` / `.borrow_mut()`   |
| Borrow rules     | Enforced at compile time         | Enforced at runtime              |
| Overhead         | Zero runtime checks              | Some overhead + panics on misuse|
