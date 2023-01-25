mod utils;

use wasm_bindgen::prelude::*;

use std::ops::{Add, Sub, Mul, Div};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Fractal {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

impl Fractal {
    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width * 3 + (column * 3)) as usize
    }
}

#[wasm_bindgen]
impl Fractal {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        let vals: Vec<f64> = (-(self.width as i32 / 2 )..(self.width as i32/2))
            .map(|i| {
                f64::from(i as i32) * 0.01
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
                        if is_stable(Complex {real: *re, img: *im}, 30) {
                            let idx = self.get_index(i, j);
                            
                            next[idx] = 0; // red
                            next[idx+1] = 0; // green
                            next[idx+2] = 0; // blue
                        }
                    })
            });

        self.cells = next;
    }

    pub fn new() -> Fractal {
        let width = 1024;
        let height = 1024;

        let cells = (0..width * height * 3)
            .map(|i| {
                255
            })
            .collect();

        Fractal {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}


use std::fmt;

impl fmt::Display for Fractal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = cell;
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

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