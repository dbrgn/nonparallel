# nonparallel

[![CircleCI][circle-ci-badge]][circle-ci]
[![Crates.io Version][crates-io-badge]][crates-io]
[![Crates.io Downloads][crates-io-download-badge]][crates-io-download]
[![License][license-badge]][license]

A procedural macro for Rust that allows you to ensure that functions are not
running at the same time. This is especially useful for integration tests,
where tests that are writing to the same database table should not run in
parallel.

The goal of mutual exclusion is achieved by acquiring a mutex at the beginning
of the annotated function. So in essence this macro is syntactical sugar for
writing `MUT.lock().unwrap()` at the beginning of every function.

Different functions can synchronize on different mutexes. That's why a
static mutex reference must be passed to the `nonparallel` annotation.


## Usage

```rust
use std::sync::Mutex;
use nonparallel::nonparallel;

// Create two locks
static MUT_A: Mutex<()> = Mutex::new(());
static MUT_B: Mutex<()> = Mutex::new(());

// Mutually exclude parallel runs of functions using those two locks

#[nonparallel(MUT_A.lock().unwrap())]
fn function_a1() {
    // This will not run in parallel to function_a2
}

#[nonparallel(MUT_A.lock().unwrap())]
fn function_a2() {
    // This will not run in parallel to function_a1
}

#[nonparallel(MUT_B.lock().unwrap())]
fn function_b() {
    // This may run in parallel to function_a*
}
```


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

<!-- Badges -->
[circle-ci]: https://circleci.com/gh/dbrgn/nonparallel/tree/master
[circle-ci-badge]: https://circleci.com/gh/dbrgn/nonparallel/tree/master.svg?style=shield
[crates-io]: https://crates.io/crates/nonparallel
[crates-io-badge]: https://img.shields.io/crates/v/nonparallel.svg?maxAge=3600
[crates-io-download]: https://crates.io/crates/nonparallel
[crates-io-download-badge]: https://img.shields.io/crates/d/nonparallel.svg?maxAge=3600
[license]: https://github.com/dbrgn/nonparallel#license
[license-badge]: https://img.shields.io/badge/License-Apache%202.0%20%2f%20MIT-blue.svg
