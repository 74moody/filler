#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    pub fn from_handshake(line: &str) -> Option<Player> {
        // Typical: "$$$ exec p1 : [...]"
        // Robust: search "p1" or "p2"
        if line.contains("p1") {
            Some(Player::P1)
        } else if line.contains("p2") {
            Some(Player::P2)
        } else {
            None
        }
    }

    #[inline]
    pub fn is_me(self, c: char) -> bool {
        match self {
            Player::P1 => c == '@' || c == 'a',
            Player::P2 => c == '$' || c == 's',
        }
    }

    #[inline]
    pub fn is_opp(self, c: char) -> bool {
        c != '.' && !self.is_me(c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Anfield {
    pub w: usize,
    pub h: usize,
    pub cells: Vec<Vec<char>>, // [y][x]
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub filled: Vec<Pos>, // offsets of 'O'/'#' relative to top-left placement
    pub sum_dx: i64,      // sum of filled x offsets
    pub sum_dy: i64,      // sum of filled y offsets
}

#[derive(Clone, Debug)]
pub struct Round {
    pub me: Player,
    pub board: Anfield,
    pub piece: Piece,
}
