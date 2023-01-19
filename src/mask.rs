use std::{fs, vec};
pub struct Mask{
    pub row: usize,
    pub col: usize,
    pub bits: Vec<Vec<u8>>
}

impl Mask{
    pub fn new( mask: &str) -> Mask{
        let data = fs::read(mask).expect("Unable to read file");
        let line_break = data.iter().position(|&x| x == 10).unwrap();
        let mut bits = vec!();
        let mut nd = vec!();

        for x in data.iter(){
            if *x != 10{
                nd.push(x);
            }
        }

        for row in nd.chunks(line_break){
            let mut x = vec!();
            for c in row{
                println!("{:?}", c);
                if **c == 88{
                    x.push(0);
                }else if **c == 79{
                    x.push(1);
                }
            }
            bits.push(x);
        }

        Mask{
            row: bits.len(), 
            col: bits[0].len(),
            bits
        }
    }

    pub fn alive_count(&self)-> u32{
        let mut count = 0;
        for i in 0..self.row{
            for j in 0..self.col{
                if self.bits[i][j] == 1{
                    count += 1;
                }
            }
        }
        count
    }
}