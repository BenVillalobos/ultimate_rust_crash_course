use crate::NUM_ROWS;
use crate::NUM_COLS;
use crate::frame::Drawable;
use crate::frame::Frame;

pub struct Enemy {
    x: usize,
    y: usize,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS/2,
            y: 1,
        }
    }
}

impl Drawable for Enemy {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "X";
    }
}