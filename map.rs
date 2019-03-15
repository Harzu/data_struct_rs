use map::*;

mod map {
  #[derive(Debug, Clone)]
  pub struct Map<T> {
    values: Vec<(String, T)>
  }

  pub trait IMap<T> {
    fn new() -> Self;
    fn set(&mut self, key: &str, value: T);
    fn get(&self, key: &str) -> T;
    fn has(&self, key: &str) -> bool;
    fn delete(&mut self, key: &str);
    fn clear(&mut self);
  }

  impl<T> IMap<T> for Map<T>
    where T: Clone
  {
    fn new() -> Self {
      Map {
        values: Vec::new()
      }
    }

    fn set(&mut self, key: &str, value: T) {
      if self.has(key) == false {
        let new_value: (String, T) = (key.to_string(), value);
        self.values.push(new_value);
      }
    }

    fn get(&self, key: &str) -> T {
      match self.values.iter().find(|value| value.0 == key.to_string()) {
        Some(found_value) => found_value.1.clone(),
        None => panic!("not value with key: {}", key)
      }
    }

    fn has(&self, key: &str) -> bool {
      match self.values.iter().find(|ref value | value.0 == key.to_string()) {
          Some(_) => true,
          None => false
      }
    }

    fn delete(&mut self, key: &str) {
      if let Some(index) = self.values.iter().position(|value| value.0 == key.to_string()) {
        self.values.remove(index);
      }
    }

    fn clear(&mut self) {
      self.values = Vec::new();
    }
  }
}

fn main() {
  let mut mp = Map::new();
  mp.set("My Hero", "Batman");
  mp.set("My anti hero", "Joker");
  let hero = mp.get("My Hero");
  dbg!(hero);
  mp.delete("My anti hero");
  let _ = mp.get("My anti hero");
  mp.clear();
  let _ = mp.get("My Hero");
}