#![feature(try_trait)]
#![feature(option_unwrap_none)]

use std::collections::LinkedList;
use std::ops::{Sub, Add};
use std::option::NoneError;

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
    board.place(rook, location).unwrap();
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
    Tester::place_and_move_legal(piece, from, to, None);
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
    Tester::place_and_move_legal(piece, from, to, None);
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
    Tester::place_and_move_legal(piece, from, to, None);
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
    Tester::place_and_move_legal(piece, from, to, None);
}

#[test]
fn test_move_pawn_illegal_1() {
    let piece = Piece::Pawn(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 0, y: 2 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_move_pawn_illegal_2() {
    let piece = Piece::Pawn(Color::Black);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 1, y: 1 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_move_pawn_legal_2() {
    let piece = Piece::Pawn(Color::White);
    let target = Piece::Pawn(Color::Black);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 1, y: 1 };
    Tester::place_target_and_piece_then_move_legal(piece, target, from, to, None);
}

#[test]
fn test_attack_pawn_legal_3() {
    let piece = Piece::Pawn(Color::Black);
    let target = Piece::Pawn(Color::White);
    let from = Location { x: 1, y: 1 };
    let to = Location { x: 0, y: 0 };
    Tester::place_target_and_piece_then_move_legal(piece, target, from, to, Some(Piece::Queen(Color::Black)));
}

#[test]
fn test_move_pawn_legal_4() {
    let piece = Piece::Pawn(Color::Black);
    let target = Piece::Pawn(Color::White);
    let from = Location { x: 5, y: 5 };
    let to = Location { x: 6, y: 4 };
    Tester::place_target_and_piece_then_move_legal(piece, target, from, to, None);
}

#[test]
fn test_promote_pawn_legal() {
    let piece = Piece::Pawn(Color::White);
    let promotion = Some(Piece::Queen(Color::White));
    let from = Location { x: 5, y: 6 };
    let to = Location { x: 5, y: 7 };
    Tester::place_and_move_legal(piece, from, to, promotion);
}

#[test]
fn test_move_pawn_illegal_4() {
    let piece = Piece::Pawn(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 1, y: 1 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_en_pessant_legal() {
    let mut board = Board::new();
    let piece = Piece::Pawn(Color::Black);
    let target = Piece::Pawn(Color::White);
    let starting_attacker = Location { x: 2, y: 3 };
    let starting_target = Location { x: 1, y: 1 };
    let ending_target = Location { x: 1, y: 3 };
    let ending_attacker = Location { x: 1, y: 2 };

    board.place(piece, starting_attacker).unwrap();
    board.place(target, starting_target).unwrap();

    board.make_move(Move::new(starting_target, ending_target)).unwrap();
    board.make_move(Move::new(starting_attacker, ending_attacker)).unwrap();

    board.get_piece_from(&ending_target).unwrap_none();
}

#[test]
fn test_en_pessant_illegal() {
    let mut board = Board::new();
    let piece = Piece::Pawn(Color::Black);
    let target = Piece::Pawn(Color::White);
    let starting_attacker = Location { x: 0, y: 3 };
    let starting_target = Location { x: 1, y: 3 };
    let ending_attacker = Location { x: 1, y: 2 };

    board.place(piece, starting_attacker).unwrap();
    board.place(target, starting_target).unwrap();

    board.make_move(Move::new(starting_attacker, ending_attacker)).unwrap_err();
}

#[test]
fn test_move_pawn_illegal_3() {
    let piece = Piece::Pawn(Color::Black);
    let target = Piece::Pawn(Color::Black);
    let from = Location { x: 1, y: 1 };
    let to = Location { x: 0, y: 0 };
    Tester::place_target_and_piece_then_move_illegal(piece, target, from, to);
}

#[test]
fn test_move_king_legal_1() {
    let piece = Piece::King(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 0, y: 1 };
    Tester::place_and_move_legal(piece, from, to, None);
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
    Tester::place_and_move_legal(piece, from, to, None);
}

#[test]
fn test_move_knight_illegal_1() {
    let piece = Piece::Knight(Color::White);
    let from = Location { x: 0, y: 0 };
    let to = Location { x: 3, y: 1 };
    Tester::place_and_move_illegal(piece, from, to);
}

#[test]
fn test_castle_king_side_legal_1() {
    let mut board = Board::new();
    let king = Piece::King(Color::White);
    let rook = Piece::Rook(Color::White);
    board.place(rook, Location { x: 7, y: 0 }).unwrap();
    board.place(king, Location { x: 4, y: 0 }).unwrap();
    board.make_move(Move::new(Location { x: 4, y: 0 }, Location { x: 6, y: 0 })).unwrap();

    match board.get_piece_from(&Location { x: 6, y: 0 }).unwrap() {
        Piece::King(_) => {}
        _ => panic!("king should be here post-castle")
    };

    match board.get_piece_from(&Location { x: 5, y: 0 }).unwrap() {
        Piece::Rook(_) => {}
        _ => panic!("rook should be here post-castle")
    };
}

#[test]
fn test_castle_king_side_illegal_1() {
    let mut board = Board::new();
    let king = Piece::King(Color::White);
    let rook = Piece::Rook(Color::White);
    board.place(rook, Location { x: 7, y: 0 }).unwrap();
    board.place(king, Location { x: 4, y: 0 }).unwrap();

    board.make_move(Move::new(Location { x: 4, y: 0 }, Location { x: 5, y: 0 })).unwrap();

    board.make_move(Move::new(Location { x: 5, y: 0 }, Location { x: 4, y: 0 })).unwrap();

    board.make_move(Move::new(Location { x: 4, y: 0 }, Location { x: 6, y: 0 })).unwrap_err();
}

#[test]
fn test_castle_queen_side_illegal_1() {
    let mut board = Board::new();
    let king = Piece::King(Color::Black);
    let rook = Piece::Rook(Color::Black);
    board.place(rook, Location { x: 0, y: 7 }).unwrap();
    board.place(king, Location { x: 4, y: 7 }).unwrap();

    board.make_move(Move::new(Location { x: 4, y: 7 }, Location { x: 4, y: 6 })).unwrap();

    board.make_move(Move::new(Location { x: 4, y: 6 }, Location { x: 4, y: 7 })).unwrap();

    board.make_move(Move::new(Location { x: 4, y: 6 }, Location { x: 2, y: 6 })).unwrap_err();
}

#[test]
fn test_castle_queen_side_legal_1() {
    let mut board = Board::new();
    let king = Piece::King(Color::Black);
    let rook = Piece::Rook(Color::Black);
    board.place(rook, Location { x: 0, y: 7 }).unwrap();
    board.place(king, Location { x: 4, y: 7 }).unwrap();
    board.make_move(Move::new(Location { x: 4, y: 7 }, Location { x: 2, y: 7 })).expect("should be a valid castle");

    match board.get_piece_from(&Location { x: 2, y: 7 }).expect("king should be here") {
        Piece::King(_) => {}
        _ => panic!("king should be here post-castle")
    };

    match board.get_piece_from(&Location { x: 3, y: 7 }).expect("rook should be here ") {
        Piece::Rook(_) => {}
        _ => panic!("rook should be here post-castle")
    };
}

#[test]
fn test_stay_in_check() {
    let mut board = Board::new();
    let king = Piece::King(Color::Black);
    let rook = Piece::Rook(Color::White);
    board.place(king, Location::new(0, 0)).unwrap();
    board.place(rook, Location::new(1, 1)).unwrap();
    board.make_move(Move::new(Location::new(0, 0), Location::new(0, 1))).unwrap_err();
}

#[test]
fn test_move_into_check() {
    let mut board = Board::new();
    let king = Piece::King(Color::Black);
    let rook = Piece::Rook(Color::White);
    board.place(king, Location::new(0, 0)).unwrap();
    board.place(rook, Location::new(1, 1)).unwrap();
    board.make_move(Move::new(Location::new(0, 0), Location::new(0, 1))).unwrap_err();
}

struct Tester;

impl Tester {
    fn place_and_move_legal(piece: Piece, from: Location, to: Location, promotion: Option<Piece>) {
        let mut board = Board::new();
        board.place(piece, from).unwrap();
        if let Some(promotion) = promotion {
            board.make_move(Move::new_with_piece(from, to, promotion)).unwrap();
            assert_eq!(board.get_piece_from(&to).unwrap(), promotion);
        } else {
            board.make_move(Move::new(from, to)).unwrap();
            board.get_piece_from(&to).unwrap();
        }
    }

    fn place_and_move_illegal(piece: Piece, from: Location, to: Location) {
        let mut board = Board::new();
        board.place(piece, from).unwrap();
        board.make_move(Move::new(from, to)).unwrap_err();
    }

    fn squares_moved(piece: Piece, from: Location, to: Location, expected: Vec<Location>) {
        let mut board = Board::new();
        board.place(piece, from).unwrap();
        let mut expected_sorted = expected.clone();
        expected_sorted.sort();

        let mut result_sorted = board.get_piece_from(&from)
            .unwrap()
            .squares_moved_over(Move::new(from, to))
            .unwrap();

        result_sorted.sort();

        assert_eq!(expected_sorted, result_sorted)
    }

    fn place_target_and_piece_then_move_legal(piece: Piece, target: Piece, from: Location, to: Location, promotion: Option<Piece>) {
        let mut board = Board::new();
        board.place(piece, from).unwrap();
        board.place(target, to).unwrap();
        board.make_move(Move::new_with_opt_piece(from, to, promotion)).unwrap();
        board.get_piece_from(&to).unwrap();
    }

    fn place_target_and_piece_then_move_illegal(piece: Piece, target: Piece, from: Location, to: Location) {
        let mut board = Board::new();
        board.place(piece, from).unwrap();
        board.place(target, to).unwrap();
        board.make_move(Move::new(from, to)).unwrap_err();
    }
}


struct Board {
    squares: [[Option<Piece>; 8]; 8],
    past_moves: LinkedList<(Move, Piece)>,
}

#[derive(Copy, Clone, Debug)]
struct Move {
    from: Location,
    to: Location,
    promoted: Option<Piece>,
}




impl Move {
    fn new(from: Location, to: Location) -> Self {
        Move {
            from,
            to,
            promoted: None,
        }
    }

    fn new_with_opt_piece(from: Location, to: Location, piece: Option<Piece>) -> Self {
        Move {
            from,
            to,
            promoted: piece,
        }
    }

    fn new_with_piece(from: Location, to: Location, piece: Piece) -> Self {
        Move {
            from,
            to,
            promoted: Some(piece),
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Location {
        Location {
            x,
            y,
        }
    }

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

    fn locations_between(&self, dest: Location) -> Result<Vec<Location>, FailReason> {
        let m = Move::new(self.clone(), dest);
        if let Ok(rook_result) = Rook::squares_moved(m, &Color::White) {
            Ok(rook_result)
        } else if let Ok(bishop_result) = Bishop::squares_moved(m, &Color::White) {
            Ok(bishop_result)
        } else {
            Err(FailReason::ImpossibleMove(String::from("invalid call to locations_between, locations_between can only do queen-like moves")))
        }
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
    Rook(Color),
    Knight(Color),
    Pawn(Color),
    King(Color),
    Queen(Color),
    Bishop(Color),
}

#[derive(Copy, Clone, PartialEq, Debug)]
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

#[derive(Debug, PartialOrd, PartialEq)]
enum FailReason {
    // refactor to &str
    ImpossibleMove(String),
    NoPieceHere(String),
    Blocked(String),
    OutOfBounds(String),
    NeedPromotion(String),
    Checked(String),
}


impl From<NoneError> for FailReason {
    fn from(_: NoneError) -> Self {
        FailReason::NoPieceHere(String::from("no piece found when trying to unwrap a value from the board"))
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

    fn make_move(&mut self, m: Move) -> Result<(), FailReason> {
        self.is_valid_move(m)?;
        let moving_peice = self.get_piece_from(&m.from).expect("should be a piece here after is_valid_move call");
        self.past_moves.push_front((m, moving_peice));
        let color = self.get_piece_from(&m.from).unwrap().color().clone();
        self.do_move(m);
        if self.is_in_check(&color) {
            self.undo_last_move();
            return Err(FailReason::Checked(String::from("your in check mate")));
        }
        Ok(())
    }

    fn is_valid_move(&self, m: Move) -> Result<(), FailReason> {
        Board::do_bounds_check(m)?;
        let piece = self.get_piece_from(&m.from)?;
        self.do_piece_specific_checks(&m, piece)?;
        let blocked = piece.squares_moved_over(m)?.iter().any(|square| { self.is_blocked(m, piece, square) });


        if blocked {
            Err(FailReason::Blocked(String::from("despite the move being valid, the piece was blocked midway through or by its own peice at the goal position")))
        } else {
            Ok(())
        }
    }

    fn do_piece_specific_checks(&self, m: &Move, piece: Piece) -> Result<(), FailReason> {
        match piece {
            Piece::Pawn(c) => self.is_valid_pawn_move(&m, &c),
            Piece::King(_) => self.is_valid_king_move(&m),
            _ => Ok(())
        }
    }

    fn is_promotion(m: Move) -> bool {
        m.to.y == 0 || m.to.y == 7
    }

    fn is_valid_pawn_move(&self, m: &Move, c: &Color) -> Result<(), FailReason> {
        if Board::is_promotion(*m) && m.promoted.is_none() {
            println!("1");
            return Err(FailReason::NeedPromotion(String::from("the pawn moved to the last row, but we dont know what you want to promote it to")));
        } else if Pawn::is_attacking_validly(*m, &c) {
            println!("2");
            let target = match self.get_piece_from(&m.to) {
                None => return self.check_en_passant(&m),
                Some(target) => target
            };
            println!("hello! {:?}", target);
            match self.is_opposite_color(target, &m.from) {
                Ok(is_opp) if is_opp => Ok(()),
                Ok(is_opp) if !is_opp => Err(FailReason::ImpossibleMove(String::from("not a valid move for a pawn, you've tried moving diagonally into your own piece"))),
                Err(err) => Err(err),
                _ => unreachable!("covered both true and false cases for Ok and all Err")
            }
        } else {
            println!("3");
            Ok(())
        }
    }

    fn check_en_passant(&self, m: &&Move) -> Result<(), FailReason> {
        match self.past_moves.back() {
            None => { Err(FailReason::ImpossibleMove(String::from("can only en passant a pawn that just moved"))) }
            Some((last_move, _)) => {
                if (last_move.to - last_move.from).as_abs_tup() == (0, 2) && self.is_opposite_color(self.get_piece_from(&last_move.to)?, &m.from)? {
                    match self.get_piece_from(&last_move.to) {
                        None => { panic!("there should be a piece here") }
                        Some(p) => {
                            match p {
                                Piece::Pawn(_) => { Ok(()) }
                                _ => Err(FailReason::ImpossibleMove(String::from("piece we're trying to en passant is not a pawn")))
                            }
                        }
                    }
                } else {
                    Err(FailReason::ImpossibleMove(String::from("last move was not a opposite colored pawn moving two spaces forward")))
                }
            }
        }
    }

    fn do_bounds_check(m: Move) -> Result<(), FailReason> {
        if !m.from.is_in_bounds() || !m.to.is_in_bounds() {
            return Err(FailReason::OutOfBounds(String::from("invalid move, ranges for location are 0-7 inclusive")));
        } else {
            Ok(())
        }
    }

    fn is_blocked(&self, m: Move, piece: Piece, square: &Location) -> bool {
        if Board::square_is_in_middle_of_path(*square, m) {
            // println!("square is in middle of path");
            if self.get_piece_from(square).is_some() {
                // println!("square contains piece, BLOCKED");
                true
            } else {
                // println!("square does not contain piece");
                false
            }
        } else if self.get_piece_from(square).is_some() {
            // println!("square is not in middle of path and contains something");
            if m.from == *square {
                // println!("lol its just me");
                false
            } else if m.to == *square {
                // println!("we're attacking this bad boy now");
                if self.is_opposite_color(piece, square).unwrap() {
                    // println!("we can attack as we are different colours");
                    false
                } else {
                    // println!("we can not attack as we are the same color, BLOCKED");
                    true
                }
            } else {
                panic!("should not be possible")
            }
        } else {
            // println!("square is not in middle of path and nothing is here");
            false
        }
    }

    fn is_opposite_color(&self, piece: Piece, taken_square: &Location) -> Result<bool, FailReason> {
        if let Some(other) = self.get_piece_from(taken_square) {
            println!("{:?}", other);
            Ok(other.color().ne(piece.color()))
        } else {
            Err(FailReason::NoPieceHere(String::from("no piece where we are trying to attack")))
        }
    }

    fn square_is_in_middle_of_path(taken_square: Location, m: Move) -> bool {
        taken_square != m.to && taken_square != m.from
    }

    fn get_piece_from(&self, square: &Location) -> Option<Piece> {
        if self.squares[square.x as usize][square.y as usize].is_none() { println!("didnt find something at {:?}", square) };
        self.squares[square.x as usize][square.y as usize]
    }

    fn do_move(&mut self, m: Move) {
        // this should always be done after an isvalid call, this function trusts the move is valid and executes the move no matter how dumb is it
        if King::is_castling(&m) {
            self.do_move(King::get_rooks_move_for_castle(&m).expect("as we're in do_move, I can huck anything"))
        } else if self.check_en_passant(&&m).is_ok() {
            let target = Pawn::get_en_passant_target(&m);
            self.squares[target.x as usize][target.y as usize] = None;
        }


        let Move { from, to, promoted: promotion, .. } = m;
        {
            let piece = self.get_piece_from(&from).expect("this should really be a valid move");
            if let Some(promotion) = promotion {
                self.squares[to.x as usize][to.y as usize] = Some(promotion);
            } else {
                self.squares[to.x as usize][to.y as usize] = Some(piece);
            }
        }
        self.squares[from.x as usize][from.y as usize] = None;
    }

    fn is_valid_king_move(&self, m: &Move) -> Result<(), FailReason> {
        if King::is_castling(m) {
            if self.past_moves.iter().any(|(past_move, _)| { past_move.to == m.from || past_move.from == m.from }) {
                return Err(FailReason::ImpossibleMove(String::from("cannot castle, king has already moved")));
            } else {
                let rook_location = King::get_rooks_location_for_castle(m).expect("already checked king was castling");
                if self.past_moves.iter().any(|(past_move, _)| { past_move.from == rook_location || past_move.to == rook_location }) {
                    return Err(FailReason::ImpossibleMove(String::from("cannot castle, rook has already moved")));
                }
            }
        }
        Ok(())
    }

    fn undo_last_move(&mut self) {
        let (last_move, _) = self.past_moves.pop_front().expect("if we're undoing moves, there should have been one prior");
        self.do_move(Move::new(last_move.to, last_move.from));
        let (_, taken_piece) = self.past_moves.iter().find(|(m, p)| {m.to == last_move.to});
        self.place(taken_piece, last_move.to);
    }

    fn is_in_check(&self, _c: &Color) -> bool {
        false
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
            _ => Err(FailReason::ImpossibleMove(String::from("invalid move for a knight")))
        }
    }
}

impl Pawn {
    fn squares_moved(m: Move, c: &Color) -> Result<Vec<Location>, FailReason> {
        let Move { from, to, .. } = m;
        match (from - to).as_tup() {
            (0, 1) | (0, -1) => {
                if Pawn::is_right_direction(m, c) {
                    Ok(to.locations_between(from).unwrap())
                } else {
                    Err(FailReason::ImpossibleMove(String::from("invalid move for a pawn, must move forward")))
                }
            }
            (0, 2) | (0, -2) => {
                if Pawn::is_in_original_position(from, c) {
                    Ok(to.locations_between(from).unwrap())
                } else {
                    Err(FailReason::ImpossibleMove(String::from("invalid move for a pawn, cannot move two forward if already moved")))
                }
            }
            (x, y) => {
                if Pawn::moved_one_diagonal(x, y) && Pawn::is_attacking_validly(m, c) {
                    Ok(to.locations_between(from).unwrap())
                } else {
                    Err(FailReason::ImpossibleMove(String::from("invalid move for a pawn")))
                }
            }
        }
    }

    fn get_en_passant_target(pawn_move: &Move) -> Location {
        Location::new(pawn_move.to.x, pawn_move.from.y)
    }

    fn is_attacking_validly(m: Move, c: &Color) -> bool {
        let Location { x: diff_x, y: diff_y } = m.from - m.to;

        diff_x.abs() == 1 && diff_y.abs() == 1 && match c {
            Color::White => diff_y == -1,
            Color::Black => diff_y == 1,
        }
    }


    fn is_right_direction(m: Move, color: &Color) -> bool {
        let Location { x: _, y } = m.from - m.to;
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
        let Move { from, to, .. } = m;

        match ((to - from).x.abs(), (to - from).y.abs()) {
            (1, 0) | (0, 1) | (1, 1) => Ok(to.locations_between(from).expect("this is a valid queen move and thus should be a valid king move as moves available to king are a subset of moves available to queen")),
            (2, 0) => Ok(to.locations_between(from).expect("this is a valid queen move and thus should be a valid king move as moves available to king are a subset of moves available to queen")),
            _ => Err(FailReason::ImpossibleMove(String::from("invalid move for a king")))
        }
    }

    fn is_castling(m: &Move) -> bool {
        match (m.to.as_tup(), m.from.as_tup()) {
            ((2, _), (4, _)) => true,
            ((6, _), (4, _)) => true,
            _ => false
        }
    }

    fn get_rooks_move_for_castle(kings_move: &Move) -> Result<Move, FailReason> {
        let from = King::get_rooks_location_for_castle(kings_move)?;

        let to = match kings_move.to.as_tup() {
            (2, y) => Location { x: 3, y },
            (6, y) => Location { x: 5, y },
            _ => unreachable!("as we're castling, it should be one of these")
        };
        Ok(Move::new(from, to))
    }

    fn get_rooks_location_for_castle(kings_move: &Move) -> Result<Location, FailReason> {
        assert!(King::is_castling(kings_move), "called get_rooks_location_for_castle when not castling");
        match kings_move.to.as_tup() {
            (2, y) => Ok(Location { x: 0, y }),
            (6, y) => Ok(Location { x: 7, y }),
            _ => unreachable!(format!("as we're castling, it should be one of these, instead the kings destination is {:?}", kings_move.to))
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
            Err(FailReason::ImpossibleMove(String::from("invalid move for a queen")))
        }
    }
}

impl Bishop {
    fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
        let Move { from, to, .. } = m;
        return if Bishop::is_diagonal(m) {
            let moves: Vec<(i32, i32)> = match from - to {
                loc if loc.x > 0 && loc.y > 0 => {
                    ((to.x..=from.x).rev())
                        .zip((to.x..=from.x).rev())
                        .collect()
                }
                loc if loc.x > 0 && loc.y < 0 => {
                    (to.x..=from.x).rev()
                        .zip(from.y..=to.y)
                        .collect()
                }
                loc if loc.x < 0 && loc.y > 0 => {
                    (from.x..=to.x)
                        .zip((to.y..=from.y).rev())
                        .collect()
                }
                loc if loc.x < 0 && loc.y < 0 => {
                    (from.x..=to.x)
                        .zip(from.y..=to.y)
                        .collect()
                }
                _ => unreachable!("from-to should not be 0 at x or y")
            };
            Ok(moves.iter().map(|(x, y)| { Location { x: *x, y: *y } }).collect())
        } else {
            Err(FailReason::ImpossibleMove(String::from("invalid move for a bishop")))
        };
    }

    fn is_diagonal(m: Move) -> bool {
        let Move { from, to, .. } = m;
        (from.x - to.x).abs() == (from.y - to.y).abs()
    }
}

impl Rook {
    fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
        let Move { from, to, .. } = m;
        match to - from {
            Location { x, y: 0 } => Ok((0..=x).into_iter().map(|x| { from + Location { x, y: 0 } }).collect()),
            Location { x: 0, y } => Ok((0..=y).into_iter().map(|y| { from + Location { x: 0, y } }).collect()),
            _ => Err(FailReason::ImpossibleMove(String::from("invalid move for a rook")))
        }
    }
}


trait Movable {
    fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason>;
}

