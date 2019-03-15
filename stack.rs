#[derive(Debug, Clone)]
struct DataStack<T: Clone> {
  values: Vec<T>,
  stack_size: usize
}

trait IDataStack<T> {
  fn new(size: usize) -> Self;
  fn push(&mut self, new_value: T);
  fn undo(&mut self);
  fn get_top(&self) -> Option<T>;
  fn is_empty(&self) -> bool;
}

impl<T> IDataStack<T> for DataStack<T>
  where T: Clone
{
  fn new(size: usize) -> Self {
    let values = Vec::with_capacity(size);
    DataStack { values, stack_size: size }
  }

  fn push(&mut self, new_value: T) {
    let current_langth = self.values.len();
    if self.stack_size == current_langth {
      panic!("stack is full");
    }

    self.values.push(new_value);
  }

  fn undo(&mut self) {
    self.values.pop();
  }

  fn is_empty(&self) -> bool {
    match self.values.len() {
      0 => true,
      _ => false
    }
  }

  fn get_top(&self) -> Option<T> {
    if self.is_empty() == true {
      panic!("stack is empty")
    }

    let index_top: usize = self.values.len() - 1;
    Some(self.values[index_top].clone())
  }
}

fn main() {
  let mut new_stack = DataStack::new(5);
  new_stack.push(1);
  println!("stack element {}", new_stack.get_top().unwrap());
  new_stack.push(2);
  println!("stack element {}", new_stack.get_top().unwrap());
  new_stack.push(3);
  println!("stack element {}", new_stack.get_top().unwrap());
  new_stack.push(4);
  println!("stack element {}", new_stack.get_top().unwrap());

  new_stack.undo();
  println!("stack element {}", new_stack.get_top().unwrap());
  new_stack.undo();
  println!("stack element {}", new_stack.get_top().unwrap());
  new_stack.undo();
  println!("stack element {}", new_stack.get_top().unwrap());
  new_stack.undo();
  println!("stack element {}", new_stack.get_top().unwrap());
}