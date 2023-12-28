use crate::ChessBoard;

pub mod random_bot;
pub mod naive_bot;

type ChessBot = fn(&ChessBoard) -> i64;

pub struct SyncChessMover {
    make_move: ChessBot,
    board: *mut ChessBoard
}

impl SyncChessMover {
    pub fn new(bot: ChessBot, cb: *mut ChessBoard) -> Self {
        SyncChessMover {
            make_move: bot,
            board: cb
        }
    }
    /**
     * lower 32 bit is the movement, and higher 32 bit is the evaluation.
     */
    pub fn sync_make_move(&mut self) -> i64 {
        (self.make_move)(unsafe {&mut (*self.board).clone()})
    }
}

