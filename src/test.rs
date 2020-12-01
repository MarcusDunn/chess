mod test {
    use crate::board::board::Board;
    use crate::chess::{Piece, Color, Movable};
    use crate::location::location::Location;
    use crate::chess_move::chess_move::Move;

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
}