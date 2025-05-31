#[derive(Copy, Clone)]
pub struct CMove {
    pub from_square: u8,
    pub to_square: u8,
    pub promotion_piece: u8,
}
