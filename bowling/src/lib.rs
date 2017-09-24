pub struct BowlingGame { 
    frame: usize,
    pins: [u16; 22]
}

impl BowlingGame {
    pub fn new() -> BowlingGame { BowlingGame { frame: 0, pins: [0; 22]} }

    pub fn roll(&mut self, pins: u16) -> Result<&BowlingGame, &'static str> {
        if 10 < pins { return Err("Invalid number of pins") } 
        else if self.frame >= 22 { return Err("Game is over") }
        self.pins[self.frame] = pins;
        if (self.frame == 20 || self.frame == 21) && self.pins[18] == 10 { self.frame += 1}
        else if self.frame == 20 && (self.pins[18] + self.pins[19] == 10) { self.frame += 2}
        else if pins == 10 && self.frame % 2 == 0 { self.frame += 2 }
        else { self.frame += 1 }
        Ok(self)
    }

    pub fn score(&self) -> Result<u16, &'static str> {
        println!("Score {:?}", self.pins);
        if self.frame < 22 { return Err("Game not over"); }
        let mut total = 0;
        for mut f in 0..10 {
            f *= 2;
            if self.pins[f] == 10 { 
                total += 10 + self.pins[f+2] + self.pins[f+3];
            } else if self.pins[2*f] + self.pins[f + 1] == 10 {
                total += 10 + self.pins[f + 2];
            } else {
                total += self.pins[f] + self.pins[f+1]
            }
        }
        Ok(total)
    }
}

