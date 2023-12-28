#[macro_export]
macro_rules! get_pos0 {
    ($x: expr) => {
       {(($x as u32) & POS_0_FLAG) as usize }
    };
}

#[macro_export]
macro_rules! get_pos1 {
    ($x: expr) => {
       { ((($x as u32) & POS_1_FLAG) as usize) >> 6 }
    };
}

#[macro_export]
macro_rules! get_promote {
    ($x: expr) => {
        { cell_from_u8(((($x as u32) & PROMOTE_FLAG) >> 12) as u8)}
    };
}
