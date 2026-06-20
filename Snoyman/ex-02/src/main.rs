// Double every value in the Vec.

fn main() {
  let mut myvec: Vec<i32> = vec![1, 2, 3, 4, 5];

  for num in &mut myvec {
    *num *= 2;
  }

  println!("{:?}", myvec);
}
