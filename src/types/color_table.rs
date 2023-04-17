use crate::read_ext::ReadExt;
use std::io::Read;

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct ColorTable(Vec<Rgb>);

impl ColorTable {
    pub fn get<T: Into<usize>>(&self, i: T) -> Option<&Rgb> {
        self.0.get(i.into())
    }

    pub(crate) fn from_bytes<T: Read>(reader: &mut T, size: usize) -> std::io::Result<ColorTable> {
        let size = size * 3;

        let color_table = reader
            .read_bytes_vec(size)
            .unwrap()
            .chunks(3)
            .map(|x| Rgb {
                r: x[0],
                g: x[1],
                b: x[2],
            })
            .collect();

        Ok(Self(color_table))
    }
}
