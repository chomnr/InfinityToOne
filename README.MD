# InfinityToOne

I wrote this simple Collatz conjecture sequencer: sequence any number; I mean any number, even big numbers.

## Sequencer v1 & v2

v1
### Sequence smaller numbers
```rust
 let mut collatz: CollatzSequence = CollatzSequence::new(parse);
 while collatz.seq[collatz.i] != 1 {
    collatz.sequence();
 }
```

v2
### Sequence larger numbers
```rust
let mut collatz_v2: CollatzEnormousSequence = CollatzEnormousSequence::new(parse);
while collatz_v2.seq[collatz_v2.i] != BigUint::from(1 as usize) {
    collatz_v2.sequence();
}
```


## Dependencies

Make sure you have the following dependencies installed:

- [Rust](https://www.rust-lang.org/)
- [num-bigint](https://crates.io/crates/num-bigint) (version 0.4.4)

## Installation

To build and run the project, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/chomnr/InfinityToOne.git
   cd InfinityToOne
   cargo run
   ```