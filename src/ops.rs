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
