#![no_std]
#![warn(missing_docs)]

//! `Line` -- A fixed capacity `String`
//!
//! Since a `Line` has a fixed capacity it can be allocated on the stack or in static memory.

use core::{borrow, cmp, error, fmt, hash, mem::MaybeUninit, ops, ptr, slice, str::FromStr};

/// A `Line` is like a `String` with a fixed capacity
///
/// The capacity is not allowed to be larger than 255.
pub struct Line<const CAPACITY: usize> {
    content: [MaybeUninit<u8>; CAPACITY],
    len: u8,
}

/// A `Line` with a capacity of 15 bytes
pub type Line15 = Line<15>;
/// A `Line` with a capacity of 31 bytes
pub type Line31 = Line<31>;
/// A `Line` with a capacity of 63 bytes
pub type Line63 = Line<63>;
/// A `Line` with a capacity of 127 bytes
pub type Line127 = Line<127>;
/// A `Line` with a capacity of 255 bytes
pub type Line255 = Line<255>;

/// Error type for mutating `Line`s
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The `Line` does not have enough capacity
    NoSpace,
}

impl<const N: usize> Line<N> {
    /// Create a new Line.
    pub const fn new() -> Self {
        assert!(N < 256);
        Self {
            content: [MaybeUninit::uninit(); N],
            len: 0,
        }
    }
    /// Clear the line and set its length to 0.
    pub fn clear(&mut self) {
        self.len = 0;
    }
    /// Returns the `Line`'s capacity in bytes.
    pub const fn capacity(&self) -> usize {
        N
    }
    /// Extracts a string slice containing the entire `Line`.
    pub fn as_str(&self) -> &str {
        // Safety: We know the buffer is valid utf-8 up to len.
        unsafe {
            core::str::from_utf8_unchecked(slice::from_raw_parts(
                self.content.as_ptr().cast(),
                self.len as usize,
            ))
        }
    }
    /// Converts a `Line` into a mutable string slice.
    pub fn as_mut_str(&mut self) -> &mut str {
        // Safety: We know the buffer is valid utf-8 up to len.
        unsafe {
            core::str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(
                self.content.as_mut_ptr().cast(),
                self.len as usize,
            ))
        }
    }
    /// Append the text to the `Line`.
    pub fn push_str(&mut self, s: &str) -> Result<(), Error> {
        assert!(N < 256);
        let l = self.len as usize;
        if l + s.len() <= N {
            // Safety: We know there is enough space and the memory does not overlap.
            unsafe {
                ptr::copy_nonoverlapping(
                    s.as_ptr(),
                    self.content[l..l + s.len()].as_mut_ptr().cast(),
                    s.len(),
                );
            }
            self.len += s.len() as u8;
            Ok(())
        } else {
            Err(Error::NoSpace)
        }
    }
}
impl<const N: usize> Clone for Line<N> {
    fn clone(&self) -> Self {
        let mut content = [MaybeUninit::uninit(); N];
        // Safety: self.content is properly initialized to self.len.
        unsafe {
            ptr::copy_nonoverlapping(
                self.content.as_ptr(),
                content.as_mut_ptr(),
                self.len as usize,
            );
        }
        Self {
            content,
            len: self.len,
        }
    }
}
impl<const N: usize> FromStr for Line<N> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        let mut line = Self::new();
        line.push_str(s)?;
        Ok(line)
    }
}
impl<const N: usize> ops::Deref for Line<N> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
impl<const N: usize> AsRef<str> for Line<N> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<const N: usize> AsMut<str> for Line<N> {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl<const N: usize> borrow::Borrow<str> for Line<N> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl<const N: usize> borrow::BorrowMut<str> for Line<N> {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
impl<const N: usize> TryFrom<&str> for Line<N> {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
impl<const N: usize> fmt::Write for Line<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s).map_err(|_| fmt::Error)
    }
}
impl<const N: usize> fmt::Debug for Line<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
impl<const N: usize> fmt::Display for Line<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
impl<const N: usize> Default for Line<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize, const M: usize> PartialEq<Line<M>> for Line<N> {
    fn eq(&self, other: &Line<M>) -> bool {
        self.as_str() == other.as_str()
    }
}
impl<const N: usize> Eq for Line<N> {}
impl<const N: usize> PartialEq<str> for Line<N> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl<const N: usize> PartialEq<Line<N>> for str {
    fn eq(&self, other: &Line<N>) -> bool {
        self == other.as_str()
    }
}
impl<'a, const N: usize> PartialEq<&'a str> for Line<N> {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}
impl<const N: usize> PartialEq<Line<N>> for &str {
    fn eq(&self, other: &Line<N>) -> bool {
        *self == other.as_str()
    }
}
impl<const N: usize, const M: usize> PartialOrd<Line<M>> for Line<N> {
    fn partial_cmp(&self, other: &Line<M>) -> Option<cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}
impl<const N: usize> Ord for Line<N> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}
impl<const N: usize> PartialOrd<str> for Line<N> {
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}
impl<const N: usize> PartialOrd<Line<N>> for str {
    fn partial_cmp(&self, other: &Line<N>) -> Option<cmp::Ordering> {
        Some(self.cmp(other.as_str()))
    }
}
impl<'a, const N: usize> PartialOrd<&'a str> for Line<N> {
    fn partial_cmp(&self, other: &&'a str) -> Option<cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}
impl<const N: usize> PartialOrd<Line<N>> for &str {
    fn partial_cmp(&self, other: &Line<N>) -> Option<cmp::Ordering> {
        Some(self.cmp(&other.as_str()))
    }
}
impl<const N: usize> hash::Hash for Line<N> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSpace => f.write_str("not enough capacity"),
        }
    }
}
impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use core::fmt::Write;

    use super::*;

    #[test]
    fn basics() {
        let mut line = Line15::new();
        write!(line, "{}", 1.5).unwrap();
        assert_eq!("1.5", line);
        assert!("1.6" > line);
        let mut line2 = Line255::new();
        write!(line2, "{line:?}").unwrap();
        assert_eq!(line2, "\"1.5\"");
        assert_eq!(line.clone(), line);
        let line3 = Line31::from_str(&line2).unwrap();
        assert_eq!(line2, line3);
        assert!(Line15::from_str("b").unwrap() > Line63::from_str("a").unwrap());
        assert_eq!(Line15::from_str("a").unwrap().as_bytes(), &[b'a']);
    }

    #[test]
    fn maps() {
        extern crate std;
        use std::collections::{BTreeMap, HashMap};
        let mut m = HashMap::new();
        m.insert(Line15::from_str("test").unwrap(), 1234);
        m.insert("elite".try_into().unwrap(), 1337);
        assert_eq!(m.get("test"), Some(&1234));
        let mut m = BTreeMap::new();
        m.insert(Line15::from_str("test").unwrap(), 1234);
        assert_eq!(m.get("test"), Some(&1234));
    }

    #[test]
    #[should_panic]
    fn invalid_capacity() {
        let _: Line<256> = Line::new();
    }
}
