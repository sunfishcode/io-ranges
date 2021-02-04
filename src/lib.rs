//! Random-access I/O
//!
//! This crate defines [`ReadAt`], [`WriteAt`], and [`EditAt`] traits which
//! define interfaces to random-access or seekable devices, such as normal
//! files, block devices, disk partitions, and memory buffers.
//!
//! It also defines [`RangeReader`], [`RangeWriter`], and [`RangeEditor`] types
//! which implement the above traits and and can be constructed from any
//! file-like type.  On Posix-ish platforms, including limited support for
//! WASI, these types just contain a single file descriptor (and implement
//! [`AsRawFd`]), plus any resources needed to safely hold the file descriptor
//! live. On Windows, they contain a single file handle (and implement
//! [`AsRawHandle`]).

#![deny(missing_docs)]
#![cfg_attr(can_vector, feature(can_vector))]
#![cfg_attr(write_all_vectored, feature(write_all_vectored))]

mod borrow_streamer;
#[cfg(feature = "io-streams")]
mod own_streamer;
#[cfg(not(windows))]
mod posish;
mod ranges;
mod slice;
#[cfg(windows)]
mod windows;

/// Functions for implementing `ReadAt` and `WriteAt` for file-like types.
pub mod filelike {
    // We can't use Windows' `read_at` or `write_at` here because it isn't able to
    // extend the length of a file we can't `reopen` (such as temporary files).
    // However, while `FileIoExt` can't use `seek_write` because it mutates the
    // current position, here we *can* use plain `seek_write` because `RangeEditor`
    // doesn't expose the current position.
    #[cfg(not(windows))]
    pub use crate::posish::*;
    #[cfg(windows)]
    pub use crate::windows::*;
}

pub use ranges::{EditAt, Metadata, Range, RangeEditor, RangeReader, RangeWriter, ReadAt, WriteAt};

pub use system_interface::fs::Advice;
