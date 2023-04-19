use std::ops::Range;
use crate::ranges_util::merge_ranges;

#[derive(Debug)]
pub enum View<E> {
    Skipped(usize),
    Item(E)
}

pub struct SectionsIter<'a, E> {
    data: &'a Vec<E>,
    ranges: Vec<Range<usize>>,
    i: usize,
    curr_range: usize,
}

impl<'a, E> SectionsIter<'a, E> {
    pub fn new(data: &'a Vec<E>, ranges: Vec<Range<usize>>) -> Self {
        SectionsIter { 
            data, 
            ranges: merge_ranges(ranges),
            curr_range: 0,
            i: 0
        }
    }
}

impl<'a, E> Iterator for SectionsIter<'a, E> {
    type Item = View<&'a E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.ranges[self.curr_range].end {
            self.curr_range += 1;
        }
        if self.curr_range >= self.ranges.len() {
            return None;
        }
        if self.i < self.ranges[self.curr_range].start {
            let diff = self.ranges[self.curr_range].start - self.i;
            // skip ahead to the next range
            self.i += diff;
            return Some(View::Skipped(diff));
        }
        if self.i >= self.data.len() {
            return None;
        }
        let res = &self.data[self.i];
        self.i += 1;
        Some(View::Item(res))
    }
}