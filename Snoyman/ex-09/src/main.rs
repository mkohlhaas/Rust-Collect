// We have a   Vec<Result<i32, &str>>.
// But we want Result<Vec<i32>, &str>

fn main() {
  // NOTE: swap commented lines to see a difference
  let myvec = vec![1, 2, 3, 4, 5];
  // let myvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  let mut new_vec = Vec::new();

  // using a for loop
  for n in myvec {
    if n > 5 {
      println!("Not allowed to double big numbers!");
      return;
    } else {
      new_vec.push(n);
    }
  }

  println!("{:?}", new_vec);
}
