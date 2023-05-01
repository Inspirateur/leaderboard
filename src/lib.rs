mod ranges_util;
mod leaderboard_trait;
mod ranked_iter;
mod sections_iter;
pub use leaderboard_trait::{Ranking, Sections};
pub use sections_iter::View;

#[cfg(test)]
mod tests {
    use crate::{*, leaderboard_trait::Sections};

    #[test]
    fn it_works() {
        let scores = vec![
            150, 142, 138, 138, 135, 120, 101, 83, 66, 61, 61
        ];
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
}