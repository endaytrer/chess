#[macro_use]
pub mod macros;

const SIZE: usize = 8;
const SIZE_2: usize = SIZE * SIZE;
const WHITE_KING_SIDE: usize = 0;
const WHITE_QUEEN_SIDE: usize = 1;
const BLACK_KING_SIDE: usize = 2;
const BLACK_QUEEN_SIDE: usize = 3;

const WHITE_KING_SIDE_POS: usize = 6;
const WHITE_QUEEN_SIDE_POS: usize = 2;
const BLACK_KING_SIDE_POS: usize = 62;
const BLACK_QUEEN_SIDE_POS: usize = 58;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

fn piece_from_u8(value: u8) -> Piece {
    unsafe { std::mem::transmute(value) }
}

type Cell = Option<(Player, Piece)>;
const PLAYER_FLAG: u8 = 8;
pub fn cell_to_u8(cell: &Cell) -> u8 {
    match cell {
        None => 0,
        Some((player, piece)) => 1 + ((*player as u8) << 3) + (*piece as u8),
    }
}
pub fn cell_from_u8(cell: u8) -> Cell {
    if cell == 0 {
        return None;
    }
    let player = if cell & PLAYER_FLAG == 0 { Player::White } else { Player::Black };
    let piece = piece_from_u8((cell - 1) & !PLAYER_FLAG);
    Some((player, piece))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black
}
impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}



pub type MoveRequest = u16;

pub const MOVES_UNCHECKED_LEADER: MoveRequest = 0;
pub const MOVES_CHECKED_LEADER: MoveRequest = 1;

pub fn mreq_new(pos0: usize, pos1: usize) -> MoveRequest {
    (pos0 as MoveRequest) | ((pos1 << 6) as MoveRequest)
}

pub fn mreq_new_with_promote(pos0: usize, pos1: usize, promote: &Cell) -> MoveRequest {
    (pos0 as MoveRequest) | ((pos1 << 6) as MoveRequest) | ((cell_to_u8(promote) as MoveRequest) << 12)
}

pub type MoveResult = u32;

pub const POS_0_FLAG: u32 = 0x3f;
pub const POS_1_FLAG: u32 = 0xfc0;
pub const EN_PASSANT_FLAG: u32 = 0x100000;
pub const CASTLE_FLAG: u32 = 0x200000;
pub const PREV_CASTLE_RIGHT_WK: u32 = 0x400000;
pub const PREV_CASTLE_RIGHT_WQ: u32 = 0x800000;
pub const PREV_CASTLE_RIGHT_BK: u32 = 0x1000000;
pub const PREV_CASTLE_RIGHT_BQ: u32 = 0x2000000;
pub const PROMOTE_FLAG: u32 = 0xf000;
pub const CAPTURE_FLAG: u32 = 0xf0000;

pub fn mres_new(mreq: MoveRequest) -> MoveResult {
    mreq as MoveResult
}

pub fn mres_set_promote(res: &mut MoveResult, promote: &Cell) {
    *res &= !PROMOTE_FLAG;
    *res |= (cell_to_u8(promote) as u32) << 12;
}

pub fn mres_set_capture(res: &mut MoveResult, capture: &Cell) {
    *res &= !CAPTURE_FLAG;
    *res |= (cell_to_u8(capture) as u32) << 16;
}

pub fn mres_set_en_passant(res: &mut MoveResult) {
    *res |= EN_PASSANT_FLAG;
}

pub fn mres_set_castle(res: &mut MoveResult) {
    *res |= CASTLE_FLAG;
}

pub fn mres_get_capture(res: MoveResult) -> Cell {
    cell_from_u8(((res & CAPTURE_FLAG) >> 16) as u8)
}

pub fn mres_get_en_passant(res: MoveResult) -> bool {
    res & EN_PASSANT_FLAG != 0
}

pub fn mres_get_castle(res: MoveResult) -> bool {
    res & CASTLE_FLAG != 0
}


pub fn mres_get_prev_castle_right_wk(res: MoveResult) -> bool {
    res & PREV_CASTLE_RIGHT_WK != 0
}

pub fn mres_get_prev_castle_right_wq(res: MoveResult) -> bool {
    res & PREV_CASTLE_RIGHT_WQ != 0
}

