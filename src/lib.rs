mod ranges_util;
mod leaderboard_trait;
mod ranked_iter;
mod peek_ranked_iter;
pub use leaderboard_trait::Leaderboard;
pub use peek_ranked_iter::View;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
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
}