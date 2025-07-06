use crate::board_state::c_move::CMove;

#[derive(Clone, Copy)]
pub struct TranspositionEntry {
    pub zobrist_hash: u64,
    pub best_move: CMove,
}

pub struct TranspositionTable {
    pub entries: Box<[TranspositionEntry]>,
    pub length: usize,
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        let length = size / size_of::<TranspositionEntry>();
        TranspositionTable {
            entries: vec![
                TranspositionEntry {
                    zobrist_hash: 0,
                    best_move: CMove::new(),
                };
                length
            ]
            .into_boxed_slice(),
            length,
        }
    }

    pub fn clear(&mut self) {
        self.entries = vec![
            TranspositionEntry {
                zobrist_hash: 0,
                best_move: CMove::new(),
            };
            self.length
        ]
        .into_boxed_slice();
    }

    pub fn add_entry(&mut self, entry: TranspositionEntry) {
        let index = (entry.zobrist_hash as usize) % self.length;
        self.entries[index] = entry;
    }

    pub fn get_entry(&self, zobrist_hash: u64) -> Option<TranspositionEntry> {
        let index = (zobrist_hash as usize) % self.length;
        let entry = self.entries[index];
        if entry.zobrist_hash == zobrist_hash {
            Some(entry)
        } else {
            None
        }
    }
}
