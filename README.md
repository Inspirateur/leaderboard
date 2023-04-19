# leaderboard
A rust crate providing 2 functions for Vec useful in leaderboards: 
 - `vec.iter_ranked()`: iterates on a sorted vec and group Equal elements together (ex aequos)
 - `vec.peek_ranked(ranges)`: same principle but with range selection (to see podium and bottom entries for example)

### Example
```rust
fn test() {
    let scores = vec![150, 142, 138, 138, 135, 120, 101, 83, 66, 61, 61];
    let mut rank = 1;
    for view in scores.peek_ranked(vec![0..3, 6..9]) {
        if let View::Skipped(offset) = view {
            println!("{view:?}");
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
Skipped(3)
7. Item([83])
8. Item([66])
9. Item([61, 61])
```