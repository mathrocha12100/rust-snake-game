#[derive(Clone, Debug)]
pub struct Screen2D {
    pub y: usize,
    pub x: usize,
}

impl Screen2D {
    pub fn create(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

pub type Vec2D = Vec<Screen2D>;
