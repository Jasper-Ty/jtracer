use na::{Point3, point};
use std::fs::File;
use std::io;
use io::prelude::*;
use std::ops::{Index, IndexMut};

pub struct ImBuf {
    pub pixels: Vec<Point3<u8>>,
    pub width: usize,
    pub height: usize,
}
impl ImBuf {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![point![0, 0, 0]; width*height],
            width,
            height
        }
    }
    pub fn to_ppm(&self, f: &mut File) -> std::io::Result<()> {
        write!(f, "P6\n")?;
        write!(f, "{} {}\n", self.width, self.height)?;
        write!(f, "255\n")?;
        for j in 0..self.height { 
            for i in 0..self.width {
                let j = self.height - j - 1;
                let pix = [self[(i,j)][0], self[(i,j)][1], self[(i,j)][2]];
                f.write_all(&pix)?;
            }
        }
        Ok(())
    }
}
impl Index<(usize, usize)> for ImBuf {
    type Output = Point3<u8>;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.pixels[j*(self.height)+i]
    }
}
impl IndexMut<(usize, usize)> for ImBuf {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[j*(self.height)+i]
    }
}
