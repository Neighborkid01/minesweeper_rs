#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[derive(Default)]
pub enum Face {
    #[default]
    Happy,
    Nervous,
    Dead,
    Cool,
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
