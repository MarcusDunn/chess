use std::collections::LinkedList;
use std::ops::{Sub, Add};
use std::convert::TryInto;

#[test]
fn test_create_game() {
    Board::new();
}

#[test]
fn test_create_game_setup_a_rook() {
    let mut board = Board::new();
    board.setup_a_rook()
}

struct Board {
    squares: [[Option<Box<dyn Movable>>; 8]; 8],
    past_moves: LinkedList<Move>,
}

struct Move {
    from: Location,
    to: Location,
}

#[derive(Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Sub for Location {
    type Output = Location;

    fn sub(self, rhs: Self) -> Self::Output {
        Location {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Location {
    type Output = Location;

    fn add(self, rhs: Self) -> Self::Output {
        Location {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

enum FailReason {
    ImpossibleMove
}

impl Board {
    fn new() -> Self {

        Board {
            squares: Default::default(),
            past_moves: LinkedList::new(),
        }
    }

    fn setup_a_rook(&mut self) {
        self.squares[0][0] = Some(Box::new(Rook::new()))
    }
}

struct Rook {}

impl Rook {
    fn new() -> Self {
        Rook{}
    }
}

impl Movable for Rook {
    fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason> {
        let Move {from, to} = m;
        match to - from {
            Location { x: x, y: 0 } => {
                let moves = (0..=x).into_iter()
                    .map(|x| { from + Location { x, y: 0 } })
                    .collect();
                Ok(moves)
            }
            Location { x: 0, y: y } => {
                let moves = (0..=y).into_iter()
                    .map(|y| { from + Location { x: 0, y } })
                    .collect();
                Ok(moves)
            }
            _ => Err(FailReason::ImpossibleMove)
        }
    }
}


trait Movable {
    fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason>;
}

