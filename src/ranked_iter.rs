pub struct RankedIter<'a, E: PartialEq> {
    data: &'a Vec<E>,
    i: usize
}

impl<'a, E: PartialEq> RankedIter<'a, E> {
    pub fn new(data: &'a Vec<E>) -> Self {
        RankedIter { data: data, i: 0 }
    }
}

impl<'a, E: PartialEq> Iterator for RankedIter<'a, E> {
    type Item = &'a [E];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.data.len() {
            return None;
        }
        let mut j = self.i+1;
        while j < self.data.len() && self.data[self.i] == self.data[j] {
            j += 1;
        }
        let res = &self.data[self.i..j];
        self.i = j;
        Some(res)
    }
}