---
#theme: beige
theme: League
---
# Hello Rust 04

Welcome

(checkpoint 01)

---

## Errata

  panic & abort

[std::panic::catch_unwind - Rust](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)

[Unwinding](https://doc.rust-lang.org/nomicon/unwinding.html#unwinding)

---

Rust has a tiered error-handling scheme:

```txt
If something might reasonably be absent, Option is used.
If something goes wrong and can reasonably be handled, Result is used.
If something goes wrong and cannot reasonably be handled, the thread panics.
If something catastrophic happens, the program aborts.
```

---

## Addentum

[Rustlog : Why is a Rust executable large?](https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html)

[FAQ - Why do Rust programs have larger binary sizes than C programs?](https://www.rust-lang.org/en-US/faq.html#why-do-rust-programs-have-larger-binary-sizes-than-C-programs)

---

## Home Work (check)

review what ?

---

## Q&A

[Frequently Asked Questions · The Rust Programming Language](https://www.rust-lang.org/en-US/faq.html)

---

## Home Work

suggestion: take a look to the F.A.Q

---

## ? Next (basic) ?

errors, test, enum, trait, iterator, string & str, owership/borrowing, log, ...

review
