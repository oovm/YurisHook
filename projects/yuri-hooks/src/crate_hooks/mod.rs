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
    weights: [u32; 19],
    arguments: [f64; 19],
    naval: [bool; 19],
    animation: [i32; 19],
}

impl PowerUpsTranspose {
    pub fn read(pid: &WindowsProcess) -> YuriResult<Self> {
        Ok(Self {
            weights: pid.read_data(0x81DA8C)?,
            arguments: pid.read_data(0x89EC28)?,
            naval: pid.read_data(0x89ECC0)?,
            animation: pid.read_data(0x81DAD8)?,
        })
    }
    pub fn write(&self, pid: &mut WindowsProcess) {
        pid.write_data(0x81DA8C, self.weights);
        pid.write_data(0x89EC28, self.arguments);
        pid.write_data(0x89ECC0, self.naval);
        pid.write_data(0x81DAD8, self.animation);
    }
    pub fn transpose(self) -> YuriResult<PowerUps> {
        let configs = self
            .weights
            .iter()
            .zip(self.arguments.iter())
            .zip(self.naval.iter())
            .zip(self.animation.iter())
            .map(|(((weight, arguments), naval), anim)| PowerUpCrate {
                weight: *weight,
                arguments: *arguments,
                naval: *naval,
                animation: *anim,
            })
            .collect::<Vec<_>>();
        Ok(PowerUps { configs })
    }
}

impl PowerUps {
    pub fn read(pid: &WindowsProcess) -> YuriResult<Self> {
        let weights = pid.read_data::<[u32; 19]>(0x81DA8C)?;
        let arguments = pid.read_data::<[f64; 19]>(0x89EC28)?;
        let naval = pid.read_data::<[bool; 19]>(0x89ECC0)?;
        let anims = pid.read_data::<[i32; 19]>(0x81DAD8)?;
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
