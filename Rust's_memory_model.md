1. The basic memory areas

A Rust program primarily uses:

┌──────────────────────────────┐
│          STACK               │
│ local variables              │
│ function arguments           │
│ fixed-size values            │
│ pointers / references        │
├──────────────────────────────┤
│          HEAP                │
│ dynamically allocated data   │
│ String contents              │
│ Vec contents                 │
│ Box<T> contents              │
├──────────────────────────────┤
│     STATIC / DATA            │
│ constants, static values     │
└──────────────────────────────┘

The important thing is that Rust does not automatically put every variable on the heap.

2. Simple integers → usually stack
let x = 10;
let y = 20;

These are fixed-size values.

Conceptually:

Stack
┌───────────┐
│ x = 10    │
├───────────┤
│ y = 20    │
└───────────┘

The compiler knows their size at compile time.

When the function ends, the stack frame disappears.

No garbage collection is necessary.

3. Arrays → usually stack

Consider:

let arr = [10, 20, 30, 40, 50];

An array has a fixed size.

Conceptually:

Stack


arr
┌────┬────┬────┬────┬────┐
│ 10 │ 20 │ 30 │ 40 │ 50 │
└────┴────┴────┴────┴────┘

The array itself contains all five integers.

You can have:

let arr: [i32; 5] = [1, 2, 3, 4, 5];

The compiler knows:

5 × 4 bytes = 20 bytes

So it can reserve the appropriate amount of stack space.

But arrays can also be on the heap

For example:

let arr = Box::new([1, 2, 3, 4, 5]);

Now the array's data is heap allocated.

Stack                  Heap


arr ────────────────▶ [1, 2, 3, 4, 5]
(pointer)

So the type doesn't always determine stack vs heap. How you allocate/use the value matters.
