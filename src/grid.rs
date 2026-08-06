use std::io::Write;

use bitvec::{order::Lsb0, view::BitView};

use crate::led;

#[derive(Clone, Debug)]
pub struct Grid {
    data: [u8; 42],
}
impl Grid {
    const LEN: usize = 42;

    pub fn get(&self, index: usize) -> Option<bool> {
        self.data[led::COMMAND_LEN..]
            .view_bits::<Lsb0>()
            .get(index)
            .map(|b| *b)
    }
    pub fn get_xy(&self, x: usize, y: usize) -> Option<bool> {
        self.get(x + y * led::WIDTH)
    }

    pub fn set(&mut self, index: usize, value: bool) {
        self.data[led::COMMAND_LEN..]
            .view_bits_mut::<Lsb0>()
            .set(index, value)
    }
    pub fn set_xy(&mut self, x: usize, y: usize, value: bool) {
        self.set(x + y * led::WIDTH, value)
    }

    pub fn clear(&mut self) {
        self.data[led::COMMAND_LEN..Self::LEN].copy_from_slice(&[0; Self::LEN - led::COMMAND_LEN]);
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), impl std::error::Error + use<W>> {
        w.write_all(&self.data)
    }
}

impl Default for Grid {
    fn default() -> Self {
        let mut data = [0; 42];

        data[..led::COMMAND_LEN].copy_from_slice(&led::COMMAND_BYTES);

        Self { data }
    }
}
