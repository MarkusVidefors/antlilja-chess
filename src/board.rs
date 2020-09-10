use crate::piece::{Color, ColoredPiece, PieceType};
use crate::pos::Pos;
use crate::r#move::Move;

pub struct Board([ColoredPiece; 64]);

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            0: [ColoredPiece::empty(); 64],
        };

        for i in 0..8 {
            board.0[8 + i] = ColoredPiece::new(PieceType::Pawn, Color::White);
            board.0[8 * 6 + i] = ColoredPiece::new(PieceType::Pawn, Color::Black);
        }

        let mut place_at_both_sides = |offset, r#type| {
            board.0[offset] = ColoredPiece::new(r#type, Color::White);
            board.0[8 * 7 + offset] = ColoredPiece::new(r#type, Color::Black);
        };

        let mut place_matching_at_both_sides = |offset, r#type| {
            place_at_both_sides(offset, r#type);
            place_at_both_sides(7 - offset, r#type);
        };

        place_matching_at_both_sides(0, PieceType::Rook);
        place_matching_at_both_sides(1, PieceType::Knight);
        place_matching_at_both_sides(2, PieceType::Bishop);
        place_at_both_sides(3, PieceType::Queen);
        place_at_both_sides(4, PieceType::King);

        return board;
    }

    pub fn at(&self, x: u8, y: u8) -> ColoredPiece {
        return self.0[(y * 8 + x) as usize];
    }

    pub fn at_index(&self, i: u8) -> ColoredPiece {
        return self.0[i as usize];
    }
    pub fn get_moves_for(&self, buffer: &mut Vec<Move>, x: u8, y: u8) -> usize {
        let piece = self.at(x, y);

        match piece.get_type() {
            PieceType::Pawn => {
                return self.add_pawn_moves(buffer, piece.get_color(), x, y);
            }
            PieceType::Rook => {
                return self.add_rook_moves(buffer, piece.get_color(), x, y);
            }
            _ => {
                return 0;
            }
        }
    }

    fn add_pawn_moves(&self, buffer: &mut Vec<Move>, color: Color, x: u8, y: u8) -> usize {
        if y == 0 || y == 7 {
            return 0;
        }

        let from = Pos::from_xy(x, y);

        let mut count: usize = 0;
        let mut add_pawn_move = |to| {
            buffer.push(Move::Move(from, to));
            count += 1;
        };

        let dir: i8 = if color == Color::White { 1 } else { -1 };

        let y_forward = (y as i8 + dir) as u8;
        if self.at(x, y_forward).is_empty() {
            add_pawn_move(Pos::from_xy(x, y_forward));

            let y_off = y as i8 + dir * 2;
            if (0..8).contains(&y_off) {
                let y_off = y_off as u8;

                if (y == 1 || y == 6) && self.at(x, y_off).is_empty() {
                    add_pawn_move(Pos::from_xy(x, y_off));
                }
            }
        }

        let mut add_pawn_take = |x: u8, y: u8| {
            let space = self.at(x, y);
            if !space.is_empty() && space.get_color() != color {
                add_pawn_move(Pos::from_xy(x, y));
            }
        };

        if x != 7 {
            add_pawn_take(x + 1, y_forward);
        }

        if x != 0 {
            add_pawn_take(x - 1, y_forward);
        }

        return count;
    }

    fn add_rook_moves(&self, buffer: &mut Vec<Move>, color: Color, x: u8, y: u8) -> usize {
        let from = Pos::from_xy(x, y);

        let mut count = 0;
        let mut loop_internal = |x, y| {
            let i = y * 8 + x;
            let space = self.0[i as usize];
            if space.is_empty() || space.get_color() != color {
                buffer.push(Move::Move(from, Pos::from_xy(x, y)));
                count += 1;
            }

            return space.is_empty();
        };

        for x in (x + 1)..8 {
            if !loop_internal(x, y) {
                break;
            }
        }

        for x in x..0 {
            if !loop_internal(x, y) {
                break;
            }
        }

        for y in (y + 1)..8 {
            if !loop_internal(x, y) {
                break;
            }
        }

        for y in y..0 {
            if !loop_internal(x, y) {
                break;
            }
        }

        return count;
    }

    pub fn print(&self) {
        for i in 0..8 {
            let start = i * 8;
            let end = start + 8;
            println!("{:?}", &self.0[start..end]);
        }
    }
}
