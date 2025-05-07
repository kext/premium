use embedded_hal_async::i2c::I2c;
use premium_pixel::Surface;

/// SSH1106 OLED Display
pub struct Ssh1106<I> {
    buffer: [u8; 1032],
    i2c: I,
}

const ADDRESS: u8 = 0x3c;

impl<I: I2c> Ssh1106<I> {
    /// Initialize the display
    pub async fn new(mut i2c: I) -> Result<Self, I::Error> {
        i2c.write(
            ADDRESS,
            &[
                0x00, 0xae, 0xd5, 0x80, 0xa8, 0x3f, 0xd3, 0x00, 0x40, 0x8d, 0x14, 0x20, 0x00, 0xa1,
                0xc8, 0xda, 0x12, 0x81, 0xcf, 0xd9, 0xf1, 0xdb, 0x40, 0xa4, 0xa6, 0xaf,
            ],
        )
        .await?;
        let mut display = Self {
            buffer: [0; 1032],
            i2c,
        };
        display.clear();
        Ok(display)
    }
    /// Send the current buffer to the display
    pub async fn display(&mut self) -> Result<(), I::Error> {
        for i in 0..8 {
            self.i2c
                .write(ADDRESS, &[0x00, 0xb0 + i as u8, 0x02, 0x10])
                .await?;
            self.i2c
                .write(ADDRESS, &self.buffer[i * 129..(i + 1) * 129])
                .await?;
        }
        Ok(())
    }
}

impl<I: I2c> Surface for Ssh1106<I> {
    fn clear(&mut self) {
        self.buffer.fill(0);
        for i in 0..8 {
            self.buffer[i * 129] = 0x40;
        }
    }
    fn pixel(&mut self, x: i32, y: i32) {
        if (0..self.width()).contains(&x) && (0..self.height()).contains(&y) {
            let p = (x + (y >> 3) * 128) as usize;
            self.buffer[p + 1 + p / 128] |= 1 << (y & 7);
        }
    }
    fn width(&self) -> i32 {
        128
    }
    fn height(&self) -> i32 {
        64
    }
}
