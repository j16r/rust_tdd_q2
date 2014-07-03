use std::default::Default;

struct Q<T> {
  items : [T, ..10],
  count : uint
}

impl<T : Default + Copy> Q<T> {
  pub fn new() -> Box<Q<T>> {
    box Q {items: [Default::default(), ..10],
           count: 0}
  }

  pub fn empty(&self) -> bool {
    self.count == 0
  }

  pub fn enqueue(&mut self, item : T) {
    self.items[self.count] = item;
    self.count += 1
  }

  pub fn dequeue(&mut self) -> T {
    Default::default()
  }

  pub fn size(&self) -> uint {
    self.count
  }

  pub fn iter<'r>(&'r self) -> Box<QCursor<'r, T>> {
    box QCursor {q: self, position: 0}
  }
}

struct QCursor<'r, T> {
  q : &'r Q<T>,
  position : uint
}

impl<'r, T> Iterator<&'r T> for QCursor<'r, T> {
  fn next(&mut self) -> Option<&'r T> {
    if self.position < self.q.count {
      self.position += 1;
      Some(&self.q.items[self.position - 1])
    } else {
      None
    }
  }
}

#[test]
fn test_empty() {
  let q = Q::<int>::new();

  assert!(q.empty());
}

#[test]
fn test_enqueue() {
  let mut q = Q::<int>::new();
  
  q.enqueue(99);
  assert!(!q.empty());
}

#[test]
#[should_fail]
fn test_enqueue_too_many_items() {
  let mut q = Q::<int>::new();
  
  for _ in range(0u, 11u) {
    q.enqueue(0);
  }
}

#[test]
fn test_dequeue() {
  let mut q = Q::<int>::new();
  
  q.enqueue(99);
  q.dequeue();
  assert!(q.empty());
}

#[test]
fn test_size() {
  let mut q = Q::<int>::new();
  
  q.enqueue(7);
  q.enqueue(102);
  
  assert!(q.size() == 2);
}

#[test]
fn test_iter() {
  let mut q = Q::<int>::new();

  q.enqueue(3);
  q.enqueue(2);
  q.enqueue(1);

  assert!(q.iter().count() == 3);
  assert!(q.iter().fold(0, |acc, &val| acc + val) == 6);
}

pub fn main() {
  let mut q = Q::<int>::new();

  q.enqueue(3);
  q.enqueue(2);
  q.enqueue(1);

  for item in q.iter() {
    print!("Item is {}\n", item);
  }
}
