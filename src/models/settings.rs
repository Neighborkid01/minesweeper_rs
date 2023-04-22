const MAX_WIDTH: usize = 32;
const MAX_HEIGHT: usize = 32;
const MAX_MINES: usize = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dimensions {
    width: usize,
    height: usize,
    mines: usize,
}

impl Default for Dimensions {
    fn default() -> Self { Dimensions::new(16, 16, 10) }
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

impl Default for Difficulty {
    fn default() -> Self { Difficulty::Beginner }
}

impl Difficulty {
    fn dimensions(&self) -> Dimensions {
        match self {
            Difficulty::Beginner => { Dimensions::new(9, 9, 10) },
            Difficulty::Intermediate => { Dimensions::new(16, 16, 40) },
            Difficulty::Expert => { Dimensions::new(30, 16, 99) },
            Difficulty::Custom(dimensions) => { *dimensions },
        }
    }

    pub fn title(&self) -> String {
        match self {
            Difficulty::Beginner => { "Beginner".into() },
            Difficulty::Intermediate => { "Intermediate".into() },
            Difficulty::Expert => { "Expert".into() },
            Difficulty::Custom(_) => { "Custom".into() },
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DifficultySetting {
    difficulty: Difficulty,
    dimensions: Dimensions,
}

impl Default for DifficultySetting {
    fn default() -> Self { DifficultySetting::new(Difficulty::default()) }
}

impl DifficultySetting {
    pub fn new(difficulty: Difficulty) -> Self {
        let dimensions = difficulty.dimensions();
        DifficultySetting { difficulty, dimensions }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.dimensions = difficulty.dimensions();
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChordSetting {
    LeftClick,
    LeftAndRightClick,
    Disabled,
}

impl Default for ChordSetting {
    fn default() -> Self { ChordSetting::LeftClick }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FirstClickSetting {
    Any,
    Safe,
    Zero,
}

impl Default for FirstClickSetting {
    fn default() -> Self { FirstClickSetting::Zero }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Settings {
    difficulty_setting: DifficultySetting,
    chord_setting: ChordSetting,
    first_click_setting: FirstClickSetting,
    allow_mark_cell_as_unknown: bool,
}

impl Settings {
    pub fn new(
        difficulty_setting: DifficultySetting,
        chord_setting: ChordSetting,
        first_click_setting: FirstClickSetting,
        allow_mark_cell_as_unknown: bool,
    ) -> Self {
        Settings {
            difficulty_setting,
            chord_setting,
            first_click_setting,
            allow_mark_cell_as_unknown,
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty_setting.difficulty()
    }

    pub fn dimensions(&self) -> Dimensions {
        self.difficulty_setting.dimensions()
    }

    pub fn chord_setting(&self) -> ChordSetting {
        self.chord_setting
    }

    pub fn first_click_setting_is_any(&self) -> bool {
        self.first_click_setting == FirstClickSetting::Any
    }

    pub fn first_click_setting_is_zero(&self) -> bool {
        self.first_click_setting == FirstClickSetting::Zero
    }

    pub fn allow_mark_cell_as_unknown(&self) -> bool {
        self.allow_mark_cell_as_unknown
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty_setting.set_difficulty(difficulty);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::new(
            DifficultySetting::default(),
            ChordSetting::default(),
            FirstClickSetting::Zero,
            false
        )
    }
}