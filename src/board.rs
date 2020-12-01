pub mod board {
    use crate::chess::{FailReason, Piece, Movable, Color, Pawn, King};
    use crate::location::location::Location;
    use std::collections::LinkedList;
    use crate::chess_move::chess_move::Move;

    pub struct Board {
        squares: [[Option<Piece>; 8]; 8],
        past_moves: LinkedList<(Move, Piece)>,
    }


    impl Board {
        pub fn new() -> Self {
            Board {
                squares: Default::default(),
                past_moves: LinkedList::new(),
            }
        }

        pub fn place(&mut self, piece: Piece, location: Location) -> Result<(), ()> {
            if self.squares[location.x as usize][location.y as usize].is_some() {
                Err(())
            } else {
                self.squares[location.x as usize][location.y as usize] = Some(piece);
                Ok(())
            }
        }

        pub fn make_move(&mut self, m: Move) -> Result<(), FailReason> {
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
                return Err(FailReason::NeedPromotion(String::from("the pawn moved to the last row, but we dont know what you want to promote it to")));
            } else if Pawn::is_attacking_validly(*m, &c) {
                let target = match self.get_piece_from(&m.to) {
                    None => return self.check_en_passant(&m),
                    Some(target) => target
                };
                match self.is_opposite_color(target, &m.from) {
                    Ok(is_opp) if is_opp => Ok(()),
                    Ok(is_opp) if !is_opp => Err(FailReason::ImpossibleMove(String::from("not a valid move for a pawn, you've tried moving diagonally into your own piece"))),
                    Err(err) => Err(err),
                    _ => unreachable!("covered both true and false cases for Ok and all Err")
                }
            } else {
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

        pub fn get_piece_from(&self, square: &Location) -> Option<Piece> {
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

        pub fn undo_last_move(&mut self) {
            let (last_move, _) = self.past_moves.pop_front().expect("if we're undoing moves, there should have been one prior");
            self.do_move(Move::new(last_move.to, last_move.from));
            if let Some((_, taken_piece)) = self.past_moves.iter().find(|(m, _)| { m.to == last_move.to }) {
                self.place(*taken_piece, last_move.to).expect("just moved a piece out of this position");
            }
        }

        fn is_in_check(&self, c: &Color) -> bool {
            if let Some(king_pos) = self.find_king(*c) {
                for x in 0..8 {
                    for y in 0..8 {
                        if self.get_piece_from(&Location::new(x, y)).unwrap_or(Piece::Rook(*c)).color() != c && self.is_valid_move(Move::new(Location::new(x, y), king_pos)).is_ok()
                        {
                            return true;
                        } else {
                            continue;
                        };
                    };
                };
                return false;
            } else {
                false // for testing
            }
        }

        fn find_king(&self, color: Color) -> Option<Location> {
            for x in 0..8 {
                for y in 0..8 {
                    if let Some(Piece::King(king_color)) = self.get_piece_from(&Location::new(x, y)) {
                        if color == king_color {
                            return Some(Location { x, y });
                        } else {
                            continue;
                        };
                    } else {
                        continue;
                    };
                };
            };
            None
        }
    }
}