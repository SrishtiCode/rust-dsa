Rust Fundamentals

1. Explain ownership, borrowing, and the borrow checker. Why does Rust not need a garbage collector?
Rust manages memory through ownership, borrowing, and the borrow checker, so it doesn't need a garbage collector.

In Rust, every value has an owner, and there can only be one owner at a time. When the owner goes out of scope, Rust automatically frees the memory. For example, if a String goes out of scope, its heap memory is released automatically.

Borrowing allows us to access a value without taking ownership. We can have multiple immutable references to a value, or one mutable reference, but we cannot have both at the same time. This prevents problems such as data races and invalid memory access.

The borrow checker is a compile-time mechanism that enforces these ownership and borrowing rules. It ensures that references don't outlive the data they point to and prevents things like use-after-free and double-free errors.

Because Rust can determine memory ownership and lifetimes at compile time, it can provide automatic memory management without a runtime garbage collector. This gives Rust both memory safety and predictable performance, which is especially useful for systems programming and performance-sensitive applications.

2. What's the difference between &T, &mut T, and owned T? When would a function take each?
An owned T means the function takes ownership of the value. I use it when the function needs to consume or take responsibility for the value. &T is an immutable borrow, so the function can read the value without taking ownership, and multiple immutable borrows can coexist. &mut T is a mutable borrow, which allows the function to modify the value without taking ownership, but it requires exclusive access. In general, I use &T when I only need to read, &mut T when I need to modify, and T when I need to take ownership.

3. Explain lifetimes. Why do we need explicit lifetime annotations sometimes but not always?   
A lifetime describes how long a reference is guaranteed to remain valid. Rust uses lifetimes to ensure that references never outlive the data they point to, preventing dangling references. We don't always need to write lifetime annotations because Rust can infer lifetimes in common cases using lifetime elision rules. We need explicit lifetime annotations when the relationship between multiple references isn't obvious to the compiler, such as when a function takes multiple references and returns one of them. For example, fn longest<'a>(x: &'a str, y: &'a str) -> &'a str tells Rust that the returned reference is tied to the lifetime of the inputs.

4. Box<T> vs Rc<T> vs Arc<T> vs RefCell<T> — when do you use each, and why does RefCell exist if borrowing is checked at compile time?

5. What is the difference between Copy and Clone?

6. Explain trait objects (dyn Trait) vs generics (impl Trait / <T: Trait>). What's the performance trade-off (static vs dynamic dispatch)?

7. What are associated types in traits, and how do they differ from generic type parameters?

8. Explain Send and Sync. How does Rust's type system enforce thread safety at compile time?

9. What's the difference between unwrap(), expect(), ?, and pattern matching on Result/Option?

10. When and why would you write unsafe code? What guarantees do you lose, and what do you still have to uphold?

11. Explain zero-cost abstractions with an example (e.g., iterators compiling down to loops).

12. What is monomorphization, and how does it affect binary size and compile time for generic-heavy code (relevant for ZK libraries)?

13. How do procedural macros work, and where have you seen them used (e.g., #[derive(...)] in serialization crates)?

ZK Theory Fundamentals

14. What are the three properties a zero-knowledge proof must satisfy: completeness, soundness, and zero-knowledge? Define each precisely.

15. What's the difference between an interactive and non-interactive proof? What role does the Fiat-Shamir heuristic play?

16. What is a zk-SNARK, and what does each letter stand for?

17. Compare zk-SNARKs and zk-STARKs: trusted setup, proof size, verification time, post-quantum security.

18. What is a trusted setup / "toxic waste," and why is it a concern? What's the difference between a circuit-specific and a universal (updatable) setup?

19. Explain arithmetic circuits and R1CS (Rank-1 Constraint Systems). How does a computation get "compiled" into constraints?

20. What is a witness in a ZK proof system?

21. What's the difference between proving knowledge of a value vs. proving a statement about a value?
