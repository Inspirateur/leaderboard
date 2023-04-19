use std::ops::Range;
use crate::{ranked_iter::RankedIter, ranges_util::merge_ranges};

#[derive(Debug)]
pub enum View<E> {
    Skipped(usize),
    Item(E)
}

pub struct PeekRankedIter<'a, E: PartialEq> {
    iter: RankedIter<'a, E>,
    ranges: Vec<Range<usize>>,
    i: usize,
    curr_range: usize,
}

impl<'a, E: PartialEq> PeekRankedIter<'a, E> {
    pub fn new(data: &'a Vec<E>, ranges: Vec<Range<usize>>) -> Self {
        PeekRankedIter { 
            iter: RankedIter::new(data), 
            ranges: merge_ranges(ranges),
            curr_range: 0,
            i: 0
        }
    }
}

impl<'a, E: PartialEq> Iterator for PeekRankedIter<'a, E> {
    type Item = View<&'a [E]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.ranges[self.curr_range].end {
            self.curr_range += 1;
        }
        if self.curr_range >= self.ranges.len() {
            return None;
        }
        if self.i < self.ranges[self.curr_range].start {
            let diff = self.ranges[self.curr_range].start - self.i;
            self.i = self.ranges[self.curr_range].start;
            return Some(View::Skipped(diff));
        }
        self.i += 1;
        if let Some(item) = self.iter.next() {
            Some(View::Item(item))
        } else {
            None
        }
    }
}