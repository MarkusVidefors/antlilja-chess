use crate::{Board, Color, Move, MoveMap, PieceType, Pos, TaggedPiece};

mod moves;
mod pgn;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum GameResult {
    InvalidMove,
    Ok,
    Checkmate,
    Stalemate,
}

pub struct Game {
    board: Board,
    move_map: MoveMap,
    history: Vec<(Board, Pos, Move)>,
    player: Color,
    king_pos: Pos,
}

impl Default for Game {
    fn default() -> Self {
        let mut game = Game {
            board: Default::default(),
            move_map: MoveMap::new(),
            history: Vec::<(Board, Pos, Move)>::with_capacity(50),
            player: Color::White,
            king_pos: Pos::new_xy(4, 0),
        };

        game.calculate_all_moves();

        game
    }
}

impl Game {
    pub fn from_board(board: Board, player: Color) -> Self {
        let mut game = Self {
            board: board,
            move_map: MoveMap::new(),
            history: Vec::with_capacity(50),
            player: player,
            king_pos: board.find_king(player),
        };

        game.calculate_all_moves();

        game
    }
    pub fn switch_side(&mut self) -> bool {
        self.move_map.clear();
        self.player.flip();
        self.king_pos = self.board.find_king(self.player);
        self.calculate_all_moves()
    }

    pub fn current_color(&self) -> Color {
        self.player
    }

    pub fn moves_for_pos(&self, pos: Pos) -> Option<&[Move]> {
        self.move_map.at(pos)
    }

    pub fn print_ascii(&self) {
        self.board.print_ascii(self.player);
    }

    pub fn play_xy(&mut self, from_x: u8, from_y: u8, r#move: Move) -> GameResult {
        self.play(Pos::new_xy(from_x, from_y), r#move)
    }

    pub fn play(&mut self, from: Pos, r#move: Move) -> GameResult {
        if r#move == Move::None {
            return GameResult::InvalidMove;
        }

        assert!(from.index() < 64);
        assert!(!self.at_pos(from).is_empty());

        let moves = self.move_map.at(from);

        if moves.is_none() {
            return GameResult::InvalidMove;
        }

        let moves = moves.unwrap();

        if !moves.contains(&r#move) {
            return GameResult::InvalidMove;
        }

        self.history.push((self.board, from, r#move));
        self.board = self.board.board_after_move(from, r#move, self.player);

        if self.switch_side() {
            if self.board.pos_in_danger(self.king_pos, self.player) {
                GameResult::Checkmate
            } else {
                GameResult::Stalemate
            }
        } else {
            GameResult::Ok
        }
    }

    pub fn undo(&mut self, steps: usize) -> bool {
        if steps >= self.history.len() {
            return false;
        }

        self.board = self.history[self.history.len() - 1 - steps].0;
        self.history.truncate(self.history.len() - steps);

        if steps % 2 != 0 {
            self.player.flip();
        }

        self.king_pos = self.board.find_king(self.player);
        let _ = self.calculate_all_moves();
        true
    }

    pub fn at_xy(&self, x: u8, y: u8) -> TaggedPiece {
        self.board.at_xy(x, y)
    }

    pub fn at_pos(&self, pos: Pos) -> TaggedPiece {
        self.board.at_pos(pos)
    }

    pub fn at_index(&self, i: usize) -> TaggedPiece {
        self.board.at_index(i)
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn history(&self, steps: usize) -> &(Board, Pos, Move) {
        assert!(steps < self.history.len());
        &self.history[self.history.len() - 1 - steps]
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn history_iter(&self) -> std::slice::Iter<'_, (Board, Pos, Move)> {
        self.history.iter()
    }

    fn calculate_all_moves(&mut self) -> bool {
        let mut moves: usize = 0;
        for i in 0..64 {
            moves += self.calculate_moves_for(Pos::new_index(i));
        }

        moves == 0
    }

    fn calculate_moves_for(&mut self, pos: Pos) -> usize {
        let piece = self.at_pos(pos);

        if piece.is_empty() || piece.color() != self.player {
            return 0;
        }

        self.move_map.set_current_pos(pos);

        match piece.get_type() {
            PieceType::Pawn => {
                self.add_pawn_moves(pos);
            }
            PieceType::Rook => {
                self.add_straight_moves(pos);
            }
            PieceType::Knight => {
                self.add_knight_moves(pos);
            }
            PieceType::Bishop => {
                self.add_diagonal_moves(pos);
            }
            PieceType::Queen => {
                self.add_diagonal_moves(pos);
                self.add_straight_moves(pos);
            }
            PieceType::King => {
                self.add_king_moves(pos);
                self.add_castling_moves();
            }
        };

        self.move_map.current_pos_moves_len()
    }

    fn king_in_danger_after_move(&self, from: Pos, r#move: Move) -> bool {
        let board_after_move = self.board.board_after_move(from, r#move, self.player);
        board_after_move.pos_in_danger(self.king_pos, self.player)
    }
}