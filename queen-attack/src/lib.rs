pub struct ChessPosition { x: isize, y: isize }

impl ChessPosition {
    pub fn new(x: isize, y: isize) -> Result<ChessPosition, &'static str> {
        if 0 <= x && x < 8 && 0 <= y && y < 8 { Ok(ChessPosition{ x: x, y: y}) }
        else { Err("Not a valid position") }
    }
}

pub struct Queen { position: ChessPosition }

impl Queen {
    pub fn new(position: ChessPosition) -> Queen {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (x1, y1) = (self.position.x, self.position.y);
        let (x2, y2) = (other.position.x, other.position.y);
        if x1 == x2 && y1 == y2 { false }
        else { x1 == x2 ||  y1 == y2 || (x1 - x2).abs() == (y2 - y1).abs() }
    }
}
