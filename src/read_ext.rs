use std::io::{Read, Result};

pub trait ReadExt {
    fn read_byte(&mut self) -> Result<u8>;
    fn read_bytes<const SIZE: usize>(&mut self) -> Result<[u8; SIZE]>;
    fn read_bytes_vec<T: Into<usize>>(&mut self, size: T) -> Result<Vec<u8>>;
}

impl<T: Read> ReadExt for T {
    fn read_byte(&mut self) -> Result<u8> {
        let mut bytes = [0; 1];
        self.read_exact(&mut bytes)?;

        Ok(bytes[0])
    }

    fn read_bytes<const SIZE: usize>(&mut self) -> Result<[u8; SIZE]> {
        let mut bytes = [0; SIZE];
        self.read_exact(&mut bytes)?;

        Ok(bytes)
    }

    fn read_bytes_vec<T1: Into<usize>>(&mut self, size: T1) -> Result<Vec<u8>> {
        let mut bytes = vec![0; size.into()];
        self.read_exact(&mut bytes)?;

        Ok(bytes)
    }
}

pub trait ReadSubBlockExt {
    fn read_subblock(&mut self) -> Result<Vec<u8>>;
}

impl<T: Read> ReadSubBlockExt for T {
    fn read_subblock(&mut self) -> Result<Vec<u8>> {
        let mut data = Vec::default();

        let mut block_size = self.read_byte()?;
        while block_size != 0 {
            let buffer = self.read_bytes_vec(block_size)?;
            data.push(buffer);
            block_size = self.read_byte()?;
        }

        Ok(data.into_iter().flatten().collect())
    }
}
