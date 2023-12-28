use crate::chessboard::*;
use crate::get_pos0;
use crate::get_pos1;
use super::ChessBot;

const PIECE_VALUES: [i32; 6] = [100, 500, 300, 300, 900, 0];
const FORCE_CHECKMATE_LIMIT: i32 = 1 << 30;
const START_POS_VALUES: [[i32; 64]; 6] = [
    [
        0,   0,   0,   0,   0,   0,   0,   0,
        5,  10,  10, -20, -20,  10,  10,   5,
        5,  -5, -10,   0,   0, -10,  -5,   5,
        0,   0,   0,  20,  20,   0,   0,   0,
        5,   5,  10,  25,  25,  10,   5,   5,
        10,  10,  20,  30,  30,  20,  10,  10,
        50,  50,  50,  50,  50,  50,  50,  50,
        0,   0,   0,   0,   0,   0,   0,   0,
    ],
    [
        0,  0,  0,  5,  5,  0,  0,  0,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  0,  0,  0,  0,  0,
    ],
    [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ],
    [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ],
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -10,  5,  5,  5,  5,  5,  0,-10,
        0,    0,  5,  5,  5,  5,  0, -5,
        -5,   0,  5,  5,  5,  5,  0, -5,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20,
    ],
    [
		20,  30,  10,   0,   0,  10,  30,  20,
        20,  20,  -5,  -5,  -5,  -5,  20,  20,
        -10, -20, -20, -20, -20, -20, -20, -10, 
        -20, -30, -30, -40, -40, -30, -30, -20, 
        -30, -40, -40, -50, -50, -40, -40, -30, 
        -40, -50, -50, -60, -60, -50, -50, -40, 
        -60, -60, -60, -60, -60, -60, -60, -60, 
        -80, -70, -70, -70, -70, -70, -70, -80
    ]
];
const END_POS_VALUES: [[i32; 64]; 6] = [
    [
        0,   0,   0,   0,   0,   0,   0,   0,
        10,  10,  10,  10,  10,  10,  10,  10,
        10,  10,  10,  10,  10,  10,  10,  10,
        20,  20,  20,  20,  20,  20,  20,  20,
        30,  30,  30,  30,  30,  30,  30,  30,
        50,  50,  50,  50,  50,  50,  50,  50,
        80,  80,  80,  80,  80,  80,  80,  80,
        0,   0,   0,   0,   0,   0,   0,   0,
    ],
    [
        0,  0,  0,  5,  5,  0,  0,  0,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        0,  0,  0,  0,  0,  0,  0,  0,
    ],
    [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ],
    [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ],
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -10,  5,  5,  5,  5,  5,  0,-10,
        0,    0,  5,  5,  5,  5,  0, -5,
        -5,   0,  5,  5,  5,  5,  0, -5,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20,
    ],
    [
		-50, -30, -30, -30, -30, -30, -30, -50,
        -30, -25,   0,   0,   0,   0, -25, -30,
        -25, -20,  20,  25,  25,  20, -20, -25,
        -20, -15,  30,  40,  40,  30, -15, -20,
        -15, -10,  35,  45,  45,  35, -10, -15,
        -10, -5,   20,  30,  30,  20,  -5, -10,
        -5,   0,   5,   5,   5,   5,   0,  -5,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ]
];
// white is max, black is min
fn evaluate(board: &ChessBoard) -> i32 {
    let mut ans = 0;
    let mut num_white = 0;
    let mut num_black = 0;
    for pos in 0..64 {
        match board.board[pos] {
            Some((Player::White, _)) => num_white += 1,
            Some((Player::Black, _)) => num_black += 1,
            None => continue,
        }
    }
    
    for pos in 0..64 {
        match board.board[pos] {
            Some((Player::White, piece)) => {
                ans += 16 * PIECE_VALUES[piece as usize];
                ans += num_white * START_POS_VALUES[piece as usize][pos] + (16 - num_white) * END_POS_VALUES[piece as usize][pos];
            },
            Some((Player::Black, piece)) => {
                ans -= 16 * PIECE_VALUES[piece as usize];
                ans -= num_black * START_POS_VALUES[piece as usize][63 - pos] + (16 - num_black) * END_POS_VALUES[piece as usize][63 - pos];
            },
            None => continue,
        }
    }
    ans
}
// move, value
fn search(board: &ChessBoard, depth: usize, mut alpha: i32, mut beta: i32, maximize: bool) -> (Option<MoveRequest>, i32) {
    if depth == 0 {
        return (None, evaluate(board));
    }
    let mut arr = board.possible_moves();
    let checked = arr[0];
    let moves = &mut arr[1..];
    if checked == MOVES_CHECKED_LEADER && moves.len() == 0 {
        if board.player == Player::White {
            return (None, -i32::MAX);
        } else {
            return (None, i32::MAX);
        }
    }
    if moves.len() == 0 {
        return (None, 0);
    }
    moves.sort_unstable_by_key(|f| {
        let mut ans = 0;
        let pos0 = board.board[get_pos0!(*f)];
        let pos1 = board.board[get_pos1!(*f)];
        if let Some((_, piece)) = pos1 {
            ans += 2 * PIECE_VALUES[piece as usize];
            ans -= PIECE_VALUES[pos0.unwrap().1 as usize];
        }
        -ans
    });

    if maximize {
        let mut maxmove = moves[0];
        let mut maxval = -i32::MAX;
        
        for m in moves {
            let mut t = board.clone();
            t.do_move(*m);
            let (_, value) = search(&mut t, depth - 1, alpha, beta, false);
            let value = if value > FORCE_CHECKMATE_LIMIT {
                value - 1
            } else if value < -FORCE_CHECKMATE_LIMIT {
                value + 1
            } else {
                value
            };
            // board.undo_move(mres);
            if value > maxval {
                maxval = value;
                maxmove = *m;
            }
            if maxval > alpha {
                alpha = maxval;
            }
            if maxval >= beta {
                break;
            }
        }
        return (Some(maxmove), maxval);
    } else {
        let mut minmove = moves[0];
        let mut minval = i32::MAX;
        for m in moves {
            let mut t = board.clone();
            t.do_move(*m);
            let (_, value) = search(&mut t, depth - 1, alpha, beta, true);
            let value = if value > FORCE_CHECKMATE_LIMIT {
                value - 1
            } else if value < -FORCE_CHECKMATE_LIMIT {
                value + 1
            } else {
                value
            };
            if value < minval {
                minval = value;
                minmove = *m;
            }
            if minval < beta {
                beta = minval;
            }
            if minval <= alpha {
                break;
            }

        }
        return (Some(minmove), minval);
    }

}

