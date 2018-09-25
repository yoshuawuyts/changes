extern crate changes;

#[test]
fn max_len() {
  let old: Vec<usize> = vec![0, 1, 2, 3];
  let new: Vec<usize> = vec![0, 1, 2];

  let diff = changes::shortest(&old, &new);
  assert_eq!(diff, 1);
}
