use trie::*;

mod trie {
  use std::collections::HashMap;

  #[derive(Debug)]
  struct Node {
    keys: HashMap<String, Box<Node>>,
    end: bool
  }

  #[derive(Debug)]
  pub struct Trie {
    root: Node,
    length: usize
  }

  pub trait ITrie {
    fn new() -> Self;
    fn insert(&mut self, input: &str);
    fn remove(&mut self, key: &str);
  }

  impl Node {
    fn new() -> Self {
      Node {
        keys: HashMap::new(),
        end: false
      }
    }

    fn set_end(&mut self) {
      self.end = !self.end;
    }

    fn has_end(&self) -> bool {
      self.end
    }
  }

  impl ITrie for Trie {
    fn new() -> Self {
      Trie {
        root: Node::new(),
        length: 0
      }
    }

    fn insert(&mut self, input: &str) {
      let last_node = input.chars().fold(&mut self.root, |current_node, c| {
        current_node.keys
          .entry(c.to_string())
          .or_insert(Box::new(Node::new()))
      });

      last_node.set_end();
      self.length += 1;
    }

    fn remove(&mut self, key: &str) {
      let last_node = key.chars().fold(&mut self.root, |current_node, c| {
        current_node.keys.get_mut(&c.to_string()).unwrap()
      });

      if last_node.has_end() == true {
        last_node.set_end();
      }
    }
  }
}


fn main() {
  let mut trie = Trie::new();
  trie.insert("lol");
  trie.insert("loh");
  trie.insert("moh");
  trie.remove("moh");
}