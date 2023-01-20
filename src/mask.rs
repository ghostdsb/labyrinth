use std::{fs, vec};

use crate::image_scanner;
pub struct Mask{
    pub row: usize,
    pub col: usize,
    pub bits: Vec<Vec<u8>>
}

impl Mask{
    pub fn new( mask: &str) -> Mask{
        // let data = fs::read(mask).expect("Unable to read file");
        // let line_break = data.iter().position(|&x| x == 10).unwrap();
        // let mut bits = vec!();
        // let mut nd = vec!();

        // for x in data.iter(){
        //     if *x != '\n' as u8{
        //         nd.push(x);
        //     }
        // }

        // for row in nd.chunks(line_break){
        //     let mut x = vec!();
        //     for c in row{
        //         if **c == 'X' as u8{
        //             x.push(0);
        //         }else{
        //             x.push(1);
        //         }
        //     }
        //     bits.push(x);
        // }

        // let height = bits.len();
        // let width = bits[0].len();

        let ((width, height), bits) = image_scanner::create_mask(mask);

        Mask{
            row: height as usize, 
            col: width as usize,
            bits: bits
        }
    }
}