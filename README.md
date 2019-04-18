# 🦀 benchme 🐢

A simple rust library to quickly benchmark your code blocks.

> Check it out on [crates.io](https://crates.io/crates/benchme)

[![benchme on crates.io](https://img.shields.io/crates/v/benchme.svg)](https://crates.io/crates/benchme)

## 🏎 usage 🚀

```rust
//run a quick benchmark
benchmark! {
    //your super code goes here
    println!("omaewamou SHINDEIRUUUUUU! {}", 999999999);
    println!("NANI!?!?!?!?");
}

//or run a benchmark tagged with a name #[name_goes_here_AbCd123] (useful if you are benchmarking more than one thing)
benchmarknamed! {
    //name that will be displayed for the benchmark output
    #[weeeeeeee]
    //my super intense code that I need to make sure is super mega fast
    println!("To the moooooooon! 🚀");
    println!("the yeet was yote.");
}
```

## ✳️ acquiring benchme ✅

add the following in your `Cargo.toml` file:

`benchme = "0.1.0"`

and this in your super intensive `needsabenchmark.rs` file:
``` rust
#[macro_use]
extern crate benchme;
use std::time::{Instant};
```