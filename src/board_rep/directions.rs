pub type Direction = (i8, i8);

pub const NORTH: Direction = (0, -1);
pub const WEST: Direction = (-1, 0);
pub const EAST: Direction = (1, 0);
pub const SOUTH: Direction = (0, 1);
pub const NORTH_WEST: Direction = (-1, -1);
pub const NORTH_EAST: Direction = (1, -1);
pub const SOUTH_WEST: Direction = (-1, 1);
pub const SOUTH_EAST: Direction = (1, 1);
pub const KNIGHT_MOVES: [Direction; 8] = [
    (-1, 2),
    (1, 2),
    (-2, 1),
    (2, 1),
    (-2, -1),
    (2, -1),
    (-1, -2),
    (1, -2),
];
pub const KING_MOVES: [Direction; 8] = [
    NORTH, SOUTH, EAST, WEST, NORTH_WEST, NORTH_EAST, SOUTH_WEST, SOUTH_EAST,
];
