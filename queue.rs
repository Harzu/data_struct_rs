#[derive(Debug, Clone)]
struct DataQueue<T: Clone> {
  values: Vec<T>,
  queue_size: usize
}

trait IDataQueue<T> {
  fn new(size: usize) -> Self;
  fn get_top(&self) -> Option<T>;
  fn enqueue(&mut self, new_value: T);
  fn dequeue(&mut self);
  fn is_empty(&self) -> bool;
}

impl<T> IDataQueue<T> for DataQueue<T>
  where T: Clone
{
  fn new(size: usize) -> Self {
    assert!(size > 0);
    let values = Vec::with_capacity(size);
    DataQueue { values, queue_size: size }
  }

  fn enqueue(&mut self, new_value: T) {
    let queue_length = self.values.len();
    if queue_length == self.queue_size {
      panic!("queue is full");
    }

    self.values.push(new_value);
  }

  fn dequeue(&mut self) {
    self.values.remove(0);
  }

  fn is_empty(&self) -> bool {
    match self.values.len() {
      0 => true,
      _ => false
    }
  }

  fn get_top(&self) -> Option<T> {
    if self.is_empty() == true {
      panic!("queue is empty");
    }

    Some(self.values[0].clone())
  }
}

fn main() {
  let mut new_queue = DataQueue::new(5);
  new_queue.enqueue(1);
  println!("queue element {}", new_queue.get_top().unwrap());
  new_queue.enqueue(2);
  println!("queue element {}", new_queue.get_top().unwrap());
  new_queue.enqueue(3);
  println!("queue element {}", new_queue.get_top().unwrap());
  new_queue.enqueue(4);
  println!("queue element {}", new_queue.get_top().unwrap());

  new_queue.dequeue();
  println!("queue element {}", new_queue.get_top().unwrap());
  new_queue.dequeue();
  println!("queue element {}", new_queue.get_top().unwrap());
  new_queue.dequeue();
  println!("queue element {}", new_queue.get_top().unwrap());
  new_queue.dequeue();
  println!("queue element {}", new_queue.get_top().unwrap());
}