pub const A8: u8 = 36;
pub const B8: u8 = 37;
pub const C8: u8 = 38;
pub const D8: u8 = 39;
pub const E8: u8 = 40;
pub const F8: u8 = 41;
pub const G8: u8 = 42;
pub const H8: u8 = 43;
pub const A7: u8 = 52;
pub const B7: u8 = 53;
pub const C7: u8 = 54;
pub const D7: u8 = 55;
pub const E7: u8 = 56;
pub const F7: u8 = 57;
pub const G7: u8 = 58;
pub const H7: u8 = 59;
pub const A6: u8 = 68;
pub const B6: u8 = 69;
pub const C6: u8 = 70;
pub const D6: u8 = 71;
pub const E6: u8 = 72;
pub const F6: u8 = 73;
pub const G6: u8 = 74;
pub const H6: u8 = 75;
pub const A5: u8 = 84;
pub const B5: u8 = 85;
pub const C5: u8 = 86;
pub const D5: u8 = 87;
pub const E5: u8 = 88;
pub const F5: u8 = 89;
pub const G5: u8 = 90;
pub const H5: u8 = 91;
pub const A4: u8 = 100;
pub const B4: u8 = 101;
pub const C4: u8 = 102;
pub const D4: u8 = 103;
pub const E4: u8 = 104;
pub const F4: u8 = 105;
pub const G4: u8 = 106;
pub const H4: u8 = 107;
pub const A3: u8 = 116;
pub const B3: u8 = 117;
pub const C3: u8 = 118;
pub const D3: u8 = 119;
pub const E3: u8 = 120;
pub const F3: u8 = 121;
pub const G3: u8 = 122;
pub const H3: u8 = 123;
pub const A2: u8 = 132;
pub const B2: u8 = 133;
pub const C2: u8 = 134;
pub const D2: u8 = 135;
pub const E2: u8 = 136;
pub const F2: u8 = 137;
pub const G2: u8 = 138;
pub const H2: u8 = 139;
pub const A1: u8 = 148;
pub const B1: u8 = 149;
pub const C1: u8 = 150;
pub const D1: u8 = 151;
pub const E1: u8 = 152;
pub const F1: u8 = 153;
pub const G1: u8 = 154;
pub const H1: u8 = 155;
pub const ON_BOARD_SQUARES: [u8; 64] = [
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
];
pub const ON_AND_OFF_BOARD_SQUARES: [u8; 192] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, A8, B8, C8, D8, E8, F8, G8, H8, 0, 0, 0, 0,
    0, 0, 0, 0, A7, B7, C7, D7, E7, F7, G7, H7, 0, 0, 0, 0,
    0, 0, 0, 0, A6, B6, C6, D6, E6, F6, G6, H6, 0, 0, 0, 0,
    0, 0, 0, 0, A5, B5, C5, D5, E5, F5, G5, H5, 0, 0, 0, 0,
    0, 0, 0, 0, A4, B4, C4, D4, E4, F4, G4, H4, 0, 0, 0, 0,
    0, 0, 0, 0, A3, B3, C3, D3, E3, F3, G3, H3, 0, 0, 0, 0,
    0, 0, 0, 0, A2, B2, C2, D2, E2, F2, G2, H2, 0, 0, 0, 0,
    0, 0, 0, 0, A1, B1, C1, D1, E1, F1, G1, H1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
pub const SQUARE_NAMES: [&str; 192] = [
    "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
    "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
    "-", "-", "-", "-", "A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8", "-", "-", "-", "-",
    "-", "-", "-", "-", "A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7", "-", "-", "-", "-",
    "-", "-", "-", "-", "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6", "-", "-", "-", "-",
    "-", "-", "-", "-", "A5", "B5", "C5", "D5", "E5", "F5", "G5", "H5", "-", "-", "-", "-",
    "-", "-", "-", "-", "A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4", "-", "-", "-", "-",
    "-", "-", "-", "-", "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3", "-", "-", "-", "-",
    "-", "-", "-", "-", "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2", "-", "-", "-", "-",
    "-", "-", "-", "-", "A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1", "-", "-", "-", "-",
    "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
    "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
];