pub trait TransposedArray {
    type Transposed;
    fn get_index(&self, index: usize) -> Option<Self::Transposed>;
}

pub struct TransposeIndexer<I> {
    raw: I,
    now: usize,
}

impl<I: TransposedArray> Iterator for TransposeIndexer<I> {
    type Item = I::Transposed;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.raw.get_index(self.now);
        self.now += 1;
        i
    }
}

impl<I: TransposedArray> TransposeIndexer<I> {
    pub fn new(raw: I) -> TransposeIndexer<I> {
        TransposeIndexer { raw, now: 0 }
    }
}
