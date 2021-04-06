Just a repo so I can work through "Programming Rust 2nd Edition"

Notes are for me, so probably won't make any sense to anyone else, sorry!

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