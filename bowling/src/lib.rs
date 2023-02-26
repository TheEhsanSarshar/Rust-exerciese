#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Copy)]
pub struct Frame {
    score: u16,
    reamained_pins: u16,
    number_of_balls: u16,
}
pub struct BowlingGame {
    frames: [Frame; 10],
    /* current frame */
    cursor: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        let mut to_return = BowlingGame {
            cursor: 0,
            frames: [Frame {
                score: 0,
                reamained_pins: 10,
                number_of_balls: 2,
            }; 10],
        };
        to_return.frames[9].number_of_balls = 3;
        to_return.frames[9].reamained_pins = 30;
        to_return
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let mut current_frame = &mut self.frames[self.cursor as usize];
        if current_frame.reamained_pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        current_frame.reamained_pins -= pins;
        if current_frame.reamained_pins == 0 {
            self.cursor += 1
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        Some(0);
        if self.cursor == 9 {
            Some(0)
        } else {
            None
        }
        // unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
