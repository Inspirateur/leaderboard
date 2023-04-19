use std::ops::Range;
use crate::{ranked_iter::RankedIter, peek_ranked_iter::PeekRankedIter};

pub trait Leaderboard<E: PartialEq> {
    fn iter_ranked(&self) -> RankedIter<E>;

    fn peek_ranked(&self, ranges: Vec<Range<usize>>) -> PeekRankedIter<E>;
}

impl<E: PartialEq> Leaderboard<E> for Vec<E> {
    fn iter_ranked(&self) -> RankedIter<E> {
        RankedIter::new(&self)
    }

    fn peek_ranked(&self, ranges: Vec<Range<usize>>) -> PeekRankedIter<E> {
        PeekRankedIter::new(&self, ranges)
    }
}