pub fn mres_get_prev_castle_right_bk(res: MoveResult) -> bool {
    res & PREV_CASTLE_RIGHT_BK != 0
}

pub fn mres_get_prev_castle_right_bq(res: MoveResult) -> bool {
    res & PREV_CASTLE_RIGHT_BQ != 0
}

#[derive(Clone, Copy)]
pub struct ChessBoard {
    pub board: [Cell; SIZE_2],
    pub player: Player,
    castle_rights: [bool; 4], // [0] = white king side, [1] = white queen side, [2] = black king side, [3] = black queen side
    en_passant: Option<usize>, // the position of possible en passant.
    half_move: usize,
    full_move: usize
}
impl ChessBoard {
    pub fn new() -> Self {
        use Player::*;
        use Piece::*;
        ChessBoard {
            board: [Some((White, Rook)), Some((White, Knight)), Some((White, Bishop)), Some((White, Queen)), Some((White, King)), Some((White, Bishop)), Some((White, Knight)), Some((White, Rook)),
                    Some((White, Pawn)), Some((White, Pawn)),   Some((White, Pawn)),   Some((White, Pawn)),  Some((White, Pawn)), Some((White, Pawn)),   Some((White, Pawn)),   Some((White, Pawn)),
                    None,        None,          None,          None,         None,        None,          None,          None,
                    None,        None,          None,          None,         None,        None,          None,          None,
                    None,        None,          None,          None,         None,        None,          None,          None,
                    None,        None,          None,          None,         None,        None,          None,          None,
                    Some((Black, Pawn)), Some((Black, Pawn)),   Some((Black, Pawn)),   Some((Black, Pawn)),  Some((Black, Pawn)), Some((Black, Pawn)),   Some((Black, Pawn)),   Some((Black, Pawn)),
                    Some((Black, Rook)), Some((Black, Knight)), Some((Black, Bishop)), Some((Black, Queen)), Some((Black, King)), Some((Black, Bishop)), Some((Black, Knight)), Some((Black, Rook))],
            player: Player::White,
            castle_rights: [true, true, true, true],
            en_passant: Option::None,
            half_move: 0,
            full_move: 1
        }
    }
    /**
     * Serialize the chessboard in format ([x1, x2] is one byte, x1 is high four bits, x2 is low four bits)
     * [B1,A1], [D1,C1], [F1,E1], [H1,G1], [B2,A2], ..., [H8,G8], [En passant column(3bits),qkQK,(w/b)], [en passant possible(1bit), halfmove(7bits)], fullmove(16bits), little endian
     */
    pub fn serialize(&self) -> [u8; 36] { 
        let mut ans = [0u8; 36];
        for i in 0..32 {
            let c1 = &self.board[2 * i];
            let c2 = &self.board[2 * i + 1];
            let tmp: &mut u8 = &mut ans[i];
            *tmp |= cell_to_u8(c1);
            *tmp |= cell_to_u8(c2) << 4;
        }
        ans[32] = (self.player as u8) | ((self.castle_rights[WHITE_KING_SIDE] as u8) << 1) | ((self.castle_rights[WHITE_QUEEN_SIDE] as u8) << 2) | ((self.castle_rights[BLACK_KING_SIDE] as u8) << 3) | ((self.castle_rights[BLACK_QUEEN_SIDE] as u8) << 4);
        ans[33] = self.half_move as u8 & 0x7f; // no more than 128 half moves.
        if let Some(pos) = self.en_passant {
            ans[32] |= ((pos as u8) & 0x7) << 5;
            ans[33] |= 1 << 7;
        }
        ans[34] = (self.full_move as u16 & 0xff) as u8;
        ans[35] = (self.full_move as u16 >> 8) as u8;
        ans
    }

