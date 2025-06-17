use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Read}, vec};




pub struct BinaryParser{
    pub(crate) map: HashMap<String, [f32;100]>,
}

impl BinaryParser{
    fn bytes_to_f32(&self, bytes: &[u8]) -> f32 {
        assert!(bytes.len() == 4, "Slice must be exactly 4 bytes long");
        f32::from_le_bytes(bytes.try_into().unwrap())
    }

    pub fn parse_file(&mut self, file:&File){
        let mut reader = BufReader::new(file);

    // loop over all words with vecs in file
    let mut counter = 0;
    loop{

        // let mut word_buf:Vec<u8> = Vec::with_capacity(100);
        let mut str_len_buffer:Vec<u8> = vec![0];
        reader.read(&mut str_len_buffer).unwrap();

        // This feels criminally slow and stupid. I should only need one 100 byte-cap buffer and fill that
        let mut word_buf = vec![0u8;*str_len_buffer.get(0).unwrap() as usize];
        reader.read_exact(&mut word_buf).unwrap();



        // // loop over letters until we find null byte
        // loop{
        //     match reader.read(&mut one_byte_buf){
        //         Ok(bytes_read) =>{
        //             if bytes_read == 0{break;}
        //             // Check if we found end of word
        //             let byte = *one_byte_buf.first().unwrap();
        //             if  byte == 0x0{
        //                 break;
        //             }else{
        //                 word_buf.push(byte);
        //             }
        //         },
        //         Err(_) => panic!("Failed on error while reading")
        //     }
        // }



        // We should now have word as u8 buffer

        // and now read 400 bytes for vectors
        let mut vector_buffer:Vec<u8> = vec![0u8;400];
        match reader.read_exact(&mut vector_buffer){
            Ok(_) => {
                let mut f32_vecs: [f32;100] = [0.0;100];
                for i in 0..100{
                    let raw_bytes = &vector_buffer[i*4..(i+1)*4];
                    f32_vecs[i] = self.bytes_to_f32(raw_bytes);
                }
        
                // Insert into hash-map
                let word_s:String = match String::from_utf8(word_buf){
                    Ok(s) =>s,
                    Err(_) => {
                        println!("Skipped one because illegal");
                        continue;
                    },
                };
                self.map.insert(word_s, f32_vecs) ;


                counter +=1;
            },
            Err(_) => {
                println!("Quit after reading {} entries",counter);
                return;
            }

        }
}
}}