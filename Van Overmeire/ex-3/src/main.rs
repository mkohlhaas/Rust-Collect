fn main() {
  // Exactly the same code, but now we are telling Rust that
  // we want a vector with tuples of type (String, i32).
  let map: Vec<(String, i32)> = vec![1, 2, 3] // now we specify a Vec type for our variable
    .into_iter()
    .map(|i| i + 1)
    .map(|i| (i.to_string(), i))
    .collect();

  println!("{map:?}");
}
