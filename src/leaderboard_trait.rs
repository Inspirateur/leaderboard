use std::ops::Range;
use crate::{ranked_iter::RankedIter, sections_iter::SectionsIter};

pub trait Ranking<E: PartialEq> {
    fn iter_ranked(&self) -> RankedIter<E>;
}

pub trait Sections<E> {
    fn iter_sections(&self, ranges: Vec<Range<usize>>) -> SectionsIter<E>;
}

impl<E: PartialEq> Ranking<E> for Vec<E> {
    fn iter_ranked(&self) -> RankedIter<E> {
        RankedIter::new(&self)
    }
}

impl<E> Sections<E> for Vec<E> {
    fn iter_sections(&self, ranges: Vec<Range<usize>>) -> SectionsIter<E> {
        SectionsIter::new(&self, ranges)
    }
}