fn naive_bot(board: &ChessBoard) -> i64 {
    let max_depth = 5;
    let maximize = board.player == Player::White;
    // iterative deepening
    let mut moves = board.possible_moves();
    moves.remove(0);
    let mut evaluated_moves: Vec<(MoveRequest, i32)> = vec![];
    moves.into_iter().for_each(|mov| {
        let mut new_board = board.clone();
        new_board.do_move(mov);
        evaluated_moves.push((mov, if maximize { 1 } else { -1 } * evaluate(&new_board)));
    });

    for depth in 1..max_depth {
        let mut alpha = -i32::MAX;
        let mut beta = i32::MAX;
        let mut new_evaluated_moves: Vec<(MoveRequest, i32)> = vec![];
        evaluated_moves.sort_unstable_by_key(|m| {-m.1});
        if maximize {
            let mut maxval = -i32::MAX;
            for (mov, _) in evaluated_moves {
                let mut new_board = board.clone();
                new_board.do_move(mov);
                let (_, value) = search(&new_board, depth, alpha, beta, false);
                if value > maxval {
                    maxval = value;
                }
                if maxval > alpha {
                    alpha = maxval;
                }
                new_evaluated_moves.push((mov, value));
            }
        } else {
            let mut minval = i32::MAX;
            for (mov, _) in evaluated_moves {
                let mut new_board = board.clone();
                new_board.do_move(mov);
                let (_, value) = search(&new_board, depth, alpha, beta, true);
                if value < minval {
                    minval = value;
                }
                if minval < beta {
                    beta = minval;
                }
                new_evaluated_moves.push((mov, -value));
            }
        }
        evaluated_moves = new_evaluated_moves;
        
    }
    evaluated_moves.sort_unstable_by_key(|m| {-m.1});
    
    // let (m, eval) = search(board, 5, -i32::MAX, i32::MAX, board.player == Player::White);
    // return 
    (((if maximize {evaluated_moves[0].1} else {-evaluated_moves[0].1}) as i64) << 32) | evaluated_moves[0].0 as i64
}
pub static NAIVE_BOT: ChessBot = naive_bot;