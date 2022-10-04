use std::collections::HashSet;
use gloo_console as console;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Mine,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DisplayState {
    Default,
    Unknown,
    Flagged,
    Shown,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub value: Value,
    pub display: DisplayState,
}

impl Cell {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            display: DisplayState::Default,
        }
    }

    pub fn handle_click(&mut self) {
        match self.display {
            DisplayState::Default | DisplayState::Unknown => {
                self.display = DisplayState::Shown;
            },
            DisplayState::Flagged | DisplayState::Shown => {
                self.display = self.display;
            }
        }
    }

    pub fn cycle_display(&mut self) {
        match self.display {
            DisplayState::Default => { self.set_display(DisplayState::Flagged) }
            DisplayState::Flagged => { self.set_display(DisplayState::Unknown) }
            DisplayState::Unknown => { self.set_display(DisplayState::Default) }
            DisplayState::Shown   => { self.set_display(DisplayState::Shown) }
        }
    }

    pub fn is_shown(&self) -> bool {
        self.display == DisplayState::Shown
    }

    pub fn is_mine(&self) -> bool {
        self.value == Value::Mine
    }

    pub fn is_flagged(&self) -> bool {
        self.display == DisplayState::Flagged
    }

    pub fn is_zero(&self) -> bool {
        self.value == Value::Zero
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }

    pub fn set_display(&mut self, display: DisplayState) {
        self.display = display;
    }

    pub fn set_display_to_flagged(&mut self) {
        self.set_display(DisplayState::Flagged);
    }

    pub fn calculate_value(index: usize, neighbors: &HashSet<usize>, mines: &HashSet<usize>) -> Value {
        if mines.contains(&index) { return Value::Mine }

        let neighboring_mines = neighbors.intersection(&mines).count();
        match neighboring_mines {
            0 => { Value::Zero },
            1 => { Value::One },
            2 => { Value::Two },
            3 => { Value::Three },
            4 => { Value::Four },
            5 => { Value::Five },
            6 => { Value::Six },
            7 => { Value::Seven },
            8 => { Value::Eight },
            _ => panic!(),
        }
    }

    pub fn get_value_string(&self) -> &str {
        let res = match self.display {
            DisplayState::Default => " ",
            DisplayState::Flagged => "🚩",
            DisplayState::Unknown => "?",
            DisplayState::Shown => {
                match self.value {
                    Value::Mine => "*",
                    Value::Zero => " ",
                    Value::One => "1",
                    Value::Two => "2",
                    Value::Three => "3",
                    Value::Four => "4",
                    Value::Five => "5",
                    Value::Six => "6",
                    Value::Seven => "7",
                    Value::Eight => "8",
                }
            }
        };
        return res;
    }
}