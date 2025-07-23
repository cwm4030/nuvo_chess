#[derive(Clone, Copy, Default)]
pub struct CMove {
    pub from: u8,
    pub to: u8,
    pub promotion: u8,
}