    // return 
    pub fn do_move(&mut self, mreq: MoveRequest) -> MoveResult {
        // deal with special cases
        let mut ans: MoveResult = mres_new(mreq);
        self.half_move += 1;
        let pos0 = (ans & POS_0_FLAG) as usize;
        let pos1 = ((ans & POS_1_FLAG) >> 6) as usize;
        let promote_to = cell_from_u8(((ans & PROMOTE_FLAG) >> 12) as u8);

        if self.board[pos0].unwrap().1 == Piece::Pawn {
            self.half_move = 0; // every pawn advance comes with a halfmove update
            // 1. en passant
            // capture enemy
            if let Some(pos) = self.en_passant {
                if pos == pos1 {
                    let captured_pos = pos0 / SIZE * SIZE + pos1 % SIZE;
                    mres_set_capture(&mut ans, &self.board[captured_pos]);
                    mres_set_en_passant(&mut ans);
                    self.board[captured_pos] = None;
                }
            }
            // update en passant of self
            self.en_passant = match self.player {
                Player::White => {
                    if pos0 / SIZE == 1 && pos1 - pos0 == 2 * SIZE {
                        Some(pos0 + SIZE)
                    } else {
                        None
                    }
                },
                Player::Black => {
                    if pos0 / SIZE == (SIZE - 2) && pos0 - pos1 == 2 * SIZE {
                        Some(pos0 - SIZE)
                    } else {
                        None
                    }
                }
            };
            // 2 promote
            match self.player {
                Player::White => {
                    if pos1 / SIZE == (SIZE - 1) {
                        // update pos0 since we will update later.
                        mres_set_promote(&mut ans, &promote_to);
                        self.board[pos0] = promote_to;
                    }
                },
                Player::Black => {
                    if pos1 / SIZE == 0 {
                        mres_set_promote(&mut ans, &promote_to);
                        self.board[pos0] = promote_to;
                    }
                }
            }
        } else {
            self.en_passant = None;
        }
        // 3. castle
        // castle first
        
        match self.player {
            Player::White => {
                if self.castle_rights[WHITE_KING_SIDE] && pos0 == WHITE_KING_SIDE_POS - 2 && pos1 == WHITE_KING_SIDE_POS {

                    self.castle_rights[WHITE_KING_SIDE] = false;
                    self.castle_rights[WHITE_QUEEN_SIDE] = false;

                    // move castle to 5
                    mres_set_castle(&mut ans);
                    self.board[WHITE_KING_SIDE_POS - 1] = self.board[7];
                    self.board[7] = None;

                    // king will be automatically moved
                }

                if self.castle_rights[WHITE_QUEEN_SIDE] && pos0 == WHITE_QUEEN_SIDE_POS + 2 && pos1 == WHITE_QUEEN_SIDE_POS {
                    
                    self.castle_rights[WHITE_KING_SIDE] = false;
                    self.castle_rights[WHITE_QUEEN_SIDE] = false;

                    // move castle to 3
                    mres_set_castle(&mut ans);
                    self.board[WHITE_QUEEN_SIDE_POS + 1] = self.board[0];
                    self.board[0] = None;

                    // king will be automatically moved
                }
            },
            Player::Black => {
                if self.castle_rights[BLACK_KING_SIDE] && pos0 == BLACK_KING_SIDE_POS - 2 && pos1 == BLACK_KING_SIDE_POS {

                    self.castle_rights[BLACK_KING_SIDE] = false;
                    self.castle_rights[BLACK_QUEEN_SIDE] = false;

                    // move castle to 61
                    mres_set_castle(&mut ans);
                    self.board[BLACK_KING_SIDE_POS - 1] = self.board[63];
                    self.board[63] = None;

                    // king will be automatically moved
                }

                if self.castle_rights[BLACK_QUEEN_SIDE] && pos0 == BLACK_QUEEN_SIDE_POS + 2 && pos1 == BLACK_QUEEN_SIDE_POS {
                    self.castle_rights[WHITE_KING_SIDE] = false;
                    self.castle_rights[WHITE_QUEEN_SIDE] = false;

                    // move castle to 59
                    mres_set_castle(&mut ans);
                    self.board[BLACK_QUEEN_SIDE_POS + 1] = self.board[56];
                    self.board[56] = None;

                    // king will be automatically moved
                }
            }
        }
        // then check if this move make castle impossible. both source and destination to/from 0, 4, 7, 56, 60, 63
        if self.castle_rights[WHITE_KING_SIDE] {
            if pos0 == 4 || pos0 == 7 || pos1 == 4 || pos1 == 7 {
                self.castle_rights[WHITE_KING_SIDE] = false;
            }
        }
        if self.castle_rights[WHITE_QUEEN_SIDE] {
            if pos0 == 0 || pos0 == 4 || pos1 == 0 || pos1 == 4 {
                self.castle_rights[WHITE_QUEEN_SIDE] = false;
            }
        }
        if self.castle_rights[BLACK_KING_SIDE] {
            if pos0 == 60 || pos0 == 63 || pos1 == 60 || pos1 == 63 {
                self.castle_rights[BLACK_KING_SIDE] = false;
            }
        }
        if self.castle_rights[BLACK_QUEEN_SIDE] {
            if pos0 == 56 || pos0 == 60 || pos1 == 56 || pos1 == 60 {
                self.castle_rights[BLACK_QUEEN_SIDE] = false;
            }
        }
        if self.board[pos1] != None {
            self.half_move = 0;
            mres_set_capture(&mut ans, &self.board[pos1]);
        }
        self.board[pos1] = self.board[pos0];
        self.board[pos0] = None;
        if self.player == Player::Black {
            self.full_move += 1;
        }
        self.player = self.player.opponent();
        return ans;
    }
    
