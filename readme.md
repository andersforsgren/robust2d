# robust2d [![Build Status](https://travis-ci.com/andersforsgren/robust2d.svg?branch=master)](https://travis-ci.com/andersforsgren/robust2d)
Robust geometric predicates based on Jonathan Schewchuk's library https://www.cs.cmu.edu/~quake/robust.html
Provides a simple ffi wrapper for the C library as well as some ergonomic Rust types and functions.

# Example

```rust
let _ = arithmetic::Library::init();  // Must initialize some static constants for the robust arithmetic

// Circle center at (0.0, 1.0) r=1.0
let a = Point::new(-1.0, 1.0);
let b = Point::new(1.0, 1.0);
let c = Point::new(0.0, 2.0);

// Test point
let p2 = Point::new(0.1, 0.5);

println!("{:?}", p2.orient_to_circle(&a, &b, &c)); // "Inside"
```
