#[derive(Debug)]
struct Heap {
  nodes: Vec<u64>
}

impl Heap {
  pub fn new() -> Self {
    Heap { nodes: vec![0] }
  }

  pub fn insert(&mut self, value: u64) {
    let lastIndex = self.nodes.len();
    self.nodes.push(value);
    self.siftUp(lastIndex);
  }

  pub fn extract_max(&mut self) -> u64 {
    let firstIndex = 1;
    let lastIndex = self.nodes.len() - 1;
    let max = self.nodes[firstIndex];
    self.nodes.swap(firstIndex, lastIndex);
    self.nodes.pop();
    if self.nodes.len() == 0 { return max; }
    self.siftDown(firstIndex);
    max
  }

  fn siftUp(&mut self, index: usize) {
    let mut i = index;
    let mut parent = self.parent(&i);
    while parent >= 1 {
      if self.nodes[parent] < self.nodes[i] {
        self.nodes.swap(parent, i);
      }

      i = parent;
      parent = self.parent(&i);
    }
  }

  fn siftDown(&mut self, index: usize) {
    let mut left = self.left(&index);
    let mut right = self.right(&index);
    let mut i = index;
    let mut nextIndex = 0;

    while left < self.nodes.len() {
      if right < self.nodes.len() && self.nodes[right] > self.nodes[left] {
        nextIndex = right;
      } else {
        nextIndex = left;
      }

      if self.nodes[i] < self.nodes[nextIndex] {
        self.nodes.swap(i, nextIndex);
        i = nextIndex;
        left = self.left(&i);
        right = self.right(&i);
      } else { break }
    }
  }

  fn left(&self, index: &usize) -> usize {
    index * 2
  }

  fn right(&self, index: &usize) -> usize {
    (index * 2) + 1
  }

  fn parent(&self, index: &usize) -> usize {
    index / 2
  }
}

fn main() {
  let mut heap = Heap::new();
  heap.insert(304);
  heap.insert(255);
  heap.insert(146);
  heap.insert(29);
  heap.insert(157);
  heap.insert(96);
  heap.insert(105);
  heap.extract_max();
  heap.insert(200);
  heap.insert(10);
  heap.extract_max();
  heap.insert(5);
  heap.insert(500);
  heap.extract_max();
  heap.insert(10);
  heap.insert(8);
  heap.insert(10);
  heap.insert(10);
  heap.extract_max();
  heap.extract_max();
  heap.insert(2);
  heap.insert(3);
  heap.insert(15);
  heap.insert(18);
  heap.insert(12);
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.insert(3);
  heap.insert(0);
  heap.extract_max();
  heap.extract_max();
  heap.insert(32323);
  heap.insert(334);
  heap.insert(11111);
  heap.extract_max();
  heap.extract_max();
  heap.insert(323123123);
  heap.insert(100000000);
  heap.insert(323123123);
  heap.insert(100000000);
  heap.insert(323123123);
  heap.insert(100000000);
  heap.insert(323123123);
  heap.insert(100000000);
  heap.insert(323123123);
  heap.insert(100000000);
  heap.insert(323123123);
  heap.insert(100000000);
  heap.insert(323123123);
  heap.insert(100000000);
  heap.insert(323123123);
  heap.insert(100000000);
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();
  heap.extract_max();

  println!("{:#?}", heap);
}