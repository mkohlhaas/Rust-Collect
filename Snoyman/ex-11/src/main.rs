// We're still dealing with two different traits (IntoIterator and FromIterator), and there's
// nothing preventing these from being different types. Therefore, some kind of type annotation is
// still necessary.

fn main() {
  // NOTE: swap commented lines to see a difference
  let myvec = vec![1, 2, 3, 4, 5];
  // let myvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  let new_vec = myvec
    .into_iter()
    .map(|x| {
      if x > 5 {
        Err("Not allowed to double big numbers.")
      } else {
        Ok(x * 2)
      }
    })
    .collect::<Result<Vec<_>, _>>(); // this time type annotation with the turbofish

  match new_vec {
    Ok(new_vec) => println!("{:?}", new_vec),
    Err(e) => println!("{}", e),
  }
}
