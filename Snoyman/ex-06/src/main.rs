// In Haskell, Purescript … world, Functor's map is always "shape preserving." When you map over a
// list in Haskell/Purescript, the result will always be a list.

// That's not the case with the IntoIterator/FromIterator combination. IntoIterator DESTROYS the
// original data structure, fully consuming it and producing an Iterator. Similarly, FromIterator
// PRODUCES a brand new data structure out of thin air, without any reference to the original data
// structure. Therefore, an explicit type annotation saying what the output type should be is
// necessary.

fn main() {
  let myvec = vec![1, 2, 3, 4, 5];

  let new_vec = myvec.into_iter().map(|x| x * 2).collect::<Vec<_>>(); // with turbofish

  println!("{:?}", new_vec);
}
