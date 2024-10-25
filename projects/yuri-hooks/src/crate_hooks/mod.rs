use crate::{
    YuriResult,
    helpers::{TransposeIndexer, TransposedArray},
};
use std::fmt::{Debug, Display, Formatter};
use win_memory::{MemoryError, ProcessData, WindowsProcess};

const POWER_UPS_LIMIT: usize = 19;

#[derive(Copy, Clone, Debug)]
pub struct PowerUps {
    weights: [u32; POWER_UPS_LIMIT],
    arguments: [f64; POWER_UPS_LIMIT],
    naval: [bool; POWER_UPS_LIMIT],
    animation: [i32; POWER_UPS_LIMIT],
}
#[derive(Copy, Clone, Debug)]
pub struct PowerUpCrate {
    /// the weight of the effect
    weight: u32,
    /// the effect-specific argument
    arguments: f64,
    /// can this crate appear on water?
    naval: bool,
    /// index of AnimTypeClass::Array
    animation: i32,
}

impl TransposedArray for PowerUps {
    type Transposed = PowerUpCrate;

    fn get_index(&self, index: usize) -> Option<Self::Transposed> {
        if index < POWER_UPS_LIMIT {
            return None;
        }
        unsafe {
            Some(PowerUpCrate {
                weight: *self.weights.get_unchecked(index),
                arguments: *self.arguments.get_unchecked(index),
                naval: *self.naval.get_unchecked(index),
                animation: *self.animation.get_unchecked(index),
            })
        }
    }
}
impl IntoIterator for PowerUps {
    type Item = PowerUpCrate;
    type IntoIter = TransposeIndexer<PowerUps>;

    fn into_iter(self) -> Self::IntoIter {
        TransposeIndexer::new(self)
    }
}

impl Display for PowerUps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.clone().into_iter().collect::<Vec<_>>().fmt(f)
    }
}

impl ProcessData for PowerUps {
    fn read(pid: &WindowsProcess) -> Result<Self, MemoryError> {
        Ok(Self {
            weights: pid.read_data(0x81DA8C)?,
            arguments: pid.read_data(0x89EC28)?,
            naval: pid.read_data(0x89ECC0)?,
            animation: pid.read_data(0x81DAD8)?,
        })
    }

    fn write(&self, pid: &mut WindowsProcess) -> Result<(), MemoryError> {
        pid.write_data(0x81DA8C, self.weights)?;
        pid.write_data(0x89EC28, self.arguments)?;
        pid.write_data(0x89ECC0, self.naval)?;
        pid.write_data(0x81DAD8, self.animation)?;
        Ok(())
    }
}
