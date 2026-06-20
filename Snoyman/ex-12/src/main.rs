// FromIterator trait
// https://doc.rust-lang.org/std/iter/trait.FromIterator.html

fn main() {
  {
    let five_fives = std::iter::repeat(5).take(5);
    let v = Vec::from_iter(five_fives);

    assert_eq!(v, vec![5, 5, 5, 5, 5]);
  }

  // using Iterator::collect() to implicitly use FromIterator
  {
    let five_fives = std::iter::repeat(5).take(5);
    let v: Vec<i32> = five_fives.collect();

    assert_eq!(v, vec![5, 5, 5, 5, 5]);
  }
}
