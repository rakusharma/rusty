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

Feel free to use or expand this cheat sheet in your Rust projects!
