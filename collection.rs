#[derive(Debug)]
struct Collections {
  values: Vec<usize>
}

impl Collections {
  fn new() -> Self {
    Collections { values: Vec::new() }
  }

  fn with_values(values: Vec<usize>) -> Self {
    Collections { values }
  }

  fn add(&mut self, value: usize) {
    if self.has(&value) == false {
      self.values.push(value);
    }
  }

  fn has(&self, value: &usize) -> bool {
    self.values.contains(value)
  }

  fn remove(&mut self, value: usize) {
    match self.values.iter().position(| val | val == &value) {
      None => panic!("element {} not found", value),
      Some(index) => self.values.remove(index)
    };
  }

  fn union(&mut self, collection: Self) -> Self {
    for item in collection.values.iter() {
      if self.has(&item) == false {
        self.values.push(*item);
      }
    }

    Collections::with_values(self.values.clone())
  }

  fn intersection(&mut self, collection: Self) -> Self {
    let mut new_values = Vec::new();
    for item in collection.values.iter() {
      if self.has(&item) == true {
        new_values.push(*item);
      }
    }

    Collections::with_values(new_values)
  }

  fn difference(&mut self, collection: Self) -> Vec<usize> {
    let mut new_values = Vec::new();
    for item in collection.values.iter() {
      if self.has(&item) == true {
        new_values.push(*item);
      }
    }

    new_values
  }

  fn included(&self, collection: Self) -> bool {
    for item in collection.values.iter() {
      if self.has(&item) == false {
        return false
      }
    }

    true
  }
}

fn main() {
  let mut first_coll = Collections::new();
  let mut second_coll = Collections::new();

  first_coll.add(1);
  first_coll.add(2);
  second_coll.add(3);
  second_coll.add(4);
  second_coll.add(1);

  let mut thrid_call = first_coll.union(second_coll);
  dbg!(thrid_call);
}