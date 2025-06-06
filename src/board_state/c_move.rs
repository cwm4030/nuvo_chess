#[derive(Copy, Clone)]
pub struct CMove {
    pub from_index: u8,
    pub to_index: u8,
    pub promotion_piece: u8,
}
