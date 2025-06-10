use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


const DIMENSIONS:i32 = 100;


#[derive(Clone,PartialEq)]
struct Embedding{
    values: Vec<f32>, // probably way too precise, don't actually need mantissa and all that, just a sign bit and the rest can be "like" u8/u16 -> TODO
}

impl From<Vec<f32>> for Embedding{
    fn from(value: Vec<f32>) -> Self {
        if value.len() != 100{
            panic!("Invalid vector, length not 100");
        }else{
            // let arr: [f32;100] = value.iter().try_into().;
            Self{values:value}
        }
    }
}


struct Parser{
    map: HashMap<String,Embedding>,
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dot_product() {
        let parser = Parser{map:HashMap::new()};

        let v1:Vec<f32> = vec![-1.0,2.1,-2.3];
        let v2:Vec<f32> = vec![3.0,1.9,-1.2];

        let res = parser.dot_product(&v1, &v2);
        assert_eq!(res,3.7499998);
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
    fn parse_line(&self,line:&str)->(String,Embedding){
        let line_segments = line.split_once(" ").unwrap();
        let word = line_segments.0.to_string();

        let values: Vec<f32> = line_segments.1.split(" ").map(|x|{
            let i:f32 = x.parse().expect("Found corrupt value in file");
            i
        }).collect();
        (word, Embedding{values:values})
    }

    fn add_embedding(&mut self, pair:(String,Embedding)){
        self.map.insert(pair.0, pair.1);
    }

    fn dot_product(&self, v1:&Vec<f32>, v2:&Vec<f32>)->f64{
        v1.iter().zip(v2).map(|(x1,x2)|{
            (x1 * x2) as f64
        }).sum()
    }
    fn vec_mag(&self, v1:&Vec<f32>) -> f64{
        (v1.iter().map(|x|x*x).sum::<f32>() as f64).sqrt()

    }

    fn check_similarity(&self,word_1:&str, word_2:&str)->Result<f64,WordNotFoundError>{
        // lookups
        let v1: &Vec<f32> = &self.map.get(word_1).ok_or(WordNotFoundError{})?.values;
        let v2: &Vec<f32> = &self.map.get(word_2).ok_or(WordNotFoundError{})?.values;


        let sim = (
            self.dot_product(&v1, &v2)
        )/(
            self.vec_mag(v1) * self.vec_mag(v2)
        );
        Ok(sim)
    }

    fn print_sim(&self,word_1:&str, word_2:&str){
        let res = self.check_similarity(word_1, word_2);
        match res {
            Ok(r) =>{
                println!("{} -> {}: {}",word_1,word_2,r)
            },
            Err(WordNotFoundError)=>{
                println!("Couldn't find {} or {}",word_1,word_2)
            }
        }

    }
}


fn main() {
    let f = File::open("./English/en.vectors").expect("Download word2vec vectors under ./English/en.vectors");
    let mut reader = BufReader::new(f);

    let mut parser = Parser{
        map: HashMap::new()
    };

    // TODO: make this max line length
    let mut line = String::new();

    let empty_str = String::from("");

    let mut skipped_header = false;
    loop{
        if let Ok(_len) = reader.read_line(&mut line){
            // EOF
            if _len == 0{continue;}


            // if line is still empty, asssume we are handling the first one and just skip
            if !skipped_header {skipped_header = true; line = String::from(""); continue}

            let res = parser.parse_line(&line.trim());
            parser.add_embedding(res);
            line = String::from("")

        }else { break;}
    }
    println!("[!] All vecs loaded into memory");

    parser.print_sim("computer", "machine");
    parser.print_sim("man", "woman");
    parser.print_sim("man", "boy");
    parser.print_sim("man", "king");

    parser.print_sim("france", "germany");
    parser.print_sim("france", "china");






}
