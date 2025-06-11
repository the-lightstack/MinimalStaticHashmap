// Will provide load of 96%
const ENTRIES:u64 = 4194304;
const DIMENSIONS: u64 = 100;


const INITIAL_STATE: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0100_0000_01b3;

// taken from: https://github.com/servo/rust-fnv/blob/main/lib.rs
#[inline]
#[must_use]
pub const fn fnv_hash(bytes: &[u8]) -> u64 {
    let mut hash = INITIAL_STATE;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(PRIME);
        i += 1;
    }
    hash
}



// struct EmbeddingEntry{
//     vectors: [f32; DIMENSIONS],
// }

struct PlaceHolderEntry{
    is_full: bool,
}


// Will currently take up 1.61 GB of memory (of raw vector-data)
pub struct MinimalHashMap{
    entries: [PlaceHolderEntry;ENTRIES as usize],
    collisions_counter: u32
}

impl MinimalHashMap{
    fn insert(&mut self, word: &str){
        let h = fnv_hash(&word.as_bytes());
        let ind = h % ENTRIES;
        // Check if
        if self.entries[ind as usize].is_full{
            self.collisions_counter += 1;
        }else{
            self.entries[ind as usize].is_full = true;
        }


    }

}