#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Face {
    Happy,
    Nervous,
    Dead,
    Cool,
}

impl Default for Face {
    fn default() -> Self {
        Face::Happy
    }
}

impl Face {
    pub fn to_str(&self) -> &'static str {
        match self {
            Face::Happy => "🙂",
            Face::Nervous => "😬",
            Face::Dead => "😵",
            Face::Cool => "😎",
        }
    }
}
