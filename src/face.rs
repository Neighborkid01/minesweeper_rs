#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Face {
    Happy,
    Nervous,
    Dead,
    Cool,
}

impl Face {
    pub fn as_str(&self) -> &'static str {
        match self {
            Face::Happy     => "🙂",
            Face::Nervous   => "😬",
            Face::Dead      => "😵",
            Face::Cool      => "😎",
        }
    }
}