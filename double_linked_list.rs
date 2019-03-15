use double_linked_list::*;

mod double_linked_list {
  use std::fmt::Debug;
  use std::cell::RefCell;

  #[derive(Debug, Clone)]
  struct Node<T> {
    value: T,
    prev: Option<usize>,
    next: Option<usize>
  }

  #[derive(Debug, Clone)]
  pub struct LinkedList<T> {
    nodes: RefCell<Vec<Node<T>>>,
    head: Option<usize>,
    tail: Option<usize>
  }

  pub trait ILinkedList<T> {
    fn new() -> Self;
    fn push_front(&mut self, value: T);
    fn push_back(&mut self, value: T);
    fn pop_back(&mut self);
    fn pop_front(&mut self);
    fn remove(&mut self, value: T) -> bool;
    fn contains(&mut self, value: T) -> bool;
    fn copy_to(&self, mut index: usize) -> Vec<T>;
    fn clear(&mut self);
    fn length(&self) -> usize;
  }

  trait INode<T> {
    fn new(value: T) -> Self;
    fn get_node_value(&self) -> T;
    fn get_prev(&self) -> Option<usize>;
    fn get_next(&self) -> Option<usize>;
    fn set_prev(&mut self, new_prev: Option<usize>);
    fn set_next(&mut self, new_next: Option<usize>);
  }

  impl<T> INode<T> for Node<T>
    where T: Clone + Debug 
  {
    fn new(value: T) -> Self {
      Node {
        value,
        prev: None,
        next: None
      }
    }

    fn get_node_value(&self) -> T {
      self.value.clone()
    }

    fn get_prev(&self) -> Option<usize> {
      self.prev
    }

    fn get_next(&self) -> Option<usize> {
      self.next
    }

    fn set_prev(&mut self, new_prev: Option<usize>) {
      self.prev = new_prev;
    }

    fn set_next(&mut self, new_next: Option<usize>) {
      self.next = new_next;
    }
  }

  impl<T> LinkedList<T>
    where T: Clone + Debug + PartialEq
  {
    fn update_indexes(&mut self, side: bool, mut index: usize) {
      let mut nodes = self.nodes.borrow_mut();
      
      loop {
        match nodes[index].get_prev() {
          None => {
            if side == true { nodes[index].set_prev(Some(0)) }
          },
          Some(prev_index) => {
            nodes[index].set_prev(Some(match side {
              true => prev_index + 1,
              false if prev_index > 0 => prev_index - 1,
              false => 0
            }));
          }
        };

        if let Some(next_index) = nodes[index].get_next() {          
          nodes[index].set_next(Some(match side {
            true => next_index + 1,
            false => next_index - 1,
          }));

          index = next_index;
        } else { break }
      }
    }
  }

  impl<T> ILinkedList<T> for LinkedList<T>
    where T: Clone + Debug + PartialEq
  {
    fn new() -> Self {
      LinkedList {
        nodes: RefCell::new(Vec::new()),
        head: None,
        tail: None
      }
    }

    fn push_front(&mut self, value: T) {
      let mut node = Node::new(value);
      if self.length() == 0 {
        self.nodes.borrow_mut().push(node);
        self.head = Some(0);
        self.tail = Some(0);
      } else {
        node.set_next(Some(1));
        self.update_indexes(true, 0);
        self.nodes.borrow_mut().insert(0, node);
        self.tail = Some(self.length() - 1);        
      }
    }

    fn pop_front(&mut self) {
      if self.length() != 0 {
        self.update_indexes(false, 0);
        let mut nodes = self.nodes.borrow_mut();
        nodes.remove(0);
        nodes[0].set_prev(None);
      }
    }

    fn push_back(&mut self, value: T) {
      let mut node = Node::new(value);
      let length = self.length();
      let mut nodes = self.nodes.borrow_mut();

      if length == 0 {
        self.head = Some(0);
        self.tail = Some(0);
      } else {
        // Get index at tail node
        let tail_index = &self.tail.unwrap();
        // set index at prev Node in new Node
        node.set_prev(Some(tail_index.clone()));
        // set index at new Node in tail node
        nodes[tail_index.clone()].set_next(Some(length.clone()));
        // // update index at tail node
        self.tail = Some(length)
      }

      nodes.push(node);
    }

    fn pop_back(&mut self) {
      if self.length() != 0 {
        let mut nodes = self.nodes.borrow_mut();
        nodes.pop();
        
        let new_length = nodes.len();
        nodes[new_length - 1].set_next(None);
        self.tail = Some(nodes.len() - 1);
      }
    }
    
    fn length(&self) -> usize {
      self.nodes.borrow().len()
    }

    fn contains(&mut self, value: T) -> bool {
      let nodes = self.nodes.borrow();
      let mut index: usize = 0;
      
      loop {
        if nodes[index].get_node_value() == value {
          return true
        }

        if let Some(next) = nodes[index].get_next() {
          index = next;
        } else { break }
      }

      false
    }

    fn remove(&mut self, value: T) -> bool {
      let mut index: usize = 0;
      
      loop {
        if self.nodes.borrow()[index].get_node_value() == value {
          if self.nodes.borrow()[index].get_prev().is_some() == false {
            self.pop_front();
          } else if self.nodes.borrow()[index].get_next().is_some() == false {
            self.pop_back();
          } else {
            let mut nodes = self.nodes.borrow_mut();
            let prev_index = nodes[index].get_prev().unwrap();
            nodes[index + 1].set_prev(Some(prev_index));
            nodes.remove(index);
          } 
          
          self.tail = Some(self.length() - 1);
          return true
        }

        if let Some(next) = self.nodes.borrow()[index].get_next() {
          index = next;
        } else { break }
      }

      false
    }

    fn clear(&mut self) {
      self.nodes = RefCell::new(Vec::new());
      self.tail = None;
      self.head = None;
    }

    fn copy_to(&self, mut index: usize) -> Vec<T> {
      let mut values = Vec::new();
      let nodes = self.nodes.borrow();
      
      loop {
        values.push(nodes[index].get_node_value());

        if let Some(next) = nodes[index].get_next() {
          index = next;
        } else { break }
      }

      values
    }
  }
}

fn main() {
  let mut list = LinkedList::new();
  list.push_back("one");
  list.push_back("two");
  list.push_front("andy");
  list.push_front("marta");
  list.push_front("marta");
  list.push_front("marta");
  list.remove("one");
  list.remove("marta");
  list.pop_back();
  list.pop_front();
  println!("{:#?}", list);
}