use std::collections::HashMap;
use std::hash::RandomState;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::BufWriter;
use std::mem::size_of;

mod hmap;

#[repr(C,packed(1))]
struct Embedding{
    values: [f32;100], // probably way too precise, don't actually need mantissa and all that, just a sign bit and the rest can be "like" u8/u16 -> TODO
}

// We will then just scale them all into i16 for less mantissa and stuff wasted
// impl From<Vec<f32>> for Embedding{
//     fn from(value: Vec<f32>) -> Self {
//         if value.len() != 100{
//             panic!("Invalid vector, length not 100");
//         }else{
//             // let arr: [f32;100] = value.iter().try_into().;
//             Self{values:value}
//         }
//     }
// }

#[repr(C)]
struct Parser{
    map: HashMap<String,Embedding>,
}

#[cfg(test)]
mod tests {
    // TODO: we can surely improve this with SIMD and other processor specific operations, have supervise compiler here
    use super::*;

    #[test]
    fn test_dot_product() {
        let parser = Parser{map:HashMap::new()};

        let v1:Vec<f32> = vec![-1.0,2.1,-2.3];
        let v2:Vec<f32> = vec![3.0,1.9,-1.2];

        let res = parser.dot_product(&v1, &v2);
        assert_eq!(res,3.749999761581421);
    }

    #[test]
    fn test_vec_mag() {
        let parser = Parser{map:HashMap::new()};

        let v1:Vec<f32> = vec![1.0,2.0,3.0,-5.0];

        let res = parser.vec_mag(&v1);
        assert_eq!(res,6.244997998398398);
    }

}

struct WordNotFoundError{}


impl Parser{
    fn init()->Self{
        // IMPORTANT: the capacity is a match for the ENGLISH words list (2**22 > 4027169)
        // IDEA: we could loop over seeds to find one that has the least collisions and use that -- don't really have to worry about hashDOS
        Self { map: HashMap::with_capacity_and_hasher(4194304, RandomState::new())}

    }


    fn insert_line(&mut self,line:&str){

        let line_segments = line.split_once(" ").unwrap();
        let word = line_segments.0;

        let values = line_segments.1.split(" ").map(|x|{
            let i:f32 = x.parse().expect("Found corrupt value in file");
            i
        }).into_iter();

        let mut emb = Embedding{values:[0.0;100]};
        for (pos,v) in values.enumerate(){
            emb.values[pos] = v;
        }

        self.map.insert(word.to_string(), emb);
    }


    fn dot_product(&self, v1:&[f32;100], v2:&[f32;100])->f64{
        v1.iter().zip(v2).map(|(x1,x2)|{
            (x1 * x2) as f64
        }).sum()
    }

    fn vec_mag(&self, v1:&[f32;100]) -> f64{
        (v1.iter().map(|x|x*x).sum::<f32>() as f64).sqrt()

    }

    fn check_similarity(&self,word_1:&str, word_2:&str)->Result<f64,WordNotFoundError>{
        let v1 = self.map.get(word_1).ok_or(WordNotFoundError{})?.values;
        let v2 = self.map.get(word_2).ok_or(WordNotFoundError{})?.values;


        let sim = (
            self.dot_product(&v1, &v2)
        )/(
            self.vec_mag(&v1) * self.vec_mag(&v2)
        );
        Ok(sim)
    }

    fn print_sim(&self,word_1:&str, word_2:&str){
        let res = self.check_similarity(word_1, word_2);
        match res {
            Ok(r) =>{
                println!("{} -> {}: {}",word_1,word_2,r)
            },
            Err(_)=>{
                println!("Couldn't find {} or {}",word_1,word_2)
            }
        }

    }
}


//

fn main() {
    let f = File::open("./English/en.vectors").expect("Download word2vec vectors under ./English/en.vectors");
    let mut reader = BufReader::new(f);

    let mut parser = Parser{
        map: HashMap::new()
    };

    // TODO: make this max line length
    let mut line = String::new();



    let mut skipped_header = false;
    loop{
        if let Ok(_len) = reader.read_line(&mut line){
            // EOF
            if _len == 0{continue;}


            // if line is still empty, asssume we are handling the first one and just skip
            if !skipped_header {skipped_header = true; line = String::from(""); continue}

            parser.insert_line(&line.trim());
            // let res = parser.parse_line(&line.trim());
            // parser.add_embedding(res);
            line = String::from("")

        }else { break;}
    }
    println!("[!] All vecs loaded into memory");
    println!("[...] Writing Parser struct to binary file...");

    // let f = File::open("./vectors.bin").expect("Want to write to binary file...");
    // let writer = BufWriter::new(f);


    let s = size_of::<Parser>();
    println!("Size of parser: {}",s);

    // parser.print_sim("computer", "machine");
    // parser.print_sim("man", "woman");
    // parser.print_sim("man", "boy");
    // parser.print_sim("man", "king");

    // parser.print_sim("france", "germany");
    // parser.print_sim("france", "china");

    // thread::sleep(Duration::from_secs(30));


}
