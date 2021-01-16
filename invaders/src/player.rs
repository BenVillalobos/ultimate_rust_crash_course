use crate::invader::Invaders;
use core::time::Duration;
use crate::shot::Shot;
use crate::NUM_ROWS;
use crate::NUM_COLS;
use crate::frame::Drawable;
use crate::frame::Frame;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self{
        Self {
            x: NUM_COLS/2,
            y: NUM_ROWS-1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS-1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y-1));
            return true
        }
        false
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        // Remove dead shots by keeping live shots.
        self.shots.retain(|s| !s.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool{
        let mut hit = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit = true;
                    shot.explode();
                }
            }
        }

        hit
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for s in self.shots.iter() {
            s.draw(frame);
        }
    }
}