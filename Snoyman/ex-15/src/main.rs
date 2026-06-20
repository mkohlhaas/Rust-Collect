// Validation (collecting several errors)

use std::collections::HashMap;
use std::io::*;
use validation::Validation;

fn main() {
  let people = vec![
    Ok(("Alice", 30)),
    Err("Oops, an error!"),
    Ok(("Bob", 35)),
    Err("Uh-oh, this didn't work!"),
    Ok(("Charlies", 25)),
    Err("And neither did this!"),
  ]
  .into_iter()
  // NOTE: swap commented lines to see a difference
  .collect::<Validation<HashMap<_, _>, Vec<&str>>>(); // catches all errors into a vec
  // .collect::<Result<HashMap<_, _>, &str>>(); // catches the first error

  println!("{people:?}\n");

  // for the Validation thingy:
  // NOTE: comment out when using a Result
  match people.into_result() {
    Err(errs) => {
      println!("Errors:");
      // NOTE: The () impl of FromIterator collapses all unit items from an iterator into one.
      // https://doc.rust-lang.org/std/primitive.unit.html#impl-FromIterator%3C()%3E-for-()
      errs.into_iter().map(|x| println!("{}", x)).collect()
    }
    Ok(people) => {
      println!("Alice is: {:?}", people.get("Alice"));
    }
  }

  // with try_for_each(…) and writeln!(…) instead of map(…) and println!(…)
  let data = [1, 2, 3, 4, 5];
  let res: Result<()> = data.iter().try_for_each(|n| writeln!(stdout(), "{n}"));
  assert!(res.is_ok());
}
