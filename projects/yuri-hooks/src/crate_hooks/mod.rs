use crate::YuriResult;
use win_memory::WindowsProcess;

#[derive(Debug)]
pub struct PowerUps {
    configs: Vec<PowerUpCrate>,
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

struct PowerUpsTranspose {
    weights: Vec<u32>,
    arguments: Vec<f64>,
    naval: Vec<bool>,
    animation: Vec<i32>,
}

impl PowerUpsTranspose {
    pub fn read(pid: &WindowsProcess) -> YuriResult<Self> {
        let weights = pid.read_data_absolute::<[u32; 19]>(0x81DA8C)?;
        let arguments = pid.read_data_absolute::<[f64; 19]>(0x89EC28)?;
        let naval = pid.read_data_absolute::<[bool; 19]>(0x89ECC0)?;
        let anims = pid.read_data_absolute::<[i32; 19]>(0x81DAD8)?;
    }
    pub fn transpose(self) -> YuriResult<PowerUps> {
        let configs = weights
            .iter()
            .zip(arguments.iter())
            .zip(naval.iter())
            .zip(anims.iter())
            .map(|(((weight, arguments), naval), anim)| PowerUpCrate {
                weight: *weight,
                arguments: *arguments,
                naval: *naval,
                animation: *anim,
            })
            .collect::<Vec<_>>();
        Ok(Self { configs })
    }
}

impl PowerUps {
    pub fn read(pid: &WindowsProcess) -> YuriResult<Self> {
        let weights = pid.read_data_absolute::<[u32; 19]>(0x81DA8C)?;
        let arguments = pid.read_data_absolute::<[f64; 19]>(0x89EC28)?;
        let naval = pid.read_data_absolute::<[bool; 19]>(0x89ECC0)?;
        let anims = pid.read_data_absolute::<[i32; 19]>(0x81DAD8)?;
        let configs = weights
            .iter()
            .zip(arguments.iter())
            .zip(naval.iter())
            .zip(anims.iter())
            .map(|(((weight, arguments), naval), anim)| PowerUpCrate {
                weight: *weight,
                arguments: *arguments,
                naval: *naval,
                animation: *anim,
            })
            .collect::<Vec<_>>();
        Ok(Self { configs })
    }
    pub fn write(&self, pid: &WindowsProcess) -> YuriResult<()> {
        let weights = self.configs.iter().map(|c| c.weight).collect::<Vec<_>>().try_into()?;
    }
}
