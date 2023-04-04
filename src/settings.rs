const MAX_WIDTH: usize = 32;
const MAX_HEIGHT: usize = 32;
const MAX_MINES: usize = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dimensions {
    width: usize,
    height: usize,
    mines: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize, mines: usize) -> Self { // This should not stay public
        let w = if width  > MAX_WIDTH  { MAX_WIDTH }  else { width };
        let h = if height > MAX_HEIGHT { MAX_HEIGHT } else { height };
        let m = if mines  > MAX_MINES  { MAX_MINES }  else { mines };
        Dimensions { width: w, height: h, mines: m }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn mines(&self) -> usize {
        self.mines
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
            Difficulty::Custom(dimensions) => { *dimensions },
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChordSetting {
    LeftClick,
    LeftAndRightClick,
    Disabled,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FirstClickSetting {
    Any,
    Safe,
    Zero,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Settings {
    difficulty: Difficulty,
    dimensions: Dimensions,
    chord_setting: ChordSetting,
    first_click_setting: FirstClickSetting,
    allow_mark_cell_as_unknown: bool,
}

impl Settings {
    pub fn new(
        difficulty: Difficulty,
        chord_setting: ChordSetting,
        first_click_setting: FirstClickSetting,
        allow_mark_cell_as_unknown: bool,
    ) -> Self {
        let dimensions = difficulty.get_dimensions();
        Settings {
            difficulty,
            dimensions,
            chord_setting,
            first_click_setting,
            allow_mark_cell_as_unknown,
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn chord_setting(&self) -> ChordSetting {
        self.chord_setting
    }

    pub fn first_click_setting(&self) -> FirstClickSetting {
        self.first_click_setting
    }

    pub fn allow_mark_cell_as_unknown(&self) -> bool {
        self.allow_mark_cell_as_unknown
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.dimensions = difficulty.get_dimensions();
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::new(Difficulty::Beginner, ChordSetting::LeftClick, FirstClickSetting::Zero, false)
    }
}