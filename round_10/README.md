---
#theme: beige
theme: League
---
# Hello Rust 10

Welcome

Trait

(10.2. Traits: Defining Shared Behavior)

---

## Defining Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

```rust
pub trait DoStuff {
    fn do_struff(&mut self) -> Self;
}
```

---

## Implement a Trait for a Type

```rust
pub struct NewsArticle {
  //...
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  //...
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}
```

---

## Call implementation

```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

---

## Call implementation defined in other scope

```rust
use summary::Summary;
use other::MyStruct;

let myStruct: MyStruct = ...
println!("{}", mystruct.summarize());
```

---

## Restrictions

> we can implement a trait on a type only if either the trait or the type is local to our crate.

...

> we can’t implement external traits on external types

---

```rust
# other/src/lib.rs
use summary::Summary;

struct MyStruct{
  //...
}

impl Summary for MyStruct {
  //...
}
```

---

## Default implementation

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

```rust
impl Summary for NewsArticle {}
```

---

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

---

## Trait as arguments

```rust
pub fn notify(item: impl Summary) {...

pub fn notify<T: Summary>(item: T) {...

pub fn notify(item1: impl Summary, item2: impl Summary) {...

pub fn notify<T1: Summary, T2: Summary>(item1: T1, item2: T2) {...

pub fn notify<T: Summary>(item1: T, item2: T) {...

```

---

```rust
pub fn notify(item: impl Summary + Display) { ...

pub fn notify<T: Summary + Display>(item: T) {...

pub fn notify<T>(item: T)
  where T: Summary + Display
{...

```

---

## Returning Trait

```rust
fn returns_summarizable() -> Box<Summary> {...

fn returns_summarizable() -> impl Summary {...

```

---

## Common Traits

Debug, Clone, Eq, Ord, Hash, PartialEq, PartialOrd, PartialHash,

Display, From / Into, FromStr, Iterator,...

Drop, Sync, Send,

---

## Home Work

implements dedup + 1 test

```rust
fn dedup(l: &Vec<i32>) -> Vec<i32> {
  unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup() {
      unimplemented!()
    }
}
```
