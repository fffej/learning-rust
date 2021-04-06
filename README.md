Just a repo so I can work through "Programming Rust 2nd Edition"

Notes are for me, so probably won't make any sense to anyone else, sorry! (Chapter 5. References)

Rust aims to be safe and performant by restricting how you use pointers. Ownership is built into the language and enforced by compile-time checks. Deterministic finalization without having to remember to do it (like in C++)! Variables own values. 

You can transfer ownership

```
  let s = vec!["a".to_string(), "b".to_string()];
  let t = s; // this is a move, s is illegal now
  let u = s; // compile time problem
```

If you want to do stuff like the above use `.clone()`.

```
  let mut a = "banana".to_string();
  b = "apple".to_string(); // banana dropped
```


* `i8` through `i128` (and corresponding `u`) for ints of given width.
* `f32` and `f64` for floating points
* `bool`, `char`, `struct` as you'd expect
* `Option<&>`, `Result<T,Error>`
* `| a, b, c |` (closure)
* `("banana", 33)` - tuple of type `(&str, i32)`
* `fn swap<T>(x: &mut T, y: &mut T);` - generic swap function on type T (return type omitted since unit)
* `&T` is an immutable reference to a T (like a pointer, `*` dereferences)
* `&mut T` is a mutable EXCLUSIVE reference.
* Allocate on the head with `Box::new(t)`
* `type` is like `typedef`

