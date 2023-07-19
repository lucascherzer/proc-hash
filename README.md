# proc-hash

`proc-hash` provides functionality for embedding hashed values at compile time.

## Implemented Hashing Algorithms
- MD5
- BLAKE2b (512 bit)
- BLAKE2s (256 bit)
- SHA-3
- Whirlpool

## Example

```rust
use proc_hash::include_blake512;

assert_eq!(
    include_blake512!("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
    "7eda814eb5fe31a58a639842aebf554b505bb5e65bdcd1052ee035a1227d353c590580b49c453606e268b4a4f0c7862dff5fa748cd4b0e60c1bcd77c92dd7fd8"
);
```
