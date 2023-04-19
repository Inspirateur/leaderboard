mod ranges_util;
mod leaderboard_trait;
mod ranked_iter;
mod peek_ranked_iter;
pub use leaderboard_trait::Leaderboard;


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let scores = vec![150, 142, 138, 138, 135, 120, 101, 83, 66];
        for view in scores.peek_ranked(vec![0..3, 5..8, 6..9]) {
            println!("{:?}", view);
        }
    }
}