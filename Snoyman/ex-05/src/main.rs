// without type annotations

fn main() {
  let myvec = vec![1, 2, 3, 4, 5];

  let new_vec = myvec.into_iter().map(|x| x * 2).collect(); // ⚠️

  println!("{:?}", new_vec);
}
