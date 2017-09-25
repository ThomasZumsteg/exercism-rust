pub struct BowlingGame { 
    second: bool,
    pins: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> BowlingGame { BowlingGame { pins: Vec::new(), second: false} }

    pub fn roll(&mut self, pins: u16) -> Result<&BowlingGame, &'static str> {
        if 10 < pins || (self.second && 10 < pins + self.pins.last().unwrap()){ 
            return Err("Invalid number of pins") 
        } else if self.score().is_ok() { return Err("Game is over") }
        self.pins.push(pins);
        self.second = if pins != 10 { !self.second } else { false };
        Ok(self)
    }

    pub fn score(&self) -> Result<u16, &'static str> {
        let (mut total, mut f) = (0, 0);
        let pins = &self.pins;
        for _ in 0..10 {
            if let (Some(&first), Some(&second)) = (pins.get(f), pins.get(f+1)) {
                total += first + second;
                if first == 10 || first + second == 10 { 
                    if let Some(&third) = pins.get(f+2) { total += third; }
                    else { return Err("Game not done"); }
                }
                f += if first == 10 { 1 } else { 2 };
            } else { return Err("Game not done"); }
        }
        Ok(total)
    }
}

