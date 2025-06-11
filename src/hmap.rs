// Will provide load of 80%, resulting in 2 gb of memory being used
const ENTRIES:u64 = 5033164;
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

#[derive(Clone, Copy,Debug)]
pub struct PlaceHolderEntry{
    pub is_full: bool,
}

// Will currently take up 1.61 GB of memory (of raw vector-data)
pub struct MinimalHashMap{
    entries: Box<[PlaceHolderEntry;ENTRIES as usize]>,
    collisions_counter: u32
}

impl MinimalHashMap{
    pub fn new() -> Self{
        let placeholder = PlaceHolderEntry{is_full:false};
        Self { entries: Box::new([placeholder;ENTRIES as usize]), collisions_counter: 0 }
    }

    pub fn insert(&mut self, word: &str){
        let h = fnv_hash(&word.as_bytes());
        let ind = h % ENTRIES;
        // Check if
        if self.entries[ind as usize].is_full{
            self.collisions_counter += 1;
        }else{
            self.entries[ind as usize].is_full = true;
        }
    }

    fn score_spread(&self)->f64{
        // Clusters give big minus points, otherwise we ideally want equal spacing between the holes
        let ideal_spacing = (ENTRIES as f64) / (self.collisions_counter as f64);

        // Bigger spaces should be weighed heavier, so let's do quadratic (like variance)
        let mut score:f64 = 0.0;
        let mut hole_chunks:usize = 0;
        let mut last_hole_pos = 0;


        let v = self.entries.as_slice();

        for (pos,entry) in v.iter().enumerate(){
            if !entry.is_full{
                if last_hole_pos+1 == pos{
                    hole_chunks += 1;
                }
                let dist = pos - last_hole_pos;
                score += (dist * dist) as f64;
                last_hole_pos = pos;
            }
        };



        let default_spacing_variance = score/(self.collisions_counter as f64);

        let final_score = (default_spacing_variance - ideal_spacing).powi(3);
        final_score

    }

    pub fn info(&self){
        let col_perc = self.collisions_counter as f64/(ENTRIES as f64)*100.0;
        println!("Collisions: {} (which is {:.6}% of total)",self.collisions_counter,col_perc);

        let score_spread = self.score_spread();
        println!("Spread-score: {:.5}",score_spread);
        // I want also want to know how spread apart the "holes" are because better distribution => better for linear probing

    }

}