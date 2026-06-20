// type inference to the rescue

use std::collections::HashMap;

fn main() {
  let map: HashMap<_, _> = vec![1, 2, 3] // Rust can infer the types
    .into_iter()
    .map(|i| i + 1)
    .map(|i| (i.to_string(), i))
    .collect();
  println!("{:?}", map);
}
