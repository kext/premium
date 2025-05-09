use crate::Surface;

/// Write on a surface using the Premium typeface
pub trait Premium {
    /// Write the text with the baseline starting at the given position.
    /// Returns the width of the text in pixels.
    fn premium(&mut self, x: i32, y: i32, text: &str) -> i32;
}

impl<S: Surface> Premium for S {
    fn premium(&mut self, x: i32, y: i32, text: &str) -> i32 {
        let mut w = 0;
        for (i, c) in text.chars().enumerate() {
            if i > 0 {
                w += 2;
            }
            w += glyph(self, x + w, y, c);
        }
        w
    }
}

fn glyph<S: Surface>(surface: &mut S, x: i32, y: i32, c: char) -> i32 {
    let (c, w, o) = lookup(c);
    let h = c.len() as i32;
    for (i, l) in c.iter().enumerate() {
        for j in 0..16 {
            if l & (1 << (15 - j)) != 0 {
                surface.pixel(x + j - 2, y - h + o + i as i32);
            }
        }
    }
    w
}

fn lookup(c: char) -> (&'static [u16], i32, i32) {
    match c {
        'A' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'B' => (
            &[
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111000000000,
            ],
            6,
            0,
        ),
        'C' => (
            &[
                0b0001111000000000,
                0b0011000100000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'D' => (
            &[
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111000000000,
            ],
            6,
            0,
        ),
        'E' => (
            &[
                0b0011111100000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011111000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011111100000000,
            ],
            6,
            0,
        ),
        'F' => (
            &[
                0b0011111100000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011111000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            6,
            0,
        ),
        'G' => (
            &[
                0b0001111000000000,
                0b0011000100000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011011100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'H' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'I' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            2,
            0,
        ),
        'J' => (
            &[
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0010001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'K' => (
            &[
                0b0011000110000000,
                0b0011001100000000,
                0b0011011000000000,
                0b0011110000000000,
                0b0011100000000000,
                0b0011110000000000,
                0b0011011000000000,
                0b0011001100000000,
                0b0011000110000000,
            ],
            7,
            0,
        ),
        'L' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011111100000000,
            ],
            6,
            0,
        ),
        'M' => (
            &[
                0b0011000001100000,
                0b0011100011100000,
                0b0011110111100000,
                0b0011111111100000,
                0b0011011101100000,
                0b0011001001100000,
                0b0011000001100000,
                0b0011000001100000,
                0b0011000001100000,
            ],
            9,
            0,
        ),
        'N' => (
            &[
                0b0011000110000000,
                0b0011000110000000,
                0b0011100110000000,
                0b0011110110000000,
                0b0011111110000000,
                0b0011011110000000,
                0b0011001110000000,
                0b0011000110000000,
                0b0011000110000000,
            ],
            7,
            0,
        ),
        'O' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'P' => (
            &[
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            6,
            0,
        ),
        'Q' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
                0b0000001100000000,
            ],
            6,
            1,
        ),
        'R' => (
            &[
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'S' => (
            &[
                0b0001111000000000,
                0b0011000100000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0001111000000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0010001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'T' => (
            &[
                0b0111111000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
            ],
            4,
            0,
        ),
        'U' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'V' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
                0b0000110000000000,
            ],
            6,
            0,
        ),
        'W' => (
            &[
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0001111111100000,
                0b0000110011000000,
            ],
            10,
            0,
        ),
        'X' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'Y' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
            ],
            6,
            0,
        ),
        'Z' => (
            &[
                0b0011111100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000011000000000,
                0b0000110000000000,
                0b0001100000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011111100000000,
            ],
            6,
            0,
        ),
        'a' => (
            &[
                0b0001111000000000,
                0b0000001100000000,
                0b0001111100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111100000000,
            ],
            6,
            0,
        ),
        'b' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111000000000,
            ],
            6,
            0,
        ),
        'c' => (
            &[
                0b0001111000000000,
                0b0011000100000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'd' => (
            &[
                0b0000001100000000,
                0b0000001100000000,
                0b0001111100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111100000000,
            ],
            6,
            0,
        ),
        'e' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111100000000,
                0b0011000000000000,
                0b0011000100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'f' => (
            &[
                0b0001100000000000,
                0b0011000000000000,
                0b0111100000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            3,
            0,
        ),
        'g' => (
            &[
                0b0001111100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111100000000,
                0b0000001100000000,
                0b0010001100000000,
                0b0001111000000000,
            ],
            6,
            3,
        ),
        'h' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'i' => (
            &[
                0b0011000000000000,
                0b0000000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            2,
            0,
        ),
        'j' => (
            &[
                0b0011000000000000,
                0b0000000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0110000000000000,
            ],
            2,
            2,
        ),
        'k' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0011001100000000,
                0b0011011000000000,
                0b0011110000000000,
                0b0011100000000000,
                0b0011110000000000,
                0b0011011000000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'l' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            2,
            0,
        ),
        'm' => (
            &[
                0b0011111111100000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
            ],
            10,
            0,
        ),
        'n' => (
            &[
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'o' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'p' => (
            &[
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            6,
            2,
        ),
        'q' => (
            &[
                0b0001111100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111100000000,
                0b0000001100000000,
                0b0000001100000000,
            ],
            6,
            2,
        ),
        'r' => (
            &[
                0b0011011000000000,
                0b0011111000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            5,
            0,
        ),
        's' => (
            &[
                0b0001111000000000,
                0b0011000100000000,
                0b0011000000000000,
                0b0001111000000000,
                0b0000001100000000,
                0b0010001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        't' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0111100000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0001100000000000,
            ],
            3,
            0,
        ),
        'u' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'v' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
                0b0000110000000000,
            ],
            6,
            0,
        ),
        'w' => (
            &[
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0011001100110000,
                0b0001111111100000,
                0b0000110011000000,
            ],
            10,
            0,
        ),
        'x' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'y' => (
            &[
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111100000000,
                0b0000001100000000,
                0b0010001100000000,
                0b0001111000000000,
            ],
            6,
            3,
        ),
        'z' => (
            &[
                0b0011111100000000,
                0b0000001100000000,
                0b0000011000000000,
                0b0000110000000000,
                0b0001100000000000,
                0b0011000000000000,
                0b0011111100000000,
            ],
            6,
            0,
        ),
        '0' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        '1' => (
            &[
                0b0001100000000000,
                0b0011100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
            ],
            3,
            0,
        ),
        '2' => (
            &[
                0b0001111000000000,
                0b0010001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000011000000000,
                0b0000110000000000,
                0b0001100000000000,
                0b0011000000000000,
                0b0011111100000000,
            ],
            6,
            0,
        ),
        '3' => (
            &[
                0b0011111100000000,
                0b0000011000000000,
                0b0000110000000000,
                0b0001111000000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0010001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        '4' => (
            &[
                0b0000001100000000,
                0b0000011100000000,
                0b0000101100000000,
                0b0001001100000000,
                0b0010001100000000,
                0b0011111110000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
            ],
            6,
            0,
        ),
        '5' => (
            &[
                0b0011111100000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011111000000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0010001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        '6' => (
            &[
                0b0000111000000000,
                0b0001100000000000,
                0b0011000000000000,
                0b0011111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        '7' => (
            &[
                0b0011111100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000001100000000,
                0b0000011000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
            ],
            6,
            0,
        ),
        '8' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        '9' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111100000000,
                0b0000001100000000,
                0b0000011000000000,
                0b0001110000000000,
            ],
            6,
            0,
        ),
        '.' => (&[0b0011000000000000, 0b0011000000000000], 2, 0),
        ',' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0001000000000000,
                0b0010000000000000,
            ],
            2,
            2,
        ),
        ':' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            2,
            0,
        ),
        ';' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0000000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0001000000000000,
                0b0010000000000000,
            ],
            2,
            2,
        ),
        '!' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0000000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            2,
            0,
        ),
        '?' => (
            &[
                0b0001111000000000,
                0b0010001100000000,
                0b0000001100000000,
                0b0000011000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000000000000000,
                0b0000110000000000,
                0b0000110000000000,
            ],
            6,
            0,
        ),
        '\'' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0001000000000000,
                0b0010000000000000,
            ],
            2,
            -5,
        ),
        '_' => (&[0b0011111000000000], 5, 0),
        '+' => (
            &[
                0b0000100000000000,
                0b0000100000000000,
                0b0011111000000000,
                0b0000100000000000,
                0b0000100000000000,
            ],
            5,
            -1,
        ),
        '-' => (&[0b0011111000000000], 5, -3),
        '/' => (
            &[
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
            ],
            4,
            0,
        ),
        '\\' => (
            &[
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
            ],
            4,
            0,
        ),
        '(' => (
            &[
                0b0001100000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0001100000000000,
            ],
            3,
            0,
        ),
        ')' => (
            &[
                0b0011000000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0011000000000000,
            ],
            3,
            0,
        ),
        '<' => (
            &[
                0b0000110000000000,
                0b0001100000000000,
                0b0011000000000000,
                0b0001100000000000,
                0b0000110000000000,
            ],
            4,
            -1,
        ),
        '>' => (
            &[
                0b0011000000000000,
                0b0001100000000000,
                0b0000110000000000,
                0b0001100000000000,
                0b0011000000000000,
            ],
            4,
            -1,
        ),
        '[' => (
            &[
                0b0011110000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011000000000000,
                0b0011110000000000,
            ],
            4,
            0,
        ),
        ']' => (
            &[
                0b0011110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0000110000000000,
                0b0011110000000000,
            ],
            4,
            0,
        ),
        '{' => (
            &[
                0b0000110000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0011000000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0000110000000000,
            ],
            4,
            0,
        ),
        '}' => (
            &[
                0b0011000000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0000110000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0001100000000000,
                0b0011000000000000,
            ],
            4,
            0,
        ),
        '°' => (
            &[0b0011100000000000, 0b0010100000000000, 0b0011100000000000],
            3,
            -6,
        ),
        'Ä' => (
            &[
                0b0001001000000000,
                0b0000000000000000,
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011111100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
            ],
            6,
            0,
        ),
        'Ö' => (
            &[
                0b0001001000000000,
                0b0000000000000000,
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'Ü' => (
            &[
                0b0001001000000000,
                0b0000000000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'ä' => (
            &[
                0b0001001000000000,
                0b0000000000000000,
                0b0001111000000000,
                0b0000001100000000,
                0b0001111100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111100000000,
            ],
            6,
            0,
        ),
        'ö' => (
            &[
                0b0001001000000000,
                0b0000000000000000,
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'ü' => (
            &[
                0b0001001000000000,
                0b0000000000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0001111000000000,
            ],
            6,
            0,
        ),
        'ß' => (
            &[
                0b0001111000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011011000000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011001100000000,
                0b0011011000000000,
            ],
            6,
            0,
        ),
        '$' => (
            &[
                0b0000011000000000,
                0b0001111110000000,
                0b0011011001000000,
                0b0011011000000000,
                0b0011011000000000,
                0b0001111110000000,
                0b0000011011000000,
                0b0000011011000000,
                0b0010011011000000,
                0b0001111110000000,
                0b0000011000000000,
            ],
            8,
            1,
        ),
        '€' => (
            &[
                0b0000111100000000,
                0b0001100010000000,
                0b0001100000000000,
                0b0011111000000000,
                0b0001100000000000,
                0b0011111000000000,
                0b0001100000000000,
                0b0001100010000000,
                0b0000111100000000,
            ],
            7,
            0,
        ),
        ' ' => (&[], 2, 0),
        _ => (&[], -2, 0),
    }
}
