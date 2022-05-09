#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Cage {
    pub(crate) index: usize,
    pub(crate) sum: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CageColor {
    Yellow,
    Red,
    Green,
    Blue,
}
