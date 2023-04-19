# leaderboard
A rust crate providing 2 functions for Vec useful in leaderboards: 
 - `vec.iter_ranked()`: iterates on a sorted vec and group Equal elements together (ex aequos)
 - `vec.iter_sections(ranges)`: iterate over the vec on the domain defined by a vec of ranges

### Example
Here's how the 2 functions can be used together to display top and bottom 3 ranks with ex aequos:
```rust
fn test() {
    let scores = vec![150, 142, 138, 138, 135, 120, 101, 83, 66, 61, 61];
    let ranking: Vec<_> = scores.iter_ranked().collect();
    let last = ranking.len();
    let mut rank = 1;
    for view in ranking.iter_sections(vec![0..3, last-3..last]) {
        if let View::Skipped(offset) = view {
            println!("...");
            rank += offset
        } else {
            println!("{rank}. {view:?}");
            rank += 1
        };
    }
}

```

Output:
```
1. Item([150])
2. Item([142])
3. Item([138, 138])
...
7. Item([83])
8. Item([66])
9. Item([61, 61])
```