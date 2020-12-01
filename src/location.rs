pub(crate) mod location {
    use crate::chess::{FailReason, Color, Rook, Bishop};
    use std::ops::{Sub, Add};
    use crate::chess_move::chess_move::Move;

    #[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
    pub struct Location {
        pub x: i32,
        pub y: i32,
    }

    impl Location {
        pub fn new(x: i32, y: i32) -> Location {
            Location {
                x,
                y,
            }
        }

        pub fn as_tup(&self) -> (i32, i32) {
            let Location { x, y } = self;
            (*x, *y)
        }
        pub fn as_abs_tup(&self) -> (i32, i32) {
            let (x, y) = self.as_tup();
            (x.abs(), y.abs())
        }
        pub fn is_in_bounds(&self) -> bool {
            let (x, y) = self.as_tup();
            x >= 0 && x <= 7 && y >= 0 && y <= 7
        }

        pub fn locations_between(&self, dest: Location) -> Result<Vec<Location>, FailReason> {
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
}