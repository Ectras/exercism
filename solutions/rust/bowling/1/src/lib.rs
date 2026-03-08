#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

trait Frame: std::fmt::Debug {
    fn is_done(&self) -> bool;
    fn throw_is_valid(&self, pins: u16) -> bool;
    fn add_throw(&mut self, pins: u16) -> Result<(), Error>;
    fn throws(&self) -> &Vec<u16>;
    fn score<'a>(&self, next_throws: &mut Box<dyn Iterator<Item = u16> + 'a>) -> u16;
}

#[derive(Debug, Default)]
struct NormalFrame {
    throws: Vec<u16>,
}

impl Frame for NormalFrame {
    fn is_done(&self) -> bool {
        match &self.throws[..] {
            [10] => true,
            [_, _] => true,
            _ => false,
        }
    }

    fn throw_is_valid(&self, pins: u16) -> bool {
        match &self.throws[..] {
            [first_throw] => first_throw + pins <= 10,
            _ => pins <= 10,
        }
    }

    fn add_throw(&mut self, pins: u16) -> Result<(), Error> {
        if self.throw_is_valid(pins) {
            self.throws.push(pins);
            Ok(())
        } else {
            Err(Error::NotEnoughPinsLeft)
        }
    }

    fn throws(&self) -> &Vec<u16> {
        &self.throws
    }

    fn score<'a>(&self, next_throws: &mut Box<dyn Iterator<Item = u16> + 'a>) -> u16 {
        match &self.throws[..] {
            [10] => 10 + next_throws.next().unwrap() + next_throws.next().unwrap(),
            [a, b] if a + b == 10 => 10 + next_throws.next().unwrap(),
            [a, b] => a + b,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct FinalFrame {
    throws: Vec<u16>,
}

impl Frame for FinalFrame {
    fn is_done(&self) -> bool {
        match &self.throws[..] {
            [] | [_] => false,
            [10, _] => false,
            [a, b] if a + b == 10 => false,
            _ => true,
        }
    }

    fn throw_is_valid(&self, pins: u16) -> bool {
        match &self.throws[..] {
            [first_throw] if *first_throw < 10 => first_throw + pins <= 10,
            [10, second_throw] if *second_throw < 10 => second_throw + pins <= 10,
            _ => pins <= 10,
        }
    }

    fn add_throw(&mut self, pins: u16) -> Result<(), Error> {
        if self.throw_is_valid(pins) {
            self.throws.push(pins);
            Ok(())
        } else {
            Err(Error::NotEnoughPinsLeft)
        }
    }

    fn throws(&self) -> &Vec<u16> {
        &self.throws
    }

    fn score<'a>(&self, _next_throws: &mut Box<dyn Iterator<Item = u16> + 'a>) -> u16 {
        self.throws.iter().sum()
    }
}

pub struct BowlingGame {
    frames: Vec<Box<dyn Frame>>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![Box::new(NormalFrame::default())],
        }
    }

    fn new_frame(&self) -> Box<dyn Frame> {
        if self.frames.len() < 9 {
            Box::new(NormalFrame::default())
        } else {
            Box::new(FinalFrame::default())
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // Check if a new frame needs to be started
        let current_frame = self.frames.last().unwrap();
        if current_frame.is_done() {
            if self.frames.len() < 10 {
                self.frames.push(self.new_frame());
            } else {
                return Err(Error::GameComplete);
            }
        }

        // Add the throw to the current frame
        let current_frame = self.frames.last_mut().unwrap();
        current_frame.add_throw(pins)?;

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // Check if the game is done
        if self.frames.len() < 10 || !self.frames.last().unwrap().is_done() {
            return None;
        }

        // Compute the score for each frame
        let mut score = 0;
        for (i, frame) in self.frames.iter().enumerate() {
            let next_throws_it = self.frames[(i + 1)..]
                .iter()
                .flat_map(|f| f.throws())
                .copied();
            let mut next_throws: Box<dyn Iterator<Item = u16>> = Box::new(next_throws_it);
            score += frame.score(&mut next_throws);
        }
        Some(score)
    }
}
