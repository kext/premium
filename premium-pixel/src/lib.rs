//! Platform independent code for drawing to monochrome surfaces

#![no_std]
#![warn(missing_docs)]

mod awakening;
mod digits;
mod premium;
mod utils;

pub use awakening::Awakening;
pub use digits::large::DigitsLarge;
pub use digits::medium::DigitsMedium;
pub use premium::Premium;
pub use utils::*;

/// A drawable surface
pub trait Surface {
    /// Clear all pixels
    fn clear(&mut self);
    /// Fill a pixel
    fn pixel(&mut self, x: i32, y: i32);
    /// Get the width of the surface
    fn width(&self) -> i32;
    /// Get the height of the surface
    fn height(&self) -> i32;
    /// Draw a line
    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let dx = (x2 - x1).abs();
        let sx = (x2 - x1).signum();
        let dy = (y2 - y1).abs();
        let sy = (y2 - y1).signum();
        let mut e = if dx > dy { dx / 2 } else { -dy / 2 };
        let mut x = x1;
        let mut y = y1;
        loop {
            self.pixel(x, y);
            if x == x2 && y == y2 {
                break;
            }
            let t = e;
            if t > -dx {
                e -= dy;
                x += sx;
            }
            if t < dy {
                e += dx;
                y += sy;
            }
        }
    }
}
