---
#theme: beige
theme: League
---
# Hello Rust 08

Welcome & Happy new year

ownership

---

## Home Work (check)

update your CLI for edition 2018

---

## A "new" approach

> memory is managed through a system of ownership with a set of rules that the compiler checks at compile time

- no garbage collector
- no allocate & free
- take some time to get used to

---

## Ownership Rules

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

---

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

---

> Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

---

## ? Next ?

Rc, Arc, Cell, Box, ...
