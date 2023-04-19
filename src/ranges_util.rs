use std::{cmp::Ordering, ops::Range};

pub fn merge_ranges<E: PartialOrd>(mut ranges: Vec<Range<E>>) -> Vec<Range<E>> {
    if ranges.len() == 0 {
        return Vec::new();
    }
    ranges.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    let mut ranges = ranges.into_iter();
    let mut res: Vec<Range<E>> = Vec::new();
    res.push(ranges.next().unwrap());
    for range in ranges {
        let last = res.len()-1;
        let last_range = &res[last];
        if last_range.end >= range.start {
            if let Some(Ordering::Less) = last_range.end.partial_cmp(&range.end) {
                res[last].end = range.end;
            }
        } else {
            res.push(range);
        }
    }
    res
}