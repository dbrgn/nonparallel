# nonparallel

A procedural macro for Rust that allows you to ensure that functions (e.g.
unit tests) are not running at the same time.

This is achieved by acquiring a mutex at the beginning of the annotated
function.

Different functions can synchronize on different mutexes. That's why a
static mutex reference must be passed to the `nonparallel` annotation.

## Usage

```rust
use std::sync::Mutex;
use lazy_static::lazy_static;
use nonparallel::nonparallel;

// Create two locks
lazy_static! { static ref MUT_A: Mutex<()> = Mutex::new(()); }
lazy_static! { static ref MUT_B: Mutex<()> = Mutex::new(()); }

// Mutually exclude parallel runs of functions using those two locks

#[nonparallel(MUT_A)]
fn function_a1() {
    // This will not run in parallel to function_a2
}

#[nonparallel(MUT_A)]
fn function_a2() {
    // This will not run in parallel to function_a1
}

#[nonparallel(MUT_B)]
fn function_b() {
    // This may run in parallel to function_a*
}
```