    // will not update full moves and half moves.
    pub fn undo_move(&mut self, mres: MoveResult) {
        let pos0 = get_pos0!(mres);
        let pos1 = get_pos1!(mres);
        let promote = get_promote!(mres);
        let capture = mres_get_capture(mres);
        let is_castle = mres_get_castle(mres);
        let is_en_passant = mres_get_en_passant(mres);

        self.player = self.player.opponent();

        if let Some(_) = promote {
            self.board[pos0] = Some((self.player, Piece::Pawn));
        } else {
            self.board[pos0] = self.board[pos1];
        }
        self.en_passant = None;
        if is_en_passant {
            let captured_pos = pos0 / SIZE * SIZE + pos1 % SIZE;
            self.board[captured_pos] = capture;
            self.en_passant = Some(captured_pos);
            self.board[pos1] = None;
        } else {
            self.board[pos1] = capture;
        }
        self.castle_rights[WHITE_KING_SIDE] = mres_get_prev_castle_right_wk(mres);
        self.castle_rights[WHITE_QUEEN_SIDE] = mres_get_prev_castle_right_wq(mres);
        self.castle_rights[BLACK_KING_SIDE] = mres_get_prev_castle_right_bk(mres);
        self.castle_rights[BLACK_QUEEN_SIDE] = mres_get_prev_castle_right_bq(mres);

        if is_castle {
            if pos1 == WHITE_KING_SIDE_POS || pos1 == BLACK_KING_SIDE_POS {
                self.board[pos1 + 1] = self.board[pos1 - 1];
                self.board[pos1 - 1] = None;
            } else {
                self.board[pos1 - 2] = self.board[pos1 + 1];
                self.board[pos1 + 1] = None;
            }
        }
    }
    fn rook_move(&self, pos: usize, player: Player, ans: &mut Vec<usize>) {
        // left
        let mut k = 1;
        while (pos - k) % SIZE != (SIZE - 1) && self.board[pos - k] == None {
            ans.push(pos - k);
            k += 1;
        }
        // if enemy piece, able to capture
        if (pos - k) % SIZE != (SIZE - 1) && self.board[pos - k].unwrap().0 != player {
            ans.push(pos - k);
        }

        // right
        k = 1;
        while (pos + k) % SIZE != 0 && self.board[pos + k] == None {
            ans.push(pos + k);
            k += 1;
        }
        // if enemy piece, able to capture
        if (pos + k) % SIZE != 0 && self.board[pos + k].unwrap().0 != player {
            ans.push(pos + k);
        }

        // up
        k = 1;
        while (pos + SIZE * k) < SIZE_2 && self.board[pos + SIZE * k] == None {
            ans.push(pos + SIZE * k);
            k += 1;
        }
        // if enemy piece, able to capture
        if (pos + SIZE * k) < SIZE_2 && self.board[pos + SIZE * k].unwrap().0 != player {
            ans.push(pos + SIZE * k);
        }
        
        // down
        k = 1;
        while pos >= SIZE * k && self.board[pos - SIZE * k] == None {
            ans.push(pos - SIZE * k);
            k += 1;
        }
        // if enemy piece, able to capture
        if pos >= SIZE * k && self.board[pos - SIZE * k].unwrap().0 != player {
            ans.push(pos - SIZE * k);
        }
    }

