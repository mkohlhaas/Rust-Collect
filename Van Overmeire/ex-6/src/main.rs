// the function returns a Result of i32, or a String error
fn call_that_might_go_wrong(input: i32) -> Result<i32, String> {
  if input == 2 {
    Err("Did not expect 2".to_string()) // error case - wrap our error (message) in `Err`
  } else {
    let new_value = input + 1; // increment
    Ok(new_value) // return the incremented value in an `Ok`
  }
}

fn main() {
  // NOTE: swap commented lines to see a difference
  // let vec: Result<Vec<i32>, String> = vec![1, 3, 4, 5, 6, 7] // a result containing a vector of values, or a single error
  let vec: Result<Vec<i32>, String> = vec![1, 2, 3, 4, 5, 6, 7] // a result containing a vector of values, or a single error
    .into_iter()
    .map(call_that_might_go_wrong)
    .collect();

  println!("{:?}", vec); // prints `Err("Did not expect 2")`
}
