use crate::chessboard::ChessBoard;

use super::ChessBot;

fn random_bot(board: &ChessBoard) -> i64 {
    let x = board.possible_moves();
    let moves = &x[1..];
    let i = rand::random::<usize>() % moves.len();
    return moves[i] as i64;
}

pub static RANDOM_BOT: ChessBot = random_bot;
