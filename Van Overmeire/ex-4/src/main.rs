// introducing Result

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
  let first = call_that_might_go_wrong(1);
  println!("{:?}", first); // prints `Ok(2)`

  let second = call_that_might_go_wrong(2);
  println!("{:?}", second); // prints `Err("Did not expect 2")`
}
