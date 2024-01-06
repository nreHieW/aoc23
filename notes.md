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

#### Day 7 
A little bit of trial and error with Nom but improving. Can just use normal `lines()` in the future.
Ordering is equivalent to CompareTo
`*char_map.entry(largest_key).or_insert(0) += num_jokers;` mutable editing of HashMap
Can use Enums

#### Day 8
Improving at parsing and used `any` and `loop`
Can use `cycle()` for infinite

#### Day 9
`Map` is lazy so need to be careful when reverse iterating with state

#### Day 10
Had to use the winding rule. Hardest day so far. Code is a monstrosity. 

#### Day 11
`Range<i64>` does not have length so just find the start and end

#### Day 12
Can use `itertools::permutations`
Hard day with the dynamic programming. Credits to [this](https://github.com/mfornet/advent-of-code-2023/blob/main/src/bin/12.rs) for the help.

#### Day 13
Pretty annoying day with many edge cases but brute force works

#### Day 14
Try to always think of the simpler way rather than the efficient way

#### Day 15 
More straightforward day. `&mut` to get mutable reference

#### Day 16
Difficult day especially after the long break from holidays. Use the state trick. Not the most efficient or fast

#### Day 17
Rusty on Dijkstra but need to start learning to model things as State and not just location on a grid.
Remember that a `PriorityQueue` has `push` and `push_increase` courtesy of [this](https://www.reddit.com/r/adventofcode/comments/18zd7mz/2023_day_17_part_1rust_help_with_day_17_part_1/)