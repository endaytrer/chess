use bot::SyncChessMover;
use wasm_bindgen::prelude::*;
use chessboard::{ChessBoard, Player};

use crate::{bot::{random_bot::RANDOM_BOT, naive_bot::NAIVE_BOT}, chessboard::{MoveRequest, MoveResult}};
mod chessboard;
mod game;
mod bot;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    pub fn movePiece(pos1: usize, pos2: usize, robot: bool);
}

#[wasm_bindgen]
pub fn cb_new() -> *mut ChessBoard {
    let cb = ChessBoard::new();
    Box::into_raw(Box::new(cb))
}

#[wasm_bindgen]
pub fn cb_delete(cb: *mut ChessBoard) {
    let b = unsafe {
        Box::from_raw(cb)
    };
    drop(b);
}


#[wasm_bindgen]
pub fn serialize(cb: *const ChessBoard, data_ptr: *mut[u8; 36]) {
    let buf = unsafe{
        (*cb).serialize()
    };
    unsafe{
        *data_ptr = buf;
    }
}

#[wasm_bindgen]
pub fn cb_do_move(cb: *mut ChessBoard, m: MoveRequest) -> MoveResult {
    
    unsafe{
        (*cb).do_move(m)
    }
}

#[wasm_bindgen]
pub fn cb_get_possible_moves(cb: *const ChessBoard) -> Vec<MoveRequest> {
    return unsafe {
        (*cb).possible_moves()
    };
}

#[wasm_bindgen]
pub fn cb_get_attacking_range(cb: *const ChessBoard, attacker: i32) -> u64 {
    let player = if attacker == 0 { Player::White } else { Player::Black };
    return unsafe {
        (*cb).get_attacking_range(player).0
    };
}

#[wasm_bindgen]
pub fn am_naive(cb: *mut ChessBoard) -> *mut SyncChessMover {
    let am = SyncChessMover::new(NAIVE_BOT, cb);
    Box::into_raw(Box::new(am))
}

#[wasm_bindgen]
pub fn am_random(cb: *mut ChessBoard) -> *mut SyncChessMover {
    let am = SyncChessMover::new(RANDOM_BOT, cb);
    Box::into_raw(Box::new(am))
}

#[wasm_bindgen]
pub fn am_make_move(am: *mut SyncChessMover) -> i64 {
    unsafe {(*am).sync_make_move()}
}