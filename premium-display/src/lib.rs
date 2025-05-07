//! Device drivers for monochrome displays

#![no_std]
#![warn(missing_docs)]

mod ssh1106;

pub use ssh1106::Ssh1106;
