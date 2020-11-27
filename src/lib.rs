#![feature(try_trait)]

use std::collections::LinkedList;
use std::ops::{Sub, Add, Deref, Not};
use std::borrow::Borrow;
use std::option::NoneError;
use std::cmp::max;

#[test]
fn test_create_game() {
    Board::new();
}

#[test]
fn test_create_game_setup_a_rook() {
    let mut board = Board::new();
    board.place(
        Piece::Rook(Color::White),
        Location { x: 0, y: 0 },
    ).unwrap();
}

#[test]
fn test_get_a_rook_from_board() {
    let mut board = Board::new();
    let rook = Piece::Rook(Color::White);
    let location = Location { x: 0, y: 0 };
    board.place(rook, location);
    board.get_piece_from(&location).unwrap();
}

#[test]
fn test_squares_moved_rook() {
    let piece = Piece::Rook(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 0, y: 7 };
    let expected = vec![
        Location { x: 0, y: 0 },
        Location { x: 0, y: 1 },
        Location { x: 0, y: 2 },
        Location { x: 0, y: 3 },
        Location { x: 0, y: 4 },
        Location { x: 0, y: 5 },
        Location { x: 0, y: 6 },
        Location { x: 0, y: 7 }
    ];
    Tester::squares_moved(piece, from, to, expected)
}

#[test]
fn test_squares_moved_knight() {
    let piece = Piece::Knight(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 1, y: 2 };
    let expected = vec![
        Location { x: 0, y: 0 },
        Location { x: 1, y: 2 },
    ];
    Tester::squares_moved(piece, from, to, expected)
}

#[test]
fn test_squares_moved_pawn() {
    let piece = Piece::Pawn(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 1, y: 1 };
    let expected = vec![
        Location { x: 0, y: 0 },
        Location { x: 1, y: 1 },
    ];
    Tester::squares_moved(piece, from, to, expected)
}


#[test]
fn test_squares_moved_queen() {
    let piece = Piece::Queen(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 5, y: 5 };
    let expected = vec![
        Location { x: 0, y: 0 },
        Location { x: 1, y: 1 },
        Location { x: 2, y: 2 },
        Location { x: 3, y: 3 },
        Location { x: 4, y: 4 },
        Location { x: 5, y: 5 },
    ];
    Tester::squares_moved(piece, from, to, expected)
}

#[test]
fn test_squares_moved_bishop() {
    let piece = Piece::Bishop(Color::White);
    let from = Location { x: 5, y: 5 };
    let to = Location { x: 0, y: 0 };
    let expected = vec![
        Location { x: 5, y: 5 },
        Location { x: 4, y: 4 },
        Location { x: 3, y: 3 },
        Location { x: 2, y: 2 },
        Location { x: 1, y: 1 },
        Location { x: 0, y: 0 },
    ];
    Tester::squares_moved(piece, from, to, expected)
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
    fn place_and_move_legal(piece: Piece, from: Location, to: Location) {
        let mut board = Board::new();
        board.place(piece, from).unwrap();
        board.make_move(Move { from, to }).unwrap();
        board.get_piece_from(&to).unwrap();
    }

    fn place_and_move_illegal(piece: Piece, from: Location, to: Location) {
        let mut board = Board::new();
        board.place(piece, from).unwrap();
        board.make_move(Move { from, to }).unwrap_err();
    }

    fn squares_moved(piece: Piece, from: Location, to: Location, expected: Vec<Location>) {
        let mut board = Board::new();
        board.place(piece, from).unwrap();
        assert_eq!(
            expected,
            board.get_piece_from(&from)
                .unwrap()
                .squares_moved_over(Move { from, to })
                .unwrap()
        )
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
            Piece::Rook(c) => { Rook::squares_moved(m, c) }
            Piece::Knight(c) => { Knight::squares_moved(m, c) }
            Piece::Pawn(c) => { Pawn::squares_moved(m, c) }
            Piece::King(c) => { King::squares_moved(m, c) }
            Piece::Queen(c) => { Queen::squares_moved(m, c) }
            Piece::Bishop(c) => { Bishop::squares_moved(m, c) }
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

    fn place(&mut self, piece: Piece, location: Location) -> Result<(), ()> {
        if self.squares[location.x as usize][location.y as usize].is_some() {
            Err(())
        } else {
            self.squares[location.x as usize][location.y as usize] = Some(piece);
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
    fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
        let diff = m.to - m.from;
        match (diff.x.abs(), diff.y.abs()) {
            (1, 2) | (2, 1) => Ok(vec!(m.from, m.to)),
            _ => Err(FailReason::ImpossibleMove)
        }
    }
}

impl Pawn {
    fn squares_moved(m: Move, c: &Color) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;
        match (from - to).as_tup() {
            (0, y @ 1) | (0, y @ -1) => {
                if Pawn::is_right_direction(y, c) {
                    Ok(vec![from, to])
                } else {
                    Err(FailReason::ImpossibleMove)
                }
            }
            (0, y @ 2) | (0, y @ -2) => {
                if Pawn::is_in_original_position(from, c) {
                    Ok(vec![from, from + Location { x: 0, y: y / 2 }, to])
                } else {
                    Err(FailReason::ImpossibleMove)
                }
            }
            (x, y) => {
                if Pawn::moved_one_diagonal(x, y) {
                    Ok(vec![from, to])
                } else {
                    Err(FailReason::ImpossibleMove)
                }
            }
        }
    }

    fn is_right_direction(y: i32, color: &Color) -> bool {
        match color {
            Color::White => y == -1,
            Color::Black => y == 1,
        }
    }

    fn is_in_original_position(from: Location, color: &Color) -> bool {
        match color {
            Color::White => from.y == 1,
            Color::Black => from.y == 6,
        }
    }

    fn moved_one_diagonal(x: i32, y: i32) -> bool {
        x.abs() == 1 && y.abs() == 1
    }
}

impl King {
    fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;

        match ((to - from).x.abs(), (to - from).y.abs()) {
            (1, 0) | (0, 1) | (1, 1) => { Ok(vec!(m.to)) }
            _ => Err(FailReason::ImpossibleMove)
        }
    }
}

impl Queen {
    fn squares_moved(m: Move, c: &Color) -> Result<Vec<Location>, FailReason> {
        if let Ok(rook_result) = Rook::squares_moved(m, c) {
            Ok(rook_result)
        } else if let Ok(bishop_result) = Bishop::squares_moved(m, c) {
            Ok(bishop_result)
        } else {
            Err(FailReason::ImpossibleMove)
        }
    }
}

impl Bishop {
    fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
        let Move { from, to } = m;
        return if Bishop::is_diagonal(m) {
            Ok(
                match from - to {
                    loc if loc.x > 0 && loc.y > 0 => {
                        ((to.x..=from.x).rev()).zip((to.x..=from.x).rev())
                    }
                    loc if loc.x > 0 && loc.y < 0 => {
                        (to.x..=from.x).rev().zip(from.y..=to.y)
                    }
                    loc if loc.x < 0 && loc.y > 0 => {
                        (from.x..=to.x).zip((to.y..=from.y).rev())
                    }
                    loc if loc.x < 0 && loc.y < 0 => {
                        (from.x..=to.x).zip((from.y..=to.y))
                    }
                    _ => unreachable!("from-to should not be 0 at x or y")
                }
                    .map(|(x, y)| { Location { x: *x, y: *y } }).collect()
            )
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
    fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
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

