use std::time::{SystemTime, UNIX_EPOCH};

use crate::board_rep::{
    bit_operations::{count_bits, first_bit_pop, is_bit_set, set_bit},
    directions::{
        Direction, EAST, KING_MOVES, KNIGHT_MOVES, NORTH, NORTH_EAST, NORTH_WEST, SOUTH,
        SOUTH_EAST, SOUTH_WEST, WEST,
    },
    rng::Rng,
    squares::{A1, A2, A7, A8, H1, H2, H7, H8},
};

const ROOK_MAGICS: [u64; 64] = [
    1333066039995678720,
    1170937758628184128,
    360297866331164736,
    72066458850697472,
    144117387636542481,
    2377909546514383104,
    144124021246531844,
    72057735771865222,
    72198470039076864,
    2450099072221054080,
    1155454989873709124,
    144678414551371784,
    10412463625791866880,
    9223653752399995904,
    563092023413248,
    40673172789477504,
    9390155306406854664,
    513411182175334400,
    3607384405332459584,
    9227876186909053056,
    5188429345487874048,
    2450099485137699328,
    720597930645989392,
    1152972082146214929,
    1275573074673672,
    1756421465114157632,
    4504287359272960,
    4652227217610244228,
    11820831920181477506,
    297239776579551360,
    4611696206094533136,
    864692167837778180,
    72216198598623272,
    9007542856319520,
    153122801820110848,
    2308376352740352522,
    585609255990597632,
    2324420394485810192,
    4802701216973058,
    4620694326890989700,
    9259409909676523520,
    2469133748801650690,
    2594108570005962880,
    1152994081771749384,
    9227101647437952,
    2814973239754768,
    9223394035677331712,
    2740442851459858436,
    36029071966076992,
    35747017001728,
    76704276209996288,
    1161939699515802112,
    2342153301207863552,
    2305984098891727232,
    6053119382752854272,
    9225633749478687232,
    4774237820174995713,
    318312912273417,
    1371381271045882113,
    3395292043157761,
    4616471376633548805,
    577023775341586442,
    54060792017983492,
    36310306900689411,
];

const BISHOP_MAGICS: [u64; 64] = [
    2375013860704272,
    869336711170232320,
    74346928678174720,
    9225668654744485988,
    297538016992297984,
    5190556969072590860,
    79173563514892,
    36310551370219520,
    9838263126001860672,
    4631002243203661888,
    2419146042248273920,
    312278759507977,
    576473968463120419,
    2254008150401024,
    285890287511552,
    4611687119147238400,
    297519262449467648,
    2307250392955879688,
    2883711145292857600,
    1766552931718402048,
    1171076932706501120,
    3458905424316350466,
    2918614351474724352,
    299527862472933888,
    1289315359920137,
    46448044619080192,
    11673383011910452256,
    55170229373960196,
    3026701592851136516,
    77759661389971744,
    38282802316248328,
    324400473369085952,
    292743734482174528,
    2256215091483400,
    36099337562883136,
    2254009579077760,
    577608659624792080,
    1153575168564658240,
    793275795236782217,
    2396936472658464,
    41660532194033664,
    9223663476189302784,
    1153485146656081920,
    13844067728710699520,
    667694052740695040,
    18577966955036802,
    2271593205072896,
    578713256512848016,
    1416274089869376,
    576568538976223306,
    1666456223762,
    299068271230978,
    2886807945813753892,
    720672706126807040,
    2459000658234380546,
    291634499601702912,
    282575881125890,
    2345086885904,
    18014428591556128,
    9227875756751751200,
    2558185876127630336,
    640711886973700864,
    1288765138346497,
    326525554432016897,
];

pub struct MagicBitboards {
    pub rng: Rng,
    pub rook_masks: [u64; 64],
    pub bishop_masks: [u64; 64],
    pub rook_magics: [u64; 64],
    pub bishop_magics: [u64; 64],
    pub rook_attacks: [[u64; 4096]; 64],
    pub bishop_attacks: [[u64; 512]; 64],
    pub w_pawn_non_capture: [u64; 64],
    pub b_pawn_non_capture: [u64; 64],
    pub w_pawn_capture: [u64; 64],
    pub b_pawn_capture: [u64; 64],
    pub knight_attacks: [u64; 64],
    pub king_attacks: [u64; 64],
}

