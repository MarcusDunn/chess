#![feature(try_trait)]

use std::collections::LinkedList;
use std::ops::{Sub, Add, Deref, Not};
use std::borrow::Borrow;
use std::option::NoneError;

#[test]
fn test_create_game() {
    Board::new();
}

#[test]
fn test_create_game_setup_a_rook() {
    let mut board = Board::new();
    board.setup_a_rook().unwrap();
}

#[test]
fn test_get_a_rook_from_board() {
    let mut board = Board::new();
    board.setup_a_rook().unwrap();
    match &board.squares[0][0] {
        None => { panic!("should be a rook here") }
        Some(_) => {}
    }
}

#[test]
fn test_squared_moved_legal() {
    let mut board = Board::new();
    board.setup_a_rook().unwrap();
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
fn test_move_piece_out_of_bound() {
    let mut board = Board::new();
    board.setup_a_rook().unwrap();
    board.make_move(Move {
        from: Location { x: 0, y: 0 },
        to: Location { x: -1, y: -1 },
    }).unwrap_err();
}

#[test]
fn test_move_rook_legal_1() {
    let piece = Piece::Rook(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 7, y: 0 };
    Tester::place_and_move_legal(piece, from, to);
}

#[test]
fn test_move_rook_illegal_1() {
    let piece = Piece::Rook(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 7, y: 2 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_move_queen_legal_1() {
    let piece = Piece::Queen(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 0, y: 5 };
    Tester::place_and_move_legal(piece, from, to);
}

#[test]
fn test_move_queen_illegal_1() {
    let piece = Piece::Queen(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 1, y: 5 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_move_bishop_legal_1() {
    let piece = Piece::Bishop(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 5, y: 5 };
    Tester::place_and_move_legal(piece, from, to);
}

#[test]
fn test_move_bishop_illegal_1() {
    let piece = Piece::Bishop(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 6, y: 5 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_move_pawn_legal_1() {
    let piece = Piece::Pawn(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 0, y: 1 };
    Tester::place_and_move_legal(piece, from, to);
}

#[test]
fn test_move_pawn_illegal_1() {
    let piece = Piece::Pawn(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 0, y: 2 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_move_king_legal_1() {
    let piece = Piece::King(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 0, y: 1 };
    Tester::place_and_move_legal(piece, from, to);
}

#[test]
fn test_move_king_illegal_1() {
    let piece = Piece::King(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 0, y: 2 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_move_knight_legal_1() {
    let piece = Piece::Knight(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 2, y: 1 };
    Tester::place_and_move_legal(piece, from, to);
}

#[test]
fn test_move_knight_illegal_1() {
    let piece = Piece::Knight(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 3, y: 1 };
    Tester::place_and_move_illegal(piece, from, to);
}

struct Tester;

impl Tester {
    fn place_and_move_legal(p: Piece, from: Location, to: Location) {
        let mut board = Board::new();
        board.place(p, from).unwrap();
        board.make_move(Move { from, to, }).unwrap();
        board.get_piece_from(&to).unwrap();
    }

    fn place_and_move_illegal(p: Piece, from: Location, to: Location) {
        let mut board = Board::new();
        board.place(Piece::King(Color::Black), from).unwrap();
        board.make_move(Move { from, to, }).unwrap_err();
    }
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
        let (x, y) = self.as_tup();
        (x.abs(), y.abs())
    }
    fn is_in_bounds(&self) -> bool {
        let (x, y) = self.as_tup();
        x >= 0 && x <= 7 && y >= 0 && y <= 7
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
    Rook(Color),
    Knight(Color),
    Pawn(Color),
    King(Color),
    Queen(Color),
    Bishop(Color),
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

impl Piece {
    fn color(&self) -> &Color {
        match self {
            Piece::Rook(c) => { c }
            Piece::Knight(c) => { c }
            Piece::Pawn(c) => { c }
            Piece::King(c) => { c }
            Piece::Queen(c) => { c }
            Piece::Bishop(c) => { c }
        }
    }
}

impl Movable for Piece {
    fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason> {
        match self {
            Piece::Rook(_) => { Rook::squares_moved(m) }
            Piece::Knight(_) => { Knight::squares_moved(m) }
            Piece::Pawn(_) => { Pawn::squares_moved(m) }
            Piece::King(_) => { King::squares_moved(m) }
            Piece::Queen(_) => { Queen::squares_moved(m) }
            Piece::Bishop(_) => { Bishop::squares_moved(m) }
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

#[derive(Debug, PartialOrd, PartialEq)]
enum FailReason {
    ImpossibleMove,
    NoPieceHere,
    Blocked,
    OutOfBounds,
}

impl From<NoneError> for FailReason {
    fn from(_: NoneError) -> Self {
        FailReason::NoPieceHere
    }
}

impl Board {
    fn new() -> Self {
        Board {
            squares: Default::default(),
            past_moves: LinkedList::new(),
        }
    }

    fn place(&mut self, p: Piece, l: Location) -> Result<(), ()> {
        if self.squares[l.x as usize][l.y as usize].is_some() {
            Err(())
        } else {
            self.squares[l.x as usize][l.y as usize] = Some(p);
            Ok(())
        }
    }

    fn setup_a_rook(&mut self) -> Result<(), ()> {
        self.place(Piece::Rook(Color::White), Location { x: 0, y: 0 })
    }

    fn make_move(&mut self, m: Move) -> Result<(), FailReason> {
        println!("moving from: {:?}, to: {:?}", m.from, m.to);
        self.is_valid_move(m)?;
        println!("is valid move!");
        self.past_moves.push_front(m);
        self.do_move(m);
        Ok(())
    }

    fn is_valid_move(&self, m: Move) -> Result<(), FailReason> {
        if !m.from.is_in_bounds() || !m.to.is_in_bounds() {
            return Err(FailReason::OutOfBounds);
        }
        //TODO add special behavior for special moves like castling, en pessant, promoting
        let piece = self.get_piece_from(&m.from)?;
        let blocked = piece
            .squares_moved_over(m)?
            .iter()
            .any(|square| { self.is_blocked(m, piece, square) });


        if blocked {
            Err(FailReason::Blocked)
        } else {
            Ok(())
        }
    }

    fn is_blocked(&self, m: Move, piece: Piece, square: &Location) -> bool {
        if self.is_piece_here(&square) || *square == m.from {
            false
        } else {
            !Board::square_is_in_middle_of_path(*square, m) && self.is_opposite_color(piece, square).unwrap_or_else(|_| false)
        }
    }

    fn is_opposite_color(&self, piece: Piece, taken_square: &Location) -> Result<bool, FailReason> {
        Ok(self.get_piece_from(taken_square)?
            .color()
            .ne(piece.color()))
    }

    fn square_is_in_middle_of_path(taken_square: Location, m: Move) -> bool {
        taken_square != m.to && taken_square != m.from
    }

    fn get_piece_from(&self, square: &Location) -> Option<Piece> {
        self.squares[square.x as usize][square.y as usize]
    }

    fn is_piece_here(&self, square: &&Location) -> bool {
        self.get_piece_from(square).is_some()
    }

    fn do_move(&mut self, m: Move) {
        let Move { from, to } = m;
        {
            let piece = self.get_piece_from(&from).expect("this should really be a valid move");
            self.squares[to.x as usize][to.y as usize] = Some(piece);
        }
        self.squares[from.x as usize][from.y as usize] = None;
    }
}

#[derive(Copy, Clone)]
struct Knight;

#[derive(Copy, Clone)]
struct Pawn;

#[derive(Copy, Clone)]
struct King;

#[derive(Copy, Clone)]
struct Queen;

#[derive(Copy, Clone)]
struct Bishop;

#[derive(Copy, Clone)]
struct Rook;

impl Knight {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let diff = m.to - m.from;
        match (diff.x.abs(), diff.y.abs()) {
            (1, 2) | (2, 1) => Ok(vec!(m.to)),
            _ => Err(FailReason::ImpossibleMove)
        }
    }
}

impl Pawn {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;
        match (from - to).as_tup() {
            (0, 1) | (0, -1) => { Ok(vec![from, to]) }
            (0, y @ 2) | (0, y @ -2) => {
                if Pawn::is_in_original_position(from) {
                    Ok(vec![from, from + Location { x: 0, y: y / 2 }, to])
                } else {
                    Err(FailReason::ImpossibleMove)
                }
            }
            (x,y) => {
                if Pawn::moved_one_diagonal(x,y) {
                    Ok(vec![from, to])
                } else {
                    Err(FailReason::ImpossibleMove)
                }
            }
        }
    }

    fn is_in_original_position(from: Location) -> bool {
        from.x == 1 || from.x == 6
    }

    fn moved_one_diagonal(x: i32, y: i32) -> bool {
        x.abs() == 1 && y.abs() == 1
    }
}

impl King {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;

        match ((to - from).x.abs(), (to - from).y.abs()) {
            (1, 0) | (0, 1) | (1, 1) => { Ok(vec!(m.to)) }
            _ => Err(FailReason::ImpossibleMove)
        }
    }
}

impl Queen {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        if let Ok(rook_result) = Rook::squares_moved(m) {
            Ok(rook_result)
        } else if let Ok(bishop_result) = Bishop::squares_moved(m) {
            Ok(bishop_result)
        } else {
            Err(FailReason::ImpossibleMove)
        }
    }
}

impl Bishop {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;
        return if Bishop::is_diagonal(m) {
            Ok((from.x..=to.x).zip(from.y..=to.y).map(|(x, y)| { Location { x, y } }).collect())
        } else {
            Err(FailReason::ImpossibleMove)
        };
    }

    fn is_diagonal(m: Move) -> bool {
        let Move { from, to } = m;
        from.x - to.x == from.y - to.y
    }
}

impl Rook {
    fn squares_moved(m: Move) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;
        match to - from {
            Location { x, y: 0 } => Ok((0..=x).into_iter().map(|x| { from + Location { x, y: 0 } }).collect()),
            Location { x: 0, y } => Ok((0..=y).into_iter().map(|y| { from + Location { x: 0, y } }).collect()),
            _ => Err(FailReason::ImpossibleMove)
        }
    }
}


trait Movable {
    fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason>;
}

