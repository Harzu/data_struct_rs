use binary_search_tree::*;

mod binary_search_tree {
  use std::fmt::Debug;
  use std::cell::RefCell;

  #[derive(Debug, Clone)]
  pub struct BST<T> {
    value: T,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>
  }

  pub trait IBinarySearchTree<T> {
    fn new(value: T) -> Self;
    fn min(&mut self) -> &mut Self;
    fn max(&mut self) -> &mut Self;
    fn add(&mut self, value: T);
    fn remove(&mut self, value: T);
    fn find(&self, value: T) -> bool;
  }

  impl<T> BST<T>
    where T: PartialOrd + Clone + Debug
  {
    fn get_value(&self) -> T {
      self.value.clone()
    }

    fn child_count(&self) -> usize {
      let mut count = 0;
      if self.right.is_some() == true { count += 1; }
      if self.left.is_some() == true { count += 1; }

      count
    }

    fn search_node(&mut self, value: &T) -> Option<Box<&mut BST<T>>> {
      if value < &self.value {
        if let Some(ref mut left) = self.left {
          return left.search_node(value);
        }
      } else if value > &self.value {
        if let Some(ref mut right) = self.right {
          return right.search_node(value);
        }
      } else {
        return Some(Box::new(self))
      }

      None
    }

    fn clear(&mut self, value: T) {
      if value < self.value {
        if let Some(ref mut left) = self.left {
          if left.value != value {
            return left.clear(value);
          }
        }
        
        self.left = None;
      } else if value > self.value {
        if let Some(ref mut right) = self.right {
          if right.value != value {
            return right.clear(value);
          }
        }

        self.right = None;
      }
    }

    fn get_child_or_null(&self) -> Option<BST<T>> {
      if let Some(ref left) = self.left {
        return Some(*left.clone())
      };

      if let Some(ref right) = self.right {
        return Some(*right.clone())
      };

      None
    }

    fn remove_with_all_child(&mut self) {
      let left_node = &mut self.left;
      if let Some(ref mut left) = left_node {
        let max_node = left.max();
        self.value = max_node.get_value();
        max_node.replace_node();
      }
    }

    fn replace_node(&mut self) -> bool {
      let get_child = self.get_child_or_null();
      if let Some(child_node) = get_child {
        self.value = child_node.value;
        self.left = child_node.left;
        self.right = child_node.right;
        return true
      }

      false
    }
  }

  impl<T> IBinarySearchTree<T> for BST<T>
    where T: PartialOrd + Clone + Debug
  {
    fn new(value: T) -> Self {
      BST {
        value,
        left: None,
        right: None
      }
    }

    fn add(&mut self, value: T) {
      if value < self.value {
        match self.left {
          None => { self.left = Some(Box::new(BST::new(value))) },
          Some(ref mut target) => { target.add(value) } 
        }
      } else if value > self.value {
        match self.right {
          None => { self.right = Some(Box::new(BST::new(value))) },
          Some(ref mut target) => { target.add(value) } 
        }          
      }
    }

    fn find(&self, value: T) -> bool {
      if value < self.value {
        if let Some(ref left) = self.left {
          return left.find(value);
        }
      } else if value > self.value {
        if let Some(ref right) = self.right {
          return right.find(value);
        }
      } else {
        return true
      }

      false
    }

    fn max(&mut self) -> &mut Self {
      match self.right {
        None => self,
        Some(ref mut right) => right.max()
      }
    }

    fn min(&mut self) -> &mut Self {
      match self.left {
        None => self,
        Some(ref mut left) => left.max()
      }
    }

    fn remove(&mut self, value: T) {
      let mut child_count = 0;
      if let Some(rm_node) = self.search_node(&value) {
        child_count = rm_node.child_count();
        if child_count < 2 {
          rm_node.replace_node();
        } else {
          rm_node.remove_with_all_child();
        }
      }

      if child_count == 0 {
        self.clear(value);
      }
    }
  }
}

fn main() {
  let mut tree = BST::new(9);
  tree.add(3);
  tree.add(10);
  tree.add(2);
  tree.add(7);
  tree.add(1);
  tree.add(12);
  tree.add(6);
  tree.add(11);
  tree.add(14);
  tree.remove(9);
  tree.remove(1);
  tree.remove(14);
  tree.remove(12);

  println!("{:#?}", tree);
}