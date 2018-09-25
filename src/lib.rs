#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

#[macro_use]
extern crate failure;

mod error;

pub use error::{Error, ErrorKind, Result};

use std::cmp;

/// Representation of two differences between two states.
#[derive(Debug, Clone)]
pub enum Ops {
  /// All instructions have been executed, ignore the rest of the array.
  Done,
  /// The element from array A at the first index is equivalent to the element
  /// from array B at the next index.
  Noop(u32),
  /// Move the element from array A at the first index into array B *before* the
  /// second index.
  Move((u32, u32)),
  /// Delete the element from array B at `index`.
  Delete(u32),
  /// Replace the element from array B at the second index with the element from
  /// array A at the first index.
  Replace((u32, u32)),
}

/// Compute the shortest change possible.
#[inline]
pub fn shortest<T: cmp::PartialEq>(old: &[T], new: &[T]) -> u64 {
  let old_len = old.len();
  let new_len = new.len();
  let max_len = old_len + new_len;

  let mut buf = vec![0; max_len * 2];
  let max_len = max_len as i64;

  for d in 0..=max_len {
    let lower = -(d - 2 * cmp::max(0, d - new_len as i64));
    let upper = d - 2 * cmp::max(0, d - old_len as i64);

    for k in (lower..=upper).step_by(2) {
      let index = k as usize;
      let mut x = if k == -d || (k != d && buf[index - 1] < buf[index + 1]) {
        buf[index + 1]
      } else {
        buf[index - 1] + 1
      };

      let mut y = x - k;
      let x_index = x as usize;
      let y_index = y as usize;
      while x < old_len as i64
        && y < new_len as i64
        && old[x_index] == new[y_index]
      {
        x += 1;
        y += 1;
      }
      buf[index] = x;

      if x == old_len as i64 && y == new_len as i64 {
        return d as u64;
      }
    }
  }
  unreachable!();
}