impl Default for MagicBitboards {
    fn default() -> Self {
        MagicBitboards::new()
    }
}

impl MagicBitboards {
    pub fn new() -> Self {
        let mut magic_bitboards = MagicBitboards {
            rng: Rng::new(),
            rook_masks: [0; 64],
            bishop_masks: [0; 64],
            rook_magics: ROOK_MAGICS,
            bishop_magics: BISHOP_MAGICS,
            rook_attacks: [[0; 4096]; 64],
            bishop_attacks: [[0; 512]; 64],
            w_pawn_non_capture: [0; 64],
            b_pawn_non_capture: [0; 64],
            w_pawn_capture: [0; 64],
            b_pawn_capture: [0; 64],
            knight_attacks: [0; 64],
            king_attacks: [0; 64],
        };
        magic_bitboards.init_masks();
        magic_bitboards.init_attacks();
        magic_bitboards
    }

    pub fn magic_function(bitboard: u64, magic: u64, bit_count: u8) -> u64 {
        bitboard.wrapping_mul(magic) >> (64 - bit_count)
    }

    pub fn generate_magic_numbers(&mut self) {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        self.rng.seed(seed);
        for square in 0..64 {
            self.generate_rook_magic(square as u8);
            self.generate_bishop_magic(square as u8);
        }

        println!("Rook Magics:");
        for square in 0..64 {
            println!("{},", self.rook_magics[square as usize]);
        }

        println!();
        println!("Bishop Magics:");
        for square in 0..64 {
            println!("{},", self.bishop_magics[square as usize]);
        }
    }

    fn init_masks(&mut self) {
        for square in 0..64 {
            self.rook_masks[square] = Self::generate_rook_mask(square as u8);
            self.bishop_masks[square] = Self::generate_bishop_mask(square as u8);
        }
    }

    fn init_attacks(&mut self) {
        self.generate_w_pawn_non_capture();
        self.generate_b_pawn_non_capture();
        self.generate_w_pawn_capture();
        self.generate_b_pawn_capture();
        self.generate_knight_attacks();
        self.generate_king_attacks();
        for square in 0..64 {
            let rook_mask = self.rook_masks[square as usize];
            let bishop_mask = self.bishop_masks[square as usize];
            let rook_occupancies = Self::generate_all_occupancies(rook_mask);
            let bishop_occupancies = Self::generate_all_occupancies(bishop_mask);

            for occupancy in rook_occupancies {
                let index = Self::magic_function(
                    occupancy & rook_mask,
                    self.rook_magics[square as usize],
                    count_bits(rook_mask) as u8,
                ) as usize;
                self.rook_attacks[square as usize][index] =
                    Self::generate_rook_attacks(square as u8, occupancy);
            }
            for occupancy in bishop_occupancies {
                let index = Self::magic_function(
                    occupancy & bishop_mask,
                    self.bishop_magics[square as usize],
                    count_bits(bishop_mask) as u8,
                ) as usize;
                self.bishop_attacks[square as usize][index] =
                    Self::generate_bishop_attacks(square as u8, occupancy);
            }
        }
    }

    fn generate_rook_magic(&mut self, square: u8) {
        let mut magic = self.next_magic_number();
        while !self.valid_rook_magic(square, magic) {
            magic = self.next_magic_number();
        }
        self.rook_magics[square as usize] = magic;
    }

    fn generate_bishop_magic(&mut self, square: u8) {
        let mut magic = self.next_magic_number();
        while !self.valid_bishop_magic(square, magic) {
            magic = self.next_magic_number();
        }
        self.bishop_magics[square as usize] = magic;
    }

