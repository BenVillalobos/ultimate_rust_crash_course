use crate::frame::Drawable;
use crate::frame::Frame;
use crate::NUM_COLS;
use crate::NUM_ROWS;
use core::cmp::max;
use core::time::Duration;
use rusty_time::prelude::Timer;

pub struct Invader {
    x: usize,
    y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invader {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: 1,
        }
    }
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x > 1 && x < NUM_COLS - 2 && y > 0 && y < 9 && x % 2 == 0 && y % 2 == 0 {
                    army.push(Invader { x, y });
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: -1, // left
        }
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|inv| inv.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self.army.iter().position(|inv| (inv.x == x) && inv.y == y) {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        // If the timer has run out (2 seconds passed)
        if self.move_timer.ready {
            self.move_timer.reset();

            let mut move_down = false;

            // If we're going right (TODO: Enum)
            /*
            // This doesn't work?
            let x_values = self.army.iter().map(|inv| inv.x);
            let min_x = x_values.min().unwrap();
            let max_x = x_values.max().unwrap();
            */
            if self.direction == -1 {
                let min_x = self.army.iter().map(|inv| inv.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction *= -1;
                    move_down = true;
                }
            } else {
                let max_x = self.army.iter().map(|inv| inv.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction *= -1;
                    move_down = true;
                }
            }

            if move_down {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }
}

impl Drawable for Invader {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "X";
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            }
        }
    }
}