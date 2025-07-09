# Base4

A simple and portable rust library for representing and manipulating large lists of base-4 integers packed
into 128-bit blocks of bit-ints. It provides two main types:

- **Base4** : A fixed size buffer which can pack upto 64 base-4 integers into 128-bit block.
- **Base4Int**: A dynamic sized buffer which can recursively store as many base-4 integers allowing arbitrary length
                base-4 numbers.

## Installation

```toml
[dependencies]
base4 = { version = "0.1.0" }
```

Then start using it in your project:

```rust
use base4::Base4Int;

fn main() {
    let big_int = Base4Int::new();
    let large_base4_list = vec![2;100];

    debug_assert!(big_int.push_all(&large_base4_list));
}
```

### License

This crate can be freely distributed under both licenses MIT or Apache-2.0.