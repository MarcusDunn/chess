use std::collections::LinkedList;
use std::ops::{Sub, Add, Deref};
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

#[derive(Copy, Clone, Debug)]
struct Move {
    from: Location,
    to: Location,
}

#[derive(Copy, Clone, Debug)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn as_tup(&self) -> (i32, i32) {
        let Location { x, y } = self;
        (*x, *y)
    }
    fn as_abs_tup(&self) -> (i32, i32) {
        let (x,y) = self.as_tup();
        (x.abs(), y.abs())
    }
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

#[derive(Copy, Clone)]
enum Piece {
    Rook,
    Knight,
    Pawn,
    King,
    Queen,
    Bishop,
}

impl Movable for Piece {
    fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason> {
        match self {
            Piece::Rook => { Rook::squares_moved(m) }
            Piece::Knight => { Knight::squares_moved(m) }
            Piece::Pawn => { Pawn::squares_moved(m) }
            Piece::King => { King::squares_moved(m) }
            Piece::Queen => { Queen::squares_moved(m) }
            Piece::Bishop => { Bishop::squares_moved(m) }
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

    fn make_move(&mut self, m: Move) -> Result<(), FailReason> {
        if self.is_valid_move(m) {
            self.past_moves.push_front(m);
            self.do_move(m);
            Ok(())
        } else {
            Err(ImpossibleMove)
        }
    }

    fn setup(&mut self) {
        unimplemented!()
    }

    fn back_one_move(&mut self) {
        self.past_moves.pop_front();
        self.setup();
        for m in self.past_moves.clone() { // fix clone somehow
            self.do_move(m)
        }
    }

    fn is_valid_move(&self, m: Move) -> bool {
        unimplemented!()
    }

    fn do_move(&mut self, m: Move) {
        let Move { from, to} = m;
        {
            let piece = self.squares[from.x as usize][from.y as usize].expect("this should really be a valid move");
            self.squares[to.x as usize][to.y as usize] = Some(piece);
        }
        self.squares[from.x as usize][from.y as usize] = None;
    }

}

#[derive(Copy, Clone)]
struct Knight {}

#[derive(Copy, Clone)]
struct Pawn {}

#[derive(Copy, Clone)]
struct King {}

#[derive(Copy, Clone)]
struct Queen {}

#[derive(Copy, Clone)]
struct Bishop {}

#[derive(Copy, Clone)]
struct Rook {}

impl Knight {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;

        match ((to - from).x.abs(), (to - from).y.abs()) {
            (1, 2) => { Ok(vec!(m.to)) }
            (2, 1) => { Ok(vec!(m.to)) }
            _ => Err(ImpossibleMove)
        }
    }
}

impl Pawn {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;
        match (from - to).as_abs_tup() {
            (0, 1) => { Ok(vec![from, to]) }
            (0, 2) => {
                if Pawn::is_in_original_position(from) {
                    if (from-to).y > 0 {
                        Ok(vec![from, from + Location { x: 0, y: 1 }, to])
                    } else {
                        Ok(vec![from, from + Location { x: 0, y: -1 }, to])
                    }
                } else {
                    Err(ImpossibleMove)
                }
            }
            _ => {
                if Pawn::moved_one_diagonal(m) {
                    Ok(vec![from, to])
                } else {
                    Err(ImpossibleMove)
                }
            }
        }
    }

    fn is_in_original_position(from: Location) -> bool {
        from.x == 1 || from.x == 6
    }

    fn moved_one_diagonal(m: Move)-> bool {
        let Location {x, y} = m.from - m.to;
        x.abs() == 1 && y.abs() == 1
    }
}

impl King {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;

        match ((to - from).x.abs(), (to - from).y.abs()) {
            (1, 0) => { Ok(vec!(m.to)) }
            (0, 1) => { Ok(vec!(m.to)) }
            (1, 1) => { Ok(vec!(m.to)) }
            _ => Err(ImpossibleMove)
        }
    }
}

impl Queen {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        if let Ok(rookResult) = Rook::squares_moved(m) {
            Ok(rookResult)
        } else if let Ok(bishopResult) = Bishop::squares_moved(m) {
            Ok(bishopResult)
        } else {
            Err(ImpossibleMove)
        }
    }
}

impl Bishop {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;

        return if from.x - to.x == to.y - to.y {
            let moves = (from.x..=to.x)
                .zip(from.y..=to.y)
                .map(|(x, y)| { Location { x, y } })
                .collect();
            Ok(moves)
        } else {
            Err(ImpossibleMove)
        };
    }
}

impl Rook {
    fn new() -> Self {
        Rook {}
    }

    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
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

