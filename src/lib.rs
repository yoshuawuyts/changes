#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]

//! ## Example
//! ```rust
//! ```

/// Represents a line in a file.
#[derive(Debug, Clone)]
pub struct Line {
  /// The index in the file the line was in.
  index: u8,
  /// The content of the line.
  data: String,
}

/// The Myers Diffing algorithm
#[derive(Debug, Clone)]
pub struct Myers {
  a: Vec<Line>,
  b: Vec<Line>,
}

impl Myers {
  /// Create a new instance.
  pub fn new(a: Vec<Line>, b: Vec<Line>) -> Self{
    Self {a, b}
  }

  /// Perform the diff.
  pub fn diff(&mut self) {
    self.shortest_edit();
    unimplemented!();
  }

  /// Compute the shortest edit path between two vectors.
  fn shortest_edit(&mut self) {
    let n = self.a.len();
    let m = self.b.len();

    // TODO: when we negatively index into an array we should read from the end.
    // (this means we shouldn't index using negative values, but instead double
    // the values and do better math.)
    let max = n + m;
    let mut v: Vec<Option<u8>> = vec![None; max * 2];
    v[0] = Some(0);
    unimplemented!();
  }
}
