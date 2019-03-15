#[derive(Debug)]
struct BHeap {
  list: Vec<usize>
}

impl BHeap {
  fn new() -> Self {
    BHeap { list: Vec::new() }
  }

  fn swap_up(&mut self, index: Option<usize>) {
    let mut i = self.list.len() - 1;
    if let Some(idx) = index {
      i = idx;
    }

    let mut parent = i / 2;

    while i > 0 && self.list[parent] < self.list[i] {
      self.list.swap(i, parent);
      i = parent;
      parent = i / 2;
    }
  }

  fn swap_down(&mut self) {
    let mut i = 0;
    let mut left_child = (i * 2) + 1;
    let mut right_child = (i * 2) + 2;

    loop {
      if left_child > self.list.len() - 1 || right_child > self.list.len() - 1 {
        break
      }

      if self.list[left_child] > self.list[right_child] && self.list[i] < self.list[left_child] {
        self.list.swap(left_child, i);
        i = left_child;
      } else if self.list[i] < self.list[right_child] {
        self.list.swap(right_child, i);
        i = right_child;
      } else { break }

      left_child = (i * 2) + 1;
      right_child = (i * 2) + 2;
    }
  }

  fn insert(&mut self, value: usize) {
    self.list.push(value);
    self.swap_up(None);
  }

  fn max_heapify(&mut self) -> usize {
    let last_node = self.list.len() - 1;
    self.list.swap(0, last_node);
    let max_node = self.list[last_node];
    self.list.pop();
    self.swap_down();
    max_node
  }

  fn remove(&mut self, value: usize) {
    match self.list.iter().position(|el| el == &value) {
      None => panic!("element not found"),
      Some(index) => {
        self.list[index] = 9999;
        self.swap_up(Some(index));
        self.max_heapify();
      }
    };
  }

  fn change_priority(&mut self, from: usize, to: usize) {
    match self.list.iter().position(|el| el == &from) {
      None => panic!("element not found"),
      Some(index) => {
        self.list[index] = to;
        self.swap_up(Some(index));
        self.swap_down();
      }
    };
  }
}

fn main() {
  let mut sss = BHeap::new();
  sss.insert(11);
  sss.insert(10);
  sss.insert(5);
  sss.insert(2);
  sss.insert(1);
  sss.insert(6);
  sss.insert(7);
  sss.insert(8);
  sss.insert(3);
  sss.insert(4);
  sss.change_priority(7, 2);
  dbg!(sss);
}