    fn valid_rook_magic(&mut self, square: u8, magic: u64) -> bool {
        let mask = self.rook_masks[square as usize];
        let num_bits = count_bits(mask);
        let occupancies = Self::generate_all_occupancies(mask);
        let mut seen = [false; 4096];
        for occupancy in occupancies {
            let index = Self::magic_function(mask & occupancy, magic, num_bits as u8) as usize;
            if seen[index] {
                return false;
            }
            seen[index] = true;
        }
        true
    }

    fn valid_bishop_magic(&mut self, square: u8, magic: u64) -> bool {
        let mask = self.bishop_masks[square as usize];
        let num_bits = count_bits(mask);
        let occupancies = Self::generate_all_occupancies(mask);
        let mut seen = [false; 512];
        for occupancy in occupancies {
            let index = Self::magic_function(mask & occupancy, magic, num_bits as u8) as usize;
            if seen[index] {
                return false;
            }
            seen[index] = true;
        }
        true
    }

    fn generate_all_occupancies(mask: u64) -> Vec<u64> {
        let squares = Self::get_mask_squares(mask);
        let num_occupancies = 1 << squares.len();
        let mut occupancies = Vec::with_capacity(num_occupancies);

        for i in 0..num_occupancies {
            let occupancy = Self::generate_occupancy(&squares, i);
            occupancies.push(occupancy);
        }
        occupancies
    }

    fn generate_occupancy(squares: &[u8], index: usize) -> u64 {
        let mut occupancy = 0;
        for (i, &square) in squares.iter().enumerate() {
            if (index & (1 << i)) != 0 {
                occupancy = set_bit(occupancy, square);
            }
        }
        occupancy
    }

    fn get_mask_squares(mut mask: u64) -> Vec<u8> {
        let mut squares = Vec::with_capacity(count_bits(mask) as usize);
        while mask != 0 {
            let square = first_bit_pop(&mut mask);
            squares.push(square);
        }
        squares
    }

    fn generate_rook_mask(square: u8) -> u64 {
        let mut mask = 0;
        let file = (square % 8) as i8;
        let rank = (square / 8) as i8;

        for f in 1..7 {
            if f != file {
                mask = set_bit(mask, (rank * 8 + f) as u8);
            }
        }

        for r in 1..7 {
            if r != rank {
                mask = set_bit(mask, (r * 8 + file) as u8);
            }
        }
        mask
    }

    fn generate_bishop_mask(square: u8) -> u64 {
        let mut mask = 0;
        let file = (square % 8) as i8;
        let rank = (square / 8) as i8;

        let directions: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (df, dr) in directions.iter() {
            let mut f = file + *df;
            let mut r = rank + *dr;
            while f > 0 && f < 7 && r > 0 && r < 7 {
                mask = set_bit(mask, (r * 8 + f) as u8);
                f += *df;
                r += *dr;
            }
        }
        mask
    }

    fn generate_rook_attacks(square: u8, occupancy: u64) -> u64 {
        let mut attacks = 0;
        attacks |= Self::ray_attacks(square, occupancy, NORTH);
        attacks |= Self::ray_attacks(square, occupancy, WEST);
        attacks |= Self::ray_attacks(square, occupancy, EAST);
        attacks |= Self::ray_attacks(square, occupancy, SOUTH);
        attacks
    }

    fn generate_bishop_attacks(square: u8, occupancy: u64) -> u64 {
        let mut attacks = 0;
        attacks |= Self::ray_attacks(square, occupancy, NORTH_WEST);
        attacks |= Self::ray_attacks(square, occupancy, NORTH_EAST);
        attacks |= Self::ray_attacks(square, occupancy, SOUTH_WEST);
        attacks |= Self::ray_attacks(square, occupancy, SOUTH_EAST);
        attacks
    }

    fn ray_attacks(square: u8, occupancy: u64, direction: Direction) -> u64 {
        let mut attacks = 0;
        let mut f = (square % 8) as i8 + direction.0;
        let mut r = (square / 8) as i8 + direction.1;

        while (0..8).contains(&f) && (0..8).contains(&r) {
            let target_square = (r * 8 + f) as u8;
            if is_bit_set(occupancy, target_square) {
                attacks |= set_bit(attacks, target_square);
                break;
            }
            attacks |= set_bit(attacks, target_square);
            f += direction.0;
            r += direction.1;
        }
        attacks
    }

