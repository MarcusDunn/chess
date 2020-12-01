pub(crate) mod chess_move {
    use crate::location::location::Location;
    use crate::chess::Piece;

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
}
