#[derive(Copy, Clone, Debug, Default)]
pub struct AiSlots {
    raw: RawAiSlots,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct AiSlot {
    pub difficulty: AiDifficulty,
    pub country: i32,
    pub color: i32,
    pub start: i32,
    pub allies: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct RawAiSlots {
    difficulties: [i32; 8],
    countries: [i32; 8],
    colors: [i32; 8],
    starts: [i32; 8],
    allies: [i32; 8],
}

#[derive(Copy, Clone, Debug, Default)]
pub enum AiDifficulty {
    Player,
    Hard,
    Normal,
    #[default]
    Easy,
    Extension {
        index: u16,
    },
}

impl AiSlots {
    pub fn get_index(&self, index: usize) -> Option<AiSlot> {
        if index > 7 {
            return None;
        }
        Some(AiSlot {
            difficulty: match self.raw.difficulties[index] {
                -1 => AiDifficulty::Player,
                0 => AiDifficulty::Hard,
                1 => AiDifficulty::Normal,
                2 => AiDifficulty::Easy,
                x => AiDifficulty::Extension { index: x as u16 },
            },
            country: self.raw.countries[index],
            color: self.raw.colors[index],
            start: self.raw.starts[index],
            allies: self.raw.allies[index],
        })
    }
}
