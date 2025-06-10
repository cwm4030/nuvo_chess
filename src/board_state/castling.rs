pub const WHITE_KING: u8 = 1;
pub const WHITE_QUEEN: u8 = 2;
pub const BLACK_KING: u8 = 4;
pub const BLACK_QUEEN: u8 = 8;

pub fn get_castling_rights_string(castling_rights: u8) -> String {
    let mut rights = String::new();
    if castling_rights & WHITE_KING != 0 {
        rights.push('K');
    }
    if castling_rights & WHITE_QUEEN != 0 {
        rights.push('Q');
    }
    if castling_rights & BLACK_KING != 0 {
        rights.push('k');
    }
    if castling_rights & BLACK_QUEEN != 0 {
        rights.push('q');
    }
    if rights.is_empty() {
        rights.push('-');
    }
    rights
}
