#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if 0 <= rank && rank < 8 && 0 <= file && file < 8 {
            Some(Self { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.file == other.0.file
            || self.0.rank == other.0.rank
            || self.0.rank.abs_diff(other.0.rank) == self.0.file.abs_diff(other.0.file)
    }
}