    fn bishop_move(&self, pos: usize, player: Player, ans: &mut Vec<usize>) {
        let i = pos / SIZE;
        let j = pos % SIZE;
        // down left
        let mut k = 1;
        while i >= k && j >= k && self.board[pos - (SIZE + 1) * k] == None {
            ans.push(pos - (SIZE + 1) * k);
            k += 1;
        }
        // if enemy piece, able to capture
        if i >= k && j >= k && self.board[pos - (SIZE + 1) * k].unwrap().0 != player {
            ans.push(pos - (SIZE + 1) * k);
        }

        // up left
        k = 1;
        while i + k < SIZE && j >= k && self.board[pos + (SIZE - 1) * k] == None {
            ans.push(pos + (SIZE - 1) * k);
            k += 1;
        }
        // if enemy piece, able to capture
        if i + k < SIZE && j >= k && self.board[pos + (SIZE - 1) * k].unwrap().0 != player {
            ans.push(pos + (SIZE - 1) * k);
        }

        // up right
        k = 1;
        while i + k < SIZE && j + k < SIZE && self.board[pos + (SIZE + 1) * k] == None {
            ans.push(pos + (SIZE + 1) * k);
            k += 1;
        }
        // if enemy piece, able to capture
        if i + k < SIZE && j + k < SIZE && self.board[pos + (SIZE + 1) * k].unwrap().0 != player {
            ans.push(pos + (SIZE + 1) * k);
        }
        
        // down right
        k = 1;
        while i >= k && j + k < SIZE && self.board[pos - (SIZE - 1) * k] == None {
            ans.push(pos - (SIZE - 1) * k);
            k += 1;
        }
        // if enemy piece, able to capture
        if i >= k && j + k < SIZE && self.board[pos - (SIZE - 1) * k].unwrap().0 != player {
            ans.push(pos - (SIZE - 1) * k);
        }
    }

    fn knight_move(&self, pos: usize, player: Player, ans: &mut Vec<usize>) {
        let i = pos / SIZE;
        let j = pos % SIZE;
        if i >= 2 && j >= 1 && (self.board[pos - (2 * SIZE + 1)] == None || self.board[pos - (2 * SIZE + 1)].unwrap().0 != player) {
            ans.push(pos - (2 * SIZE + 1));
        }
        if i >= 1 && j >= 2 && (self.board[pos - (SIZE + 2)] == None || self.board[pos - (SIZE + 2)].unwrap().0 != player) {
            ans.push(pos - (SIZE + 2));
        }
        if i >= 1 && j < SIZE - 2 && (self.board[pos - (SIZE - 2)] == None || self.board[pos - (SIZE - 2)].unwrap().0 != player) {
            ans.push(pos - (SIZE - 2));
        }
        if i >= 2 && j < SIZE - 1 && (self.board[pos - (2 * SIZE - 1)] == None || self.board[pos - (2 * SIZE - 1)].unwrap().0 != player) {
            ans.push(pos - (2 * SIZE - 1));
        }
        if i < SIZE - 2 && j < SIZE - 1 && (self.board[pos + (2 * SIZE + 1)] == None || self.board[pos + (2 * SIZE + 1)].unwrap().0 != player) {
            ans.push(pos + (2 * SIZE + 1));
        }
        if i < SIZE - 1 && j < SIZE - 2 && (self.board[pos + (SIZE + 2)] == None || self.board[pos + (SIZE + 2)].unwrap().0 != player) {
            ans.push(pos + (SIZE + 2));
        }
        if i < SIZE - 1 && j >= 2 && (self.board[pos + (SIZE - 2)] == None || self.board[pos + (SIZE - 2)].unwrap().0 != player) {
            ans.push(pos + (SIZE - 2));
        }
        if i < SIZE - 2 && j >= 1 && (self.board[pos + (2 * SIZE - 1)] == None || self.board[pos + (2 * SIZE - 1)].unwrap().0 != player) {
            ans.push(pos + (2 * SIZE - 1));
        }
    }
    
