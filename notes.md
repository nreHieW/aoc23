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

#### Day 18
Tried winding rule originally but that does not work since some points might coincide nicely with an edge. 
Need to use [Shoelace Formula](https://en.wikipedia.org/wiki/Shoelace_formula) and [Pick's Theorem](https://en.wikipedia.org/wiki/Pick's_theorem). Thing to note is that Shoelace Formula gives the area and NOT the number of interior points. 

#### Day 19
Closures in rust are basically lambda functions. Quite happy with my part 2 States solution. 
Some things to note:
- Can use `Ord`
- labelled loop break

#### Day 20
Part 1 was abit messy but had to rely on the LCM 4 different sets insight online to solve. Learnt about some 'OOP' support in Rust.

#### Day 21
Mathy day. Euclidean reminder is always a non-negative number whereas with regular division sign of the reminder depends on the sign of the dividend.

#### Day 22 
More straightforward but the implementation is messy.

#### Day 23
Can just brute force dfs all paths. Visited is only on the current path so the path serves as the state.
For part 2, treat as a weighted DAG since some cells only have one direction.

#### Day 24
Reference Stack Overflow for part1. Part 2 I learnt to use the Z3 solver. Compilation, build and run times were very slow on my machine so not sure if it is an Apple Silicon issue.

To use the z solver solution: add `z3 = { version = "0.12", git="https://github.com/prove-rs/z3.rs.git", features = ["static-link-z3"]}` to `Cargo.toml`

The linear algebra solution works because we can frame the rock as standing still and applying a delta vector to each of the hailstones instead. It is inspired from [this video](https://www.youtube.com/watch?v=nP2ahZs40U8&t=425s) and this [comment](https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/keq7g67/?utm_source=share&utm_medium=web2x&context=3)

Some notes about the linear algebra solution:
- Rust cannot natively compare floats
- The code currently had rounding errors which might (?) be due to how rust treats numbers 
- When printing all solutions, the correct solution for my input has the smallest variance across all other possible solutions: `Found a possible solution: (-154, -75), largest x : 10, largest y 15.5625`
- Given that the other differences are large integer numbers, I decided to just cast this to `i64` and use a hashmap to keep track of all possible solutions.
- The final solutions is ultimately the smallest difference.

The working theory that I have as to why this is the case is that all interesection (x, y) coordinates are casted as `f64`. However, if I print the actual intersections, they are either the above case or very large 10+ digit numbers with a single decimal point. Floats are stored 64 bit IEEE 754, with a exponent of 11 bits. This means they can store up to a maximum value of roughly 2^11 which overflows the actual values of the intersection. 

#### Day 25
Cannot use Tarjans for an undirected graph because it is just a single connected component. Works if we just use the top 3 most used nodes. Seems like something like [this](https://docs.rs/rustworkx-core/0.13.2/rustworkx_core/connectivity/fn.stoer_wagner_min_cut.html) works as well.