use std::{fs, vec};

pub struct Mask {
    pub row: usize,
    pub col: usize,
    pub bits: Vec<Vec<u8>>,
}

#[derive(PartialEq)]
pub enum MaskType {
    Image,
    Text,
}

impl Mask {
    pub fn new(mask: &str, mask_type: MaskType) -> Mask {
        let ((width, height), bits) = match mask_type {
            MaskType::Image => create_image_mask(mask),
            MaskType::Text => create_text_mask(mask)
        };
        Mask {
            row: height as usize,
            col: width as usize,
            bits
        }
    }
}

fn create_image_mask(image_path: &str) -> ((u32, u32), Vec<Vec<u8>>) {
    let image = image::open(&image_path)
        .expect("No image found at provided path")
        .to_rgba8();

    let (width, height) = image.dimensions();
    let mut mask = vec![];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            let rgba = image.get_pixel(x, y);
            println!("{:?}", rgba);
            if rgba[0] == 250 {
                row.push(1u8);
            } else {
                row.push(0u8);
            }
        }
        mask.push(row);
    }

    ((width, height), mask)
}

fn create_text_mask(text_path: &str) -> ((u32, u32), Vec<Vec<u8>>) {
    let data = fs::read(text_path).expect("Unable to read file");
    let line_break = data.iter().position(|&x| x == 10).unwrap();
    let mut bits = vec![];
    let mut nd = vec![];

    for x in data.iter() {
        if *x != '\n' as u8 {
            nd.push(x);
        }
    }

    for row in nd.chunks(line_break) {
        let mut x = vec![];
        for c in row {
            if **c == 'X' as u8 {
                x.push(0);
            } else {
                x.push(1);
            }
        }
        bits.push(x);
    }

    let height = bits.len();
    let width = bits[0].len();

    ((width as u32, height as u32), bits)
}
