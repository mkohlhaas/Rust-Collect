// The power of the FromIterator!

fn main() {
  // NOTE: swap commented lines to see a difference
  let myvec = vec![1, 2, 3, 4, 5];
  // let myvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  // from: Result<Vec<i32>, &str>
  // to:   Vec<Result<i32>, &str>
  let new_vec: Result<Vec<i32>, &str> = myvec // we simply change the type to what we want!
    .into_iter()
    .map(|x| {
      if x > 5 {
        Err("Not allowed to double big numbers.")
      } else {
        Ok(x * 2)
      }
    })
    .collect();

  match new_vec {
    Ok(vec) => println!("{:?}", vec),
    Err(err) => println!("{}", err),
  }
}
