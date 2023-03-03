const MAX_WIDTH: usize = 32;
const MAX_HEIGHT: usize = 32;
const MAX_MINES: usize = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize, mines: usize) -> Self { // This should not stay public
        let w = if width  > MAX_WIDTH  { MAX_WIDTH }  else { width };
        let h = if height > MAX_HEIGHT { MAX_HEIGHT } else { height };
        let m = if mines  > MAX_MINES  { MAX_MINES }  else { mines };
        Dimensions { width: w, height: h, mines: m }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
    Custom(Dimensions),
}

impl Difficulty {
    fn get_dimensions(&self) -> Dimensions {
        match self {
            Difficulty::Beginner => { Dimensions::new(9, 9, 10) },
            Difficulty::Intermediate => { Dimensions::new(16, 16, 40) },
            Difficulty::Expert => { Dimensions::new(30, 16, 99) },
            Difficulty::Custom(dimensions) => { dimensions.clone() },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChordSetting {
    LeftClick,
    LeftAndRightClick,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Settings {
    pub difficulty: Difficulty,
    pub dimensions: Dimensions,
    pub chord_setting: ChordSetting,
}

impl Settings {
    pub fn new(difficulty: Difficulty, chord_setting: ChordSetting) -> Self {
        let dimensions = difficulty.get_dimensions();
        Settings {
            difficulty,
            dimensions,
            chord_setting,
        }
    }
}