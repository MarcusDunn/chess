use std::collections::LinkedList;
use std::ops::{Sub, Add, Deref};
use std::convert::TryInto;
use crate::FailReason::ImpossibleMove;

#[test]
fn test_create_game() {
    Board::new();
}

#[test]
fn test_create_game_setup_a_rook() {
    let mut board = Board::new();
    board.setup_a_rook();
}

#[test]
fn test_get_a_rook_from_board() {
    let mut board = Board::new();
    board.setup_a_rook();
    match &board.squares[0][0] {
        None => { panic!("should be a rook here") }
        Some(_) => {}
    }
}

#[test]
fn test_squared_moved_legal() {
    let mut board = Board::new();
    board.setup_a_rook();
    let squares = match &board.squares[0][0] {
        None => { panic!("should be a rook here") }
        Some(rook) => {
            rook.deref()
                .squares_moved_over(Move {
                    from: Location { x: 0, y: 0 },
                    to: Location { x: 7, y: 0 },
                })
        }
    };
    assert_eq!(
        squares.unwrap_or(Vec::new()),
        (0..=7).into_iter()
            .map(|x| { Location { x, y: 0 } })
            .collect::<Vec<Location>>());
}

#[test]
fn test_move_illegally() {
    let mut board = Board::new();
    board.setup_a_rook();
    let squares = match &board.squares[0][0] {
        None => { panic!("should be a rook here") }
        Some(rook) => {
            rook.deref()
                .squares_moved_over(Move {
                    from: Location { x: 0, y: 0 },
                    to: Location { x: 7, y: 7 },
                })
        }
    };
    assert_eq!(squares.err().unwrap(), FailReason::ImpossibleMove)
}


struct Board {
    squares: [[Option<Piece>; 8]; 8],
    past_moves: LinkedList<Move>,
}

struct Move {
    from: Location,
    to: Location,
}

#[derive(Copy, Clone, Debug)]
struct Location {
    x: i32,
    y: i32,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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

enum Piece {
    Rook,
    // Knight,
    // Pawn,
    // King,
    // Queen,
    // Bishop,
}

impl Movable for Piece {
    fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason> {
        match self {
            Piece::Rook => { Rook::squares_moved_over(m) }
            // Piece::Knight => {Knight::squares_moved_over(m)}
            // Piece::Pawn => {Pawn::squares_moved_over(m)}
            // Piece::King => {King::squares_moved_over(m)}
            // Piece::Queen => {Queen::squares_moved_over(m)}
            // Piece::Bishop => {Bishop::squares_moved_over(m)}
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

#[derive(Debug)]
enum FailReason {
    ImpossibleMove
}

impl PartialEq for FailReason {
    fn eq(&self, other: &Self) -> bool {
        match self {
            FailReason::ImpossibleMove => {
                match other {
                    FailReason::ImpossibleMove => { true }
                }
            }
        }
    }
}

impl Board {
    fn new() -> Self {
        Board {
            squares: Default::default(),
            past_moves: LinkedList::new(),
        }
    }

    fn setup_a_rook(&mut self) {
        self.squares[0][0] = Some(Piece::Rook)
    }
}

struct Knight {}

struct Pawn {}

struct King {}

struct Queen {}

struct Bishop {}

struct Rook {}

impl Knight {
    fn new() -> Self {
        Knight {}
    }

    fn squares_moved_over(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;

        match ((to - from).x.abs(), (to - from).y.abs()) {
            (1, 2) => { Ok(vec!(m.to)) }
            (2, 1) => { Ok(vec!(m.to)) }
            _ => Err(ImpossibleMove)
        }
    }
}

impl Pawn {
    fn new() -> Self {
        Pawn {}
    }

    fn squares_moved_over(m: Move) -> Result<Vec<Location>, FailReason> {
        unimplemented!()
    }
}

impl King {
    fn new() -> Self {
        King {}
    }

    fn squares_moved_over(m: Move) -> Result<Vec<Location>, FailReason> {
        unimplemented!()
    }
}

impl Queen {
    fn new() -> Self {
        Queen {}
    }
}

impl Bishop {
    fn new() -> Self {
        Bishop {}
    }

    fn squares_moved_over(m: Move) -> Result<Vec<Location>, FailReason> {
        unimplemented!()
    }
}

impl Rook {
    fn new() -> Self {
        Rook {}
    }

    fn squares_moved_over(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;
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

            _ => Err(ImpossibleMove)
        }
    }
}


trait Movable {
    fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason>;
}

