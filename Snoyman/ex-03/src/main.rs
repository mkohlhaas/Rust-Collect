// `map` on a Vec.

fn main() {
  let myvec: Vec<i32> = vec![1, 2, 3, 4, 5];

  let new_vec: Vec<i32> = myvec.map(|x| x * 2); // ⚠️ doesn't work

  println!("{:?}", new_vec);
}
