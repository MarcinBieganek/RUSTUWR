
struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<(u8, u8, u8)>>
}

impl Image {
    fn new_white(width: u32, height: u32) -> Image {
        Image {
            width: width,
            height: height,
            pixels: vec![vec![(255, 255, 255); width as usize]; height as usize]
        }
    }

    fn new(width: u32, height: u32, pixels: Vec<Vec<(u8, u8, u8)>>) -> Image {
        if pixels.len() != (height as usize) {
            panic!("Tried to create Image with wrong height input!");
        }
        pixels
            .iter()
            .for_each(|row: &Vec<(u8, u8, u8)>| {
                if row.len() != (width as usize) {
                    panic!("Tried to create Image with wrong width input!");
                }
            });
        Image {
            width: width,
            height: height,
            pixels: pixels
        }
    }

    fn print(&self) {
        println!("IMG width: {}", self.width);
        println!("IMG height: {}", self.height);
        println!("IMG content: {:?}", self.pixels);
    }
}

fn main() {
    let img = Image::new(2, 2, vec![vec![(1, 2, 3), (4, 5, 6)], vec![(7, 8, 9), (10, 11, 12)]]);

    let img_white = Image::new_white(2, 2);

    img.print();
    img_white.print();
}
