#![feature(try_trait)]
#![feature(option_unwrap_none)]

mod test;
mod location;
mod board;

mod chess {

    use std::collections::LinkedList;
    use std::option::NoneError;

    use crate::location::location::Location;

    #[derive(Copy, Clone, Debug)]
    pub struct Move {
        pub from: Location,
        pub to: Location,
        pub promoted: Option<Piece>,
    }


    impl Move {
        pub fn new(from: Location, to: Location) -> Self {
            Move {
                from,
                to,
                promoted: None,
            }
        }

        pub fn new_with_opt_piece(from: Location, to: Location, piece: Option<Piece>) -> Self {
            Move {
                from,
                to,
                promoted: piece,
            }
        }

        pub fn new_with_piece(from: Location, to: Location, piece: Piece) -> Self {
            Move {
                from,
                to,
                promoted: Some(piece),
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Piece {
        Rook(Color),
        Knight(Color),
        Pawn(Color),
        King(Color),
        Queen(Color),
        Bishop(Color),
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Color {
        White,
        Black,
    }

    impl Piece {
        pub fn color(&self) -> &Color {
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
    pub enum FailReason {
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

    #[derive(Copy, Clone)]
    pub struct Knight;

    #[derive(Copy, Clone)]
    pub struct Pawn;

    #[derive(Copy, Clone)]
    pub struct King;

    #[derive(Copy, Clone)]
    pub struct Queen;

    #[derive(Copy, Clone)]
    pub struct Bishop;

    #[derive(Copy, Clone)]
    pub struct Rook;

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

        pub fn get_en_passant_target(pawn_move: &Move) -> Location {
            Location::new(pawn_move.to.x, pawn_move.from.y)
        }

        pub fn is_attacking_validly(m: Move, c: &Color) -> bool {
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
                (1, 0) | (0, 1) | (1, 1) => Ok(to.locations_between(from)
                    .expect("this is a valid queen move and thus should be a valid king move as \
                 moves available to king are a subset of moves available to queen")),
                (2, 0) => Ok(to.locations_between(from)
                    .expect("this is a valid queen move and thus should be a valid king move as \
                moves available to king are a subset of moves available to queen")),
                _ => Err(FailReason::ImpossibleMove(String::from("invalid move for a king")))
            }
        }

        pub fn is_castling(m: &Move) -> bool {
            match (m.to.as_tup(), m.from.as_tup()) {
                ((2, _), (4, _)) => true,
                ((6, _), (4, _)) => true,
                _ => false
            }
        }

        pub fn get_rooks_move_for_castle(kings_move: &Move) -> Result<Move, FailReason> {
            let from = King::get_rooks_location_for_castle(kings_move)?;

            let to = match kings_move.to.as_tup() {
                (2, y) => Location { x: 3, y },
                (6, y) => Location { x: 5, y },
                _ => unreachable!("as we're castling, it should be one of these")
            };
            Ok(Move::new(from, to))
        }

        pub fn get_rooks_location_for_castle(kings_move: &Move) -> Result<Location, FailReason> {
            assert!(King::is_castling(kings_move), "called get_rooks_location_for_castle when not castling");
            match kings_move.to.as_tup() {
                (2, y) => Ok(Location { x: 0, y }),
                (6, y) => Ok(Location { x: 7, y }),
                _ => unreachable!(
                    format!("as we're castling, it should be one of these, instead the \
             kings destination is {:?}", kings_move.to)
                )
            }
        }
    }

    impl Queen {
        fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
            m.to.locations_between(m.from)
        }
    }

    impl Bishop {
        pub(crate) fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
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
        pub(crate) fn squares_moved(m: Move, _c: &Color) -> Result<Vec<Location>, FailReason> {
            let Move { from, to, .. } = m;
            match to - from {
                Location { x, y: 0 } => Ok((0..=x).into_iter().map(|x| { from + Location { x, y: 0 } }).collect()),
                Location { x: 0, y } => Ok((0..=y).into_iter().map(|y| { from + Location { x: 0, y } }).collect()),
                _ => Err(FailReason::ImpossibleMove(String::from("invalid move for a rook")))
            }
        }
    }


    pub trait Movable {
        fn squares_moved_over(&self, m: Move) -> Result<Vec<Location>, FailReason>;
    }
}