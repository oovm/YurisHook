use crate::crate_hooks::{PowerUpCrate, PowerUps};

pub struct TransposeIndexer<'i, I> {
    pub(crate) raw: &'i I,
    pub(crate) now: usize,
}

pub trait TransposedArray {
    type Transposed;
    fn get_index(&self, index: usize) -> Option<Self::Transposed>;
}

impl<'i> IntoIterator for &'i PowerUps {
    type Item = PowerUpCrate;
    type IntoIter = TransposeIndexer<'i, PowerUps>;

    fn into_iter(self) -> Self::IntoIter {
        TransposeIndexer { raw: self, now: 0 }
    }
}

impl<'i> Iterator for TransposeIndexer<'i, PowerUps> {
    type Item = PowerUpCrate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.now < crate::crate_hooks::POWER_UPS_LIMIT {
            let i = self.raw.get_index(self.now);
            self.now += 1;
            Some(i)
        }
        else {
            None
        }
    }
}
