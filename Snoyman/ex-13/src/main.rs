// FromIterator for HashMap

use std::collections::HashMap;

fn main() {
  let people = vec![("Alice", 30), ("Bob", 35), ("Charlies", 25)]
    .into_iter()
    .collect::<HashMap<_, _>>();

  println!("{people:?}");
  println!("Alice is: {:?}", people.get("Alice"));
}
