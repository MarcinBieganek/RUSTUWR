
struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Vec<(u8, u8, u8)>>
}

impl Image {
    fn new_white(width: usize, height: usize) -> Image {
        Image {
            width: width,
            height: height,
            pixels: vec![vec![(255, 255, 255); width]; height]
        }
    }

    fn new(width: usize, height: usize, pixels: Vec<Vec<(u8, u8, u8)>>) -> Image {
        if pixels.len() != height {
            panic!("Tried to create Image with wrong height input!");
        }
        pixels
            .iter()
            .for_each(|row: &Vec<(u8, u8, u8)>| {
                if row.len() != width {
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

    fn change_pixel_color(&mut self, row: usize, column: usize, rgb: (u8, u8, u8)) {
        self.pixels[row][column] = rgb;
    }

    fn get_pixel_color(&self, row: usize, column: usize) -> (u8, u8, u8) {
        self.pixels[row][column]
    }
}

fn main() {
    let mut img = Image::new(2, 2, vec![vec![(1, 2, 3), (4, 5, 6)], vec![(7, 8, 9), (10, 11, 12)]]);

    let img_white = Image::new_white(2, 2);

    img.print();
    //img_white.print();

    img.change_pixel_color(1, 0, (0, 0, 0));

    img.print();

    println!("pixel color: {:?}", img.get_pixel_color(0, 0));
}
