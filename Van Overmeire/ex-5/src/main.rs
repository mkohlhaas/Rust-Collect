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
  let vec: Vec<Result<i32, String>> = vec![1, 2, 3] // we expect a vector of results
    .into_iter()
    .map(call_that_might_go_wrong) // try to add 1 to each element
    .collect();

  println!("{:?}", vec); // prints `[Ok(2), Err("Did not expect 2"), Ok(4)]`
}
