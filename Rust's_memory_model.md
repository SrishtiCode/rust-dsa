1. The basic memory areas

A Rust program primarily uses:

```
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
```

The important thing is that Rust does not automatically put every variable on the heap.

2. Simple integers → usually stack
let x = 10;
let y = 20;

These are fixed-size values.

Conceptually:

```
Stack
┌───────────┐
│ x = 10    │
├───────────┤
│ y = 20    │
└───────────┘
```

The compiler knows their size at compile time.

When the function ends, the stack frame disappears.

No garbage collection is necessary.

3. Arrays → usually stack

Consider:

let arr = [10, 20, 30, 40, 50];

An array has a fixed size.

Conceptually:

Stack

```
arr
┌────┬────┬────┬────┬────┐
│ 10 │ 20 │ 30 │ 40 │ 50 │
└────┴────┴────┴────┴────┘
```

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

```
Stack                  Heap


arr ────────────────▶ [1, 2, 3, 4, 5]
(pointer)
```

So the type doesn't always determine stack vs heap. How you allocate/use the value matters.

4. String is different

Now look at:

let s1 = String::from("hello");

A String is a growable string.

Its contents are stored on the heap.

But the String variable itself is a small structure stored on the stack.

Conceptually:

```
Stack                         Heap

s1
┌──────────────┐
│ ptr ────────────────┐
│ len = 5      │      │
│ capacity = 5 │      │
└──────────────┘      │
                      ▼
                ┌─────────────┐
                │ h e l l o   │
                └─────────────┘
```
A String is conceptually similar to:
struct String {
    ptr: *mut u8,
    length: usize,
    capacity: usize,
}
So:
String object → Stack
String contents → Heap

5. Why does String need the heap?

Because it can grow.

let mut s = String::from("hello");


s.push_str(" world");

Initially:

Heap:


[h][e][l][l][o]

Then it might allocate a larger region:

Heap:


[h][e][l][l][o][ ][w][o][r][l][d]

The compiler doesn't necessarily know the final size at compile time.

Therefore dynamic allocation is required.

6. &str is different from String

This distinction is extremely important.

let s1 = String::from("hello");
let s2 = "hello";

s1 is a String.

s2 is a string slice (&str).

A string literal such as:

"hello"

is generally stored in the program's static/read-only data area.

Conceptually:
```
Static memory
┌─────────────┐
│ "hello"     │
└─────────────┘
       ▲
       │
Stack  │
┌──────┴──────┐
│ s2 (&str)   │
│ pointer     │
│ length = 5  │
└─────────────┘
```

So:
```
String
  ├── String object → stack
  └── characters    → heap

&str literal
  ├── reference     → stack
  └── characters    → static memory
```
