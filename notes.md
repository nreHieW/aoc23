#### Day 1
- Try to use more iterators and less loops. Seems to be a common thing in Rust

#### Day 2
Structs in Rust:
```rust
// 'a represents the lifetime ends when the struct ends
struct Cube<'a> {
    color: &'a str, // this is a reference
    amount: usize,
}

struct Game<'a> {
    id: &'a id
    rounds: Vec<Vec<Cube<'a>>>
}

impl Game { // implement some functionality
    fn some_function() {}
}
```
Use [nom](https://docs.rs/nom) to iteratively bottom up parse the input string.
Treemap has a deterministic iteration output as compared to HashMap.