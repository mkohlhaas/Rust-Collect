// layering FromIterators

use std::collections::HashMap;

fn main() {
  let people = vec![
    Ok(("Alice", 30)),
    Ok(("Bob", 35)),
    Err("Uh-oh, this didn't work!"), // NOTE: comment out this line to see the difference
    Ok(("Charlies", 25)),
  ]
  .into_iter()
  .collect::<Result<HashMap<_, _>, &str>>(); // `Result` on top of `HashMap`

  println!("{people:?}");

  match people {
    Err(e) => println!("Error occurred: {}", e),
    Ok(people) => {
      println!("Alice is: {:?}", people.get("Alice"));
    }
  }
}
