use graph::*;

mod graph {
  use std::fmt::Debug;
  use std::cell::RefCell;

  #[derive(Debug, Clone)]
  struct Node<T> {
    id: usize,
    value: T,
    links: Vec<Box<Node<T>>>
  }

  #[derive(Debug, Clone)]
  pub struct Graph<T> {
    nodes: RefCell<Vec<Node<T>>>
  }

  pub trait IGraph<T> {
    fn new() -> Self;
    fn add_vortex(&mut self, value: T);
    fn add_edge(&mut self, first_id: usize, second_id: usize);
    fn remove_node(&mut self, id: usize);
    fn remove_edge(&mut self, first_id: usize, second_id: usize);
    fn generate_matrix(&self) -> Vec<Vec<usize>>;
  }

  impl<T> Graph<T> 
    where T: Clone + Debug + PartialEq
  {
    fn find_index(&self, id: usize, vector: Option<Vec<Box<Node<T>>>>) -> usize {
      if let Some(vectors) = vector {
        return match vectors.iter().position(|ref node| node.id == id) {
          Some(index) => index,
          None => panic!("node with id {} not found", id)
        }
      }

      let nodes_borrow = self.nodes.borrow();

      match nodes_borrow.iter().position(|ref node| node.id == id) {
        Some(index) => index,
        None => panic!("node with id {} not found", id)
      }
    }

    fn find_node(&self, id: usize) -> Node<T> {
      let nodes_borrow = self.nodes.borrow();

      match nodes_borrow.iter().find(|ref node| node.id == id) {
        Some(node) => node.clone(),
        None => panic!("node with id {} not found", id)
      }
    }
  }

  impl<T> IGraph<T> for Graph<T>
    where T: Clone + Debug + PartialEq
  {
    fn new() -> Self {
      Graph {
        nodes: RefCell::new(Vec::new())
      }
    }

    fn add_vortex(&mut self, value: T) {
      let mut nodes = self.nodes.borrow_mut();
      let length = nodes.len() + 1;
      let node = Node {
        id: length,
        value,
        links: Vec::new()
      };

      nodes.push(node);
    }

    fn add_edge(&mut self, first_id: usize, second_id: usize) {
      let seocnd_node = self.find_node(second_id);
      let first_index = self.find_index(first_id, None);
      self.nodes.borrow_mut()[first_index].links.push(Box::new(seocnd_node));
    }

    fn remove_node(&mut self, id: usize) {
      let get_removed_index = self.find_index(id, None);
      let mut nodes = self.nodes.borrow_mut();      
      nodes.remove(get_removed_index);
      
      for node in nodes.iter_mut() {
        if let Some(index) = node.links.iter().position(|ref mut node| node.id == id) {
          node.links.remove(index);
        }
      }
    }

    fn remove_edge(&mut self, first_id: usize, second_id: usize) {
      let first_index = self.find_index(first_id, None);
      let first_node = self.find_node(first_id);
      let seocnd_index = self.find_index(second_id, Some(first_node.links));
      self.nodes.borrow_mut()[first_index].links.remove(seocnd_index);
    }

    fn generate_matrix(&self) -> Vec<Vec<usize>> {
      let mut matrix = Vec::new();
      let borrow_node = self.nodes.borrow();

      for node in borrow_node.iter() {
        let linksCount = borrow_node.iter().map(|ref check_node| {
          for link in node.links.iter() {
            if link.id == check_node.id {
              return 1
            }
          }

          0
        }).collect();

        matrix.push(linksCount);
      }

      matrix
    }
  }
}

fn main() {
  let mut gr = Graph::new();
  gr.add_vortex("var");
  gr.add_vortex("let");
  gr.add_vortex("some");
  gr.add_vortex("none");
  gr.add_edge(1, 2);
  gr.add_edge(3, 4);
  let matrix = gr.generate_matrix();
  dbg!(matrix);
  gr.remove_edge(1, 2);
  gr.remove_node(2);
  println!("{:#?}", gr);
}