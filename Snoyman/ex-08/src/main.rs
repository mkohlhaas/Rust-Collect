// Result type

fn main() {
  let myvec = vec![1, 2, 3, 4, 5, 6];

  let new_vec: Vec<Result<i32, &str>> = myvec
    .into_iter()
    .map(|n| {
      if n > 5 {
        Err("Not allowed to double big numbers")
      } else {
        Ok(n * 2)
      }
    })
    .collect();

  println!("{:?}", new_vec);
}
