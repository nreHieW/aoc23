#### Day 1
Try to use more iterators and less loops. Seems to be a common thing in Rust

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


#### Day 3
Can use LocatedSpan to parse
Dont need to dfs can just check since the border positions are deterministic. 

#### Day 4 
Tried to use Nom to parse. Use `impl` and `struct`
Learnt about borrowing in Rust: Cannot edit something that is being iterated over.
Can use HashSet rather than vec so we can take advantage of intersection and FoldMany rather than separatedlist
Can use tuple parsers in Nom.

#### Day 5 
Rather than use chunks can straight away use separated pair from Nom
Use release mode 
When using progress bar, good to flatmap out everything so that it is easier to view.


#### Day 6
Getting better with iterators and nom parsing