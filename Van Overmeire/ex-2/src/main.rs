use std::collections::HashMap;

fn main() {
  let map: HashMap<String, i32> = vec![1, 2, 3] // create a vector (but specify a HashMap type for the output)
    .into_iter() // turn it into an iterator
    .map(|i| i + 1) // add 1 to each element
    .map(|i| (i.to_string(), i)) // map to a tuple ('key' and 'value')
    .collect(); // collect the result

  println!("{map:?}"); // prints `{"2": 2, "3": 3, "4": 4}` or similar
}