    fn generate_w_pawn_non_capture(&mut self) {
        let direction = NORTH;
        for square in 0..64 {
            let mut file = (square % 8) as i8 + direction.0;
            let mut rank = (square / 8) as i8 + direction.1;
            let to_square = (rank * 8 + file) as u8;
            if !(A8..=H8).contains(&square) {
                self.w_pawn_non_capture[square as usize] = set_bit(0, to_square);
            }

            if (A2..=H2).contains(&square) {
                file = (square % 8) as i8 + direction.0;
                rank = (square / 8) as i8 + direction.1 * 2;
                let to_square = (rank * 8 + file) as u8;
                self.w_pawn_non_capture[square as usize] |= set_bit(0, to_square);
            }
        }
    }

    fn generate_b_pawn_non_capture(&mut self) {
        let direction = SOUTH;
        for square in 0..64 {
            let mut file = (square % 8) as i8 + direction.0;
            let mut rank = (square / 8) as i8 + direction.1;
            let to_square = (rank * 8 + file) as u8;
            if !(A1..=H1).contains(&square) {
                self.b_pawn_non_capture[square as usize] = set_bit(0, to_square);
            }

            if (A7..=H7).contains(&square) {
                file = (square % 8) as i8 + direction.0;
                rank = (square / 8) as i8 + direction.1 * 2;
                let to_square = (rank * 8 + file) as u8;
                self.b_pawn_non_capture[square as usize] |= set_bit(0, to_square);
            }
        }
    }

    fn generate_w_pawn_capture(&mut self) {
        let directions = [NORTH_WEST, NORTH_EAST];
        for square in 0..64 {
            for &direction in &directions {
                let file = (square % 8) as i8 + direction.0;
                let rank = (square / 8) as i8 + direction.1;
                if (0..8).contains(&file) && (0..8).contains(&rank) {
                    let to_square = (rank * 8 + file) as u8;
                    self.w_pawn_capture[square as usize] |= set_bit(0, to_square);
                }
            }
        }
    }

    fn generate_b_pawn_capture(&mut self) {
        let directions = [SOUTH_WEST, SOUTH_EAST];
        for square in 0..64 {
            for &direction in &directions {
                let file = (square % 8) as i8 + direction.0;
                let rank = (square / 8) as i8 + direction.1;
                if (0..8).contains(&file) && (0..8).contains(&rank) {
                    let to_square = (rank * 8 + file) as u8;
                    self.b_pawn_capture[square as usize] |= set_bit(0, to_square);
                }
            }
        }
    }

    fn generate_knight_attacks(&mut self) {
        for square in 0..64 {
            let mut attacks = 0;
            let file = (square % 8) as i8;
            let rank = (square / 8) as i8;

            for &(df, dr) in &KNIGHT_MOVES {
                let f = file + df;
                let r = rank + dr;
                if (0..8).contains(&f) && (0..8).contains(&r) {
                    attacks |= set_bit(attacks, (r * 8 + f) as u8);
                }
            }
            self.knight_attacks[square as usize] = attacks;
        }
    }

    fn generate_king_attacks(&mut self) {
        for square in 0..64 {
            let mut attacks = 0;
            let file = (square % 8) as i8;
            let rank = (square / 8) as i8;

            for &(df, dr) in &KING_MOVES {
                let f = file + df;
                let r = rank + dr;
                if (0..8).contains(&f) && (0..8).contains(&r) {
                    attacks |= set_bit(attacks, (r * 8 + f) as u8);
                }
            }
            self.king_attacks[square as usize] = attacks;
        }
    }

    fn next_magic_number(&mut self) -> u64 {
        self.rng.next_u64() & self.rng.next_u64() & self.rng.next_u64()
    }
}
