// Rust is not a pure language!

fn main() {
  let myvec = vec![1, 2, 3, 4, 5];

  let new_vec = myvec
    .into_iter() // consuming iterator (destroy collection)
    .map(|x| {
      println!("About to double {}", x); // this wouldn't be possible in pure FP languages (Haskell, Purescript …)
      x * 2
    })
    .collect::<Vec<_>>(); // create collection

  println!("{:?}", new_vec);
}

// In Purescript you would need Data.Traversable.
// https://pursuit.purescript.org/packages/purescript-foldable-traversable/6.0.0/docs/Data.Traversable

// traverse :: forall a b m. Applicative m => (a -> m b) -> t a -> m (t b)
// sequence :: forall a m.   Applicative m => t (m a) -> m (t a)
