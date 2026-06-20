// In order to use map on a Vec, we have to:
// 1. Convert the Vec into an Iterator
// 2. Perform the map on the Iterator
// 3. Convert the Iterator back into a Vec

fn main() {
  let myvec: Vec<i32> = vec![1, 2, 3, 4, 5];

  let new_vec: Vec<i32> = myvec
    .into_iter() // 1.
    .map(|x| x * 2) // 2.
    .collect(); // 3.

  println!("{:?}", new_vec);
}
