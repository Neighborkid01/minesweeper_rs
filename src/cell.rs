// use gloo_console as console;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl Value {
    pub fn get_name_string(&self) -> String {
        match self {
            Value::Mine     => String::from(""),
            Value::Zero     => String::from(""),
            Value::One      => String::from("one"),
            Value::Two      => String::from("two"),
            Value::Three    => String::from("three"),
            Value::Four     => String::from("four"),
            Value::Five     => String::from("five"),
            Value::Six      => String::from("six"),
            Value::Seven    => String::from("seven"),
            Value::Eight    => String::from("eight"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayState {
    Default,
    Unknown,
    Flagged,
    Shown(Value),
}

impl DisplayState {
    pub fn get_display_string(&self) -> &str {
        match self {
            DisplayState::Default => " ",
            DisplayState::Flagged => "🚩",
            DisplayState::Unknown => "?",
            DisplayState::Shown(value) => {
                match value {
                    Value::Mine     => "*",
                    Value::Zero     => " ",
                    Value::One      => "1",
                    Value::Two      => "2",
                    Value::Three    => "3",
                    Value::Four     => "4",
                    Value::Five     => "5",
                    Value::Six      => "6",
                    Value::Seven    => "7",
                    Value::Eight    => "8",
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    pub value: Value,
    pub display: DisplayState,
}

impl Cell {
    pub fn new(neighboring_mines: Option<usize>) -> Self {
        let value = Cell::calculate_value(neighboring_mines);

        Self {
            value,
            display: DisplayState::Default,
        }
    }

    pub fn new_empty() -> Self {
        Cell::new(Some(0))
    }

    pub fn reset(&mut self) {
        self.set_value(Value::Zero);
        self.set_display(DisplayState::Default);
    }

    pub fn handle_click(&mut self) {
        match self.display {
            DisplayState::Flagged | DisplayState::Shown(_) => {},
            DisplayState::Default | DisplayState::Unknown => {
                self.set_display(DisplayState::Shown(self.value));
            },
        }
    }

    pub fn cycle_display(&mut self) {
        match self.display {
            DisplayState::Default => { self.set_display(DisplayState::Flagged) }
            DisplayState::Flagged => { self.set_display(DisplayState::Unknown) }
            DisplayState::Unknown => { self.set_display(DisplayState::Default) }
            DisplayState::Shown(value) => { self.set_display(DisplayState::Shown(value)) }
        }
    }

    pub fn is_shown(&self) -> bool {
        self.display == DisplayState::Shown(self.value)
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


    pub fn set_display_to_flagged(&mut self) {
        self.set_display(DisplayState::Flagged);
    }

    pub fn get_value_display_string(&self) -> &str {
        self.display.get_display_string()
    }

    // Private methods
    fn set_display(&mut self, display: DisplayState) {
        self.display = display;
    }

    fn set_value(&mut self, value: Value) {
        self.value = value;
    }

    fn calculate_value(neighboring_mines: Option<usize>) -> Value {
        if neighboring_mines.is_none() { return Value::Mine; }
        match neighboring_mines.unwrap() {
            0 => { Value::Zero },
            1 => { Value::One },
            2 => { Value::Two },
            3 => { Value::Three },
            4 => { Value::Four },
            5 => { Value::Five },
            6 => { Value::Six },
            7 => { Value::Seven },
            8 => { Value::Eight },
            _ => panic!("Unexpected number of neighbors: {}", neighboring_mines.unwrap()),
        }
    }
}