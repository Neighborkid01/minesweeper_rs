use core::fmt::Display;

pub trait Min {
    fn min() -> Self;
}

pub trait Max {
    fn max() -> Self;
}

impl Min for u8 {
    fn min() -> u8 {
        std::u8::MIN
    }
}

impl Max for u8 {
    fn max() -> u8 {
        std::u8::MAX
    }
}


impl Min for i8 {
    fn min() -> i8 {
        std::i8::MIN
    }
}

impl Max for i8 {
    fn max() -> i8 {
        std::i8::MAX
    }
}
