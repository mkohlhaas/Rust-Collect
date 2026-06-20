fn main() {
  let vec: Vec<i32> = vec![1, 2, 3] // create a vector
    .into_iter() // turn it into an iterator
    .map(|i| i + 1) // add 1 to each element
    .collect(); // collect the result

  println!("{vec:?}"); // prints `[2, 3, 4]`
}