    fn king_ordinary_move(&self, pos: usize, player: Player, ans: &mut Vec<usize>) {
        let i = pos / SIZE;
        let j = pos % SIZE;
        let i_start = if i == 0 { 0 } else { i - 1 };
        let i_end = if i == SIZE - 1 { SIZE - 1 } else { i + 1 };
        let j_start = if j == 0 { 0 } else { j - 1 };
        let j_end = if j == SIZE - 1 { SIZE - 1 } else { j + 1 };
        for move_i in i_start..=i_end {
            for move_j in j_start..=j_end {
                if move_i == i && move_j == j { continue }
                let new_pos = move_i * SIZE + move_j;
                if self.board[new_pos] == None || self.board[new_pos].unwrap().0 != player {
                    ans.push(new_pos);
                }
            }
        }
    }
    
    // returns uint64, get i-th by ((x >> i) & 1); also a bool if the defender is been checked.
    pub fn get_attacking_range(&self, attacker: Player) -> (u64, bool) {
        let mut ans: u64 = 0;
        let mut checked = false;
        let defender = match attacker {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };
        for pos in 0..SIZE_2 {
            if self.board[pos] == None || self.board[pos].unwrap().0 != attacker {
                continue
            }
            let piece = self.board[pos].unwrap().1;

            match piece {
                Piece::Pawn => {
                    match attacker{
                        Player::White => {
                            if pos % SIZE >= 1 {
                                ans |= 1 << (pos + SIZE - 1);
                                if self.board[pos + SIZE - 1] == Some((defender, Piece::King)) {
                                    checked = true;
                                }
                            }
                            if pos % SIZE < SIZE - 1 {
                                ans |= 1 << (pos + SIZE + 1);
                                if self.board[pos + SIZE + 1] == Some((defender, Piece::King)) {
                                    checked = true;
                                }
                            }
                        },
                        Player::Black => {
                            if pos % SIZE >= 1 {
                                ans |= 1 << (pos - SIZE - 1);
                                if self.board[pos - SIZE - 1] == Some((defender, Piece::King)) {
                                    checked = true;
                                }
                            }
                            if pos % SIZE < SIZE - 1 {
                                ans |= 1 << (pos - SIZE + 1);
                                if self.board[pos - SIZE + 1] == Some((defender, Piece::King)) {
                                    checked = true;
                                }
                            }
                        }
                    }
                },
                Piece::Rook => {
                    let mut attacked: Vec<usize> = Vec::new();
                    self.rook_move(pos, attacker, &mut attacked);
                    for i in attacked {
                        ans |= 1 << i;
                        if self.board[i] == Some((defender, Piece::King)) {
                            checked = true;
                        }
                    }
                }
                Piece::Knight => {
                    let mut attacked: Vec<usize> = Vec::new();
                    self.knight_move(pos, attacker, &mut attacked);
                    for i in attacked {
                        ans |= 1 << i;
                        if self.board[i] == Some((defender, Piece::King)) {
                            checked = true;
                        }
                    }
                },
                Piece::Bishop => {
                    let mut attacked: Vec<usize> = Vec::new();
                    self.bishop_move(pos, attacker, &mut attacked);
                    for i in attacked {
                        ans |= 1 << i;
                        if self.board[i] == Some((defender, Piece::King)) {
                            checked = true;
                        }
                    }
                },
                Piece::Queen => {
                    let mut attacked: Vec<usize> = Vec::new();
                    self.rook_move(pos, attacker, &mut attacked);
                    self.bishop_move(pos, attacker, &mut attacked);
                    for i in attacked {
                        ans |= 1 << i;
                        if self.board[i] == Some((defender, Piece::King)) {
                            checked = true;
                        }
                    }
                },
                Piece::King => {
                    let mut attacked: Vec<usize> = Vec::new();
                    self.king_ordinary_move(pos, attacker, &mut attacked);
                    for i in attacked {
                        ans |= 1 << i;
                        if self.board[i] == Some((defender, Piece::King)) {
                            checked = true;
                        }
                    }
                },
            }
        }
        (ans, checked)

    }
    // Returns all possible moves could be done by the player.
    // The 0th element is guaranteed to be if the player is been checked. 0 is no, 1 is yes.
    // The following is a list of possible moves. lower 16 bits indicate source, and higher 16 bits indicate destination.
    pub fn possible_moves(&self) -> Vec<MoveRequest> {
        let mut ans: Vec<MoveRequest> = vec![];
        let opponent = self.player.opponent();

        let (enemy_range, checked) = self.get_attacking_range(opponent);

        if checked {
            ans.push(MOVES_CHECKED_LEADER);
        } else {
            ans.push(MOVES_UNCHECKED_LEADER);
        }

        for pos in 0..SIZE_2 {
            let (player, piece) = match self.board[pos] {
                None => continue,
                Some((player, piece)) => {
                    if self.player != player {
                        continue
                    }
                    (player, piece)
                }
            };
            match piece {
                Piece::Pawn => {
                    use Player::{White, Black};
                    use Piece::{Queen, Rook, Bishop, Knight};
                    match player {
                        White => {
                            if self.board[pos + SIZE] == None {
                                let new_pos = pos + SIZE;
                                if new_pos / SIZE == 7 {
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Queen))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Rook))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Bishop))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Knight))));
                                } else {
                                    ans.push(mreq_new(pos, new_pos));
                                }
                            }
                            // if not moved, en passant
                            if pos / SIZE == 1 && self.board[pos + SIZE] == None && self.board[pos + 2 * SIZE] == None {
                                let new_pos = pos + 2 * SIZE;
                                if new_pos / SIZE == 7 {
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Queen))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Rook))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Bishop))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Knight))));
                                } else {
                                    ans.push(mreq_new(pos, new_pos));
                                }
                            }
                            // capturing
                            if pos % SIZE < SIZE - 1 && 
                                (self.board[pos + SIZE + 1] != None && self.board[pos + SIZE + 1].unwrap().0 != player ||
                                self.en_passant != None && self.en_passant.unwrap() == pos + SIZE + 1) {
                                let new_pos = pos + SIZE + 1;
                                if new_pos / SIZE == 7 {
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Queen))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Rook))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Bishop))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Knight))));
                                } else {
                                    ans.push(mreq_new(pos, new_pos));
                                }
                            }
                            if pos % SIZE >= 1 && 
                                (self.board[pos + SIZE - 1] != None && self.board[pos + SIZE - 1].unwrap().0 != player ||
                                self.en_passant != None && self.en_passant.unwrap() == pos + SIZE - 1) {
                                let new_pos = pos + SIZE - 1;
                                if new_pos / SIZE == 7 {
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Queen))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Rook))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Bishop))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((White, Knight))));
                                } else {
                                    ans.push(mreq_new(pos, new_pos));
                                }
                            }
                        },
                        Black => {
                            if self.board[pos - SIZE] == None {
                                let new_pos = pos - SIZE;
                                if new_pos / SIZE == 0 {
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Queen))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Rook))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Bishop))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Knight))));
                                } else {
                                    ans.push(mreq_new(pos, new_pos));
                                }
                            }
                            // if not moved, en passant
                            if pos / SIZE == (SIZE - 2) && self.board[pos - SIZE] == None && self.board[pos - 2 * SIZE] == None {
                                let new_pos = pos - 2 * SIZE;
                                if new_pos / SIZE == 0 {
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Queen))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Rook))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Bishop))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Knight))));
                                } else {
                                    ans.push(mreq_new(pos, new_pos));
                                }
                            }
                            // capturing
                            if pos % SIZE < SIZE - 1 && 
                                (self.board[pos - SIZE + 1] != None && self.board[pos - SIZE + 1].unwrap().0 != player ||
                                self.en_passant != None && self.en_passant.unwrap() == pos - SIZE + 1) {
                                let new_pos = pos - SIZE + 1;
                                if new_pos / SIZE == 0 {
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Queen))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Rook))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Bishop))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Knight))));
                                } else {
                                    ans.push(mreq_new(pos, new_pos));
                                }
                            }
                            if pos % SIZE >= 1 && 
                                (self.board[pos - SIZE - 1] != None && self.board[pos - SIZE - 1].unwrap().0 != player ||
                                self.en_passant != None && self.en_passant.unwrap() == pos - SIZE - 1) {
                                let new_pos = pos - SIZE - 1;
                                if new_pos / SIZE == 0 {
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Queen))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Rook))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Bishop))));
                                    ans.push(mreq_new_with_promote(pos, new_pos, &Some((Black, Knight))));
                                } else {
                                    ans.push(mreq_new(pos, new_pos));
                                }
                            }
                        }
                    }
                },
                Piece::Rook => {
                    let mut new_poses = vec![];
                    self.rook_move(pos, player, &mut new_poses);
                    for new_pos in new_poses {
                        ans.push(mreq_new(pos, new_pos));
                    }
                },
                Piece::Knight => {
                    let mut new_poses = vec![];
                    self.knight_move(pos, player, &mut new_poses);
                    for new_pos in new_poses {
                        ans.push(mreq_new(pos, new_pos));
                    }
                },
                Piece::Bishop => {
                    let mut new_poses = vec![];
                    self.bishop_move(pos, player, &mut new_poses);
                    for new_pos in new_poses {
                        ans.push(mreq_new(pos, new_pos));
                    }
                },
                Piece::Queen => {
                    let mut new_poses = vec![];
                    self.rook_move(pos, player, &mut new_poses);
                    self.bishop_move(pos, player, &mut new_poses);
                    for new_pos in new_poses {
                        ans.push(mreq_new(pos, new_pos));
                    }
                },
                Piece::King => {
                    let mut new_poses = vec![];
                    self.king_ordinary_move(pos, player, &mut new_poses);
                    for new_pos in new_poses {
                        ans.push(mreq_new(pos, new_pos));
                    }
                    // castle
                    match player {
                        Player::White => {
                            if self.castle_rights[WHITE_KING_SIDE] && self.board[WHITE_KING_SIDE_POS - 1] == None && self.board[WHITE_KING_SIDE_POS] == None &&
                                (enemy_range >> (WHITE_KING_SIDE_POS - 2) & 1 != 1) &&
                                (enemy_range >> (WHITE_KING_SIDE_POS - 1) & 1 != 1) &&
                                (enemy_range >> (WHITE_KING_SIDE_POS) & 1 != 1) {
                                ans.push(mreq_new(pos, WHITE_KING_SIDE_POS));
                            }
                            if self.castle_rights[WHITE_QUEEN_SIDE] && self.board[WHITE_QUEEN_SIDE_POS - 1] == None && self.board[WHITE_QUEEN_SIDE_POS] == None && self.board[WHITE_QUEEN_SIDE_POS + 1] == None &&
                                (enemy_range >> (WHITE_QUEEN_SIDE_POS + 2) & 1 != 1) &&
                                (enemy_range >> (WHITE_QUEEN_SIDE_POS + 1) & 1 != 1) &&
                                (enemy_range >> (WHITE_QUEEN_SIDE_POS) & 1 != 1) {
                                ans.push(mreq_new(pos, WHITE_QUEEN_SIDE_POS));
                            }
                        },
                        Player::Black => {
                            if self.castle_rights[BLACK_KING_SIDE] && self.board[BLACK_KING_SIDE_POS - 1] == None && self.board[BLACK_KING_SIDE_POS] == None &&
                                (enemy_range >> (BLACK_KING_SIDE_POS - 2) & 1 != 1) &&
                                (enemy_range >> (BLACK_KING_SIDE_POS - 1) & 1 != 1) &&
                                (enemy_range >> (BLACK_KING_SIDE_POS) & 1 != 1) {
                                    ans.push(mreq_new(pos, BLACK_KING_SIDE_POS));
                            }
                            if self.castle_rights[BLACK_QUEEN_SIDE] && self.board[BLACK_QUEEN_SIDE_POS - 1] == None && self.board[BLACK_QUEEN_SIDE_POS] == None && self.board[BLACK_QUEEN_SIDE_POS + 1] == None &&
                                (enemy_range >> (BLACK_QUEEN_SIDE_POS + 2) & 1 != 1) &&
                                (enemy_range >> (BLACK_QUEEN_SIDE_POS + 1) & 1 != 1) &&
                                (enemy_range >> (BLACK_QUEEN_SIDE_POS) & 1 != 1) {
                                    ans.push(mreq_new(pos, BLACK_QUEEN_SIDE_POS));
                            }
                        }
                    }
                },
            }
        }
        
        // filter out moves that lead to / maintains check state;
        let mut filtered_ans = vec![ans[0]];
        for possible_move in ans.into_iter().skip(1) {

            let mut new_board = self.clone();
            new_board.do_move(possible_move);
            let (_, still_checked) = new_board.get_attacking_range(opponent);
            if !still_checked {
                filtered_ans.push(possible_move);
            }
        }
        filtered_ans
    }
}

