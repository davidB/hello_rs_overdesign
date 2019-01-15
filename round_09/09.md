---
#theme: beige
theme: League
---
# Hello Rust 09

Welcome

Box, Rc, Arc, Cell, RefCell

---

## Pointers!

```rust
&T
```

and

```rust
&mut T
```

---

## How do you return a pointer?

```rust
#[derive(Debug)]
struct T;

fn get_my_struct() -> T {
    T
}

fn main() {
    println!("{:?}", get_my_struct());
}
```

---

## How do you return a pointer?

```rust
#[derive(Debug)]
struct T;

fn get_my_struct() -> &T {
    &T
}

fn main() {
    println!("{:?}", get_my_struct());
}
```

---

## You put it in a box!

```rust
#[derive(Debug)]
struct T;

fn get_my_struct() -> Box<T> {
    Box::new(T)
}

fn main() {
    println!("{:?}", get_my_struct());
}
```

---

## Box

Box allocates on the heap

Zero cost, doesn't add anything other than a heap allocation

Removed by the compiler

---

## Knowing the size of your types

The recursive list

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

---

## A few helper traits

```rust
Deref
```

Allow access transparently of what is inside the `Box`

```rust
Drop
```

Control behaviour when the `Box` gets out of scope

---

## Cool! What else?

Reference counted pointers!

```rust
Rc
```

Used when you need several pointers to the same data.

Add a cost: the reference counter

Allow "complex" recursive data structures:
* doubly linked list
* trees where nodes points to parents
* linked list where you still want to access sublist used during construction
* ...

---

## Nice! Any way around mutability?

```rust
Cell
```

Interior mutability!

Zero cost!

But only works for `Copy` types

---

## But my type is not Copy

```rust
RefCell
```

Interior mutability!

Has a cost to keep track if it has been borrowed

---

## What is it used for?

Immutable list / hashmap of mutable objects

Hide mutable behaviour as immutable

Late initialization of an immutable object

Two methods: `borrow` and `borrow_mut`

---

## Composition

You can compose wrapper types:

```rust
Rc<RefCell<T>>
```

Referenced counted with  interior mutability

---

## But... multithreaded?

Checked with two traits:

```rust
Send
```

For types that can be transferred across thread boundaries.

```rust
Sync
```

For types that are safe to share as references between threads.

---

## Traits implementation

`Send` need to be implemented by the user for her types, `Sync` is implemented by default. It means that `&T` is `Send` by default.

You can mark your type as not `Sync` with
```rust
impl !Sync for T 
```

---

## Thread safe `Rc`

```rust
Arc
```

Same as `Rc`, but with an atomic counter. Costs more to ensure atomicity

---

## Thread safe `RefCell`

```rust
Mutex
```

and

```rust
RwLock
```

Both will allow access to the wrapped struct, and will block until access can be obtained.

`RwLock` will allow several read access to the data at the same time
