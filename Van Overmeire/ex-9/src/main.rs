#![allow(dead_code, unused_variables)]

struct ContainsAVector {
  // a 'class' containing a vector of numbers
  content: Vec<i32>,
}

fn main() {
  let vec = vec![1, 2, 3] // NOTE: no type annotation necessary
    .into_iter()
    .map(|i| i + 1)
    .collect();

  let contains_struct = ContainsAVector {
    content: vec, // NOTE: Rust knows `vec` can only be a Vec<i32>
  };
}
