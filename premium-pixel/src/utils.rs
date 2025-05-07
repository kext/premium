use core::ops::{Deref, DerefMut};

use crate::Surface;

/// Rotate a surface
#[derive(Debug)]
pub struct Rotate<S>(S);
impl<S> Rotate<S> {
    /// Rotate by 90 degrees clockwise
    pub fn by90(surface: S) -> Rotate<S> {
        Rotate(surface)
    }
    /// Rotate by 180 degrees clockwise
    pub fn by180(surface: S) -> Rotate<Rotate<S>> {
        Rotate(Rotate(surface))
    }
    /// Rotate by 270 degrees clockwise
    pub fn by270(surface: S) -> Rotate<Rotate<Rotate<S>>> {
        Rotate(Rotate(Rotate(surface)))
    }
}
impl<S: Surface> Surface for Rotate<S> {
    fn clear(&mut self) {
        self.0.clear()
    }
    fn height(&self) -> i32 {
        self.0.width()
    }
    fn width(&self) -> i32 {
        self.0.height()
    }
    fn pixel(&mut self, x: i32, y: i32) {
        self.0.pixel(self.0.width() - 1 - y, x)
    }
}
impl<S> Deref for Rotate<S> {
    type Target = S;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<S> DerefMut for Rotate<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// This struct implements the Surface trait and can be used to measure the width of text without
/// drawing anything
pub struct Measure;
impl Surface for Measure {
    fn clear(&mut self) {}
    fn pixel(&mut self, _x: i32, _y: i32) {}
    fn height(&self) -> i32 {
        1000
    }
    fn width(&self) -> i32 {
        1000
    }
}
