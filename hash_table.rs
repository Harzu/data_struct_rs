use hash_table::*;

mod hash_table {
  use std::fmt::Debug;
  use std::hash::Hasher;
  use std::collections::LinkedList;
  use std::collections::hash_map::DefaultHasher;

  #[derive(Debug, Clone)]
  struct Node<T> {
    key: String,
    value: T
  }

  #[derive(Debug)]
  pub struct HashTable<T> {
    values: Vec<LinkedList<Node<T>>>
  }

  pub trait IHashTable<T> {
    fn new() -> Self;
    fn insert(&mut self, key: &str, value: T);
    fn contains(&self, key: &str) -> bool;
    fn get(&self, key: &str) -> Option<T>;
    fn deleate(&mut self, key: &str);
  }

  // Hash func
  fn generate_index(key: &str, length: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    hasher.write(key.as_bytes());
    (hasher.finish() as usize % (length + 10))
  }

  impl<T> IHashTable<T> for HashTable<T>
    where T: Clone + Default + Debug
  {
    fn new() -> Self {
      HashTable {
        values: Vec::new()
      }
    }

    fn insert(&mut self, key: &str, value: T) {
      let index = generate_index(key, self.values.len());
      let node = Node { key: key.to_string(), value };
      
      if self.values.len() < index {
        self.values.resize(index + 1, LinkedList::new());
      }

      self.values[index].push_back(node);
    }

    fn contains(&self, key: &str) -> bool {
      let index = generate_index(key, self.values.len());
      if index > self.values.len() - 1 {
        return false
      }

      for list in self.values[index].iter() {
        if list.key == key {
          return true
        }
      }

      false
    }

    fn get(&self, key: &str) -> Option<T> {
      let index = generate_index(key, self.values.len());
      if index > self.values.len() - 1 {
        return None
      }

      for list in self.values[index].iter() {
        if list.key == key {
          return Some(list.value.clone())
        }
      }

      None
    }

    fn deleate(&mut self, key: &str) {
      let index = generate_index(key, self.values.len());
      if index <= self.values.len() - 1 {
        let new_list = self.values[index]
          .clone()
          .into_iter()
          .filter(|ref node| node.key != key)
          .collect::<LinkedList<Node<T>>>();

        self.values[index] = new_list;
      }
    }
  }
}

fn main() {
  let mut hash_m = HashTable::new();
  hash_m.insert("he", 1);
  let get = hash_m.get("he");
  dbg!(get);
  hash_m.deleate("he");
  let get = hash_m.get("he");
  dbg!(get);
}
