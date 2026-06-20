fn main() {
  // ⚠️ does not compile, and the error advises you to specify the type
  let vec = vec![1, 2, 3].into_iter().map(|i| i + 1).collect();
}
