pub fn create_mask(image_path: &str) -> ((u32,u32), Vec<Vec<u8>>){
    let image = image::open(&image_path)
    .expect("No image found at provided path")
    .to_rgba8();

    let (width, height) = image.dimensions();
    let mut mask = vec![];
    for y in 0..height{
        let mut row = vec![];
        for x in 0..width{
            let rgba = image.get_pixel(x,y);
            if rgba[0] > 100{
                row.push(0u8);
            }else{
                row.push(1u8);
            }
        }
        mask.push(row);
    }

    println!("{:?}", image.get_pixel(5, 25));
    ((width, height), mask)
}