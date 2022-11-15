use std::fs::File;
use std::io::{Error, Write};
use std::ops::{Add, Sub, Mul, Div};

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

    fn save_to_file(&self, file_name: &str) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        writeln!(file, "P6")?;
        writeln!(file, "{}", format!("{} {}", self.width, self.height))?;
        writeln!(file, "255")?;

        self.pixels
            .iter()
            .for_each(|row: &Vec<(u8, u8, u8)>| {
                row
                    .iter()
                    .for_each(|pixel: &(u8, u8, u8)| {
                        write!(file, "{}", format!("{} {} {} ", pixel.0, pixel.1, pixel.2));
                    });
                writeln!(file, "");
            });

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Complex {
    real: f64,
    img: f64
}

impl Complex {
    fn abs(self) -> f64 {
        ((self.real * self.real) + (self.img * self.img)).sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { real: self.real + other.real, img: self.img + other.img}
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { real: self.real - other.real, img: self.img - other.img}
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { 
            real: (self.real * other.real) - (self.img * other.img),
            img: (self.img * other.real) + (self.real * other.img)
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self { 
            real: ((self.real * other.real) + (self.img * other.img)) / ((other.real * other.real) + (other.img * other.img)),
            img: ((self.img * other.real) - (self.real * other.img)) / ((other.real * other.real) + (other.img * other.img))
        }
    }
}

fn is_stable(c: Complex, number_of_iter: u32) -> bool {
    let mut z = Complex {real: 0.0, img: 0.0};
    (0..number_of_iter)
        .for_each(|i| {
            z = (z * z) + c;
        });
    z.abs() <= 2.0
}

fn fractal(n: usize) -> Image {
    let mut fract = Image::new_white(n, n);

    let vals: Vec<f64> = (-(n as i32 / 2 )..(n as i32/2))
        .map(|i| {
            f64::from(i as u32) * 0.05
        })
        .collect();
    
    vals
        .iter()
        .enumerate()
        .for_each(|(i, re)| {
            vals
                .iter()
                .enumerate()
                .for_each(|(j, im)| {
                    if is_stable(Complex {real: *re, img: *im}, 8) {
                        fract.change_pixel_color(i, j, (0, 0, 0));
                    }
                })
        });

    fract
}

fn main() {
    let mut img = Image::new(2, 2, vec![vec![(1, 2, 3), (4, 5, 6)], vec![(7, 8, 9), (10, 11, 12)]]);

    let img_white = Image::new_white(250, 250);

    img.print();
    //img_white.print();

    img.change_pixel_color(1, 0, (0, 0, 0));

    img.print();

    println!("pixel color: {:?}", img.get_pixel_color(0, 0));

    //img_white.save_to_file("test.ppm");

    //=========================================================================================================

    let z = Complex {real: 3.0, img: 5.0};
    let zz = Complex {real: 7.0, img: 11.0};

    let a = z + zz;

    let z2 = Complex {real: 10.0, img: 7.0};
    let zz2 = Complex {real: 5.0, img: 4.0};

    let a2 = z2 - zz2;

    let z3 = Complex {real: 5.0, img: 2.0};
    let zz3 = Complex {real: 3.0, img: -7.0};

    let a3 = z3 * zz3;

    println!("addition : {:?}", a);
    println!("sub : {:?}", a2);
    println!("mul : {:?}", a3);

    //=====================================================================================================


    let f = fractal(100);
    f.save_to_file("fractal.ppm"); 
}
