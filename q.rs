use std::default::Default;

static Q_CAPACITY: uint = 10u;

struct Q<T> {
  items : [T, ..Q_CAPACITY],
  tail : uint,
  head : uint
}

impl<T : Default + Copy> Q<T> {
  pub fn new() -> Box<Q<T>> {
    box Q {items: [Default::default(), ..Q_CAPACITY],
           tail: 0,
           head: 0}
  }

  pub fn empty(&self) -> bool {
    self.count() == 0
  }

  pub fn enqueue(&mut self, item : T) {
    if self.count() + 1 == Q_CAPACITY {
      fail!("Queue is limited to {:u} items", Q_CAPACITY)
    }
    self.items[self.tail] = item;
    self.tail = (self.tail + 1) % Q_CAPACITY
  }

  pub fn dequeue(&mut self) -> Option<T> {
    if self.count() == 0 {
      None
    } else {
      let val : T = self.items[self.head];
      self.head = (self.head + 1) % Q_CAPACITY;
      Some(val)
    }
  }

  pub fn count(&self) -> uint {
    self.tail - self.head
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
    if self.position < self.q.tail {
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

  assert!(q.dequeue().is_none());
  
  q.enqueue(99);
  q.dequeue();
  assert!(q.empty());

  q.enqueue(11);
  assert!(q.dequeue().unwrap() == 11);
}

#[test]
fn test_dequeue_recycle() {
  let mut q = Q::<int>::new();
  
  for val in range(0i, 11i) {
    q.enqueue(val);
    assert!(q.dequeue().unwrap() == val);
  }
}

#[test]
fn test_count() {
  let mut q = Q::<int>::new();
  
  q.enqueue(7);
  q.enqueue(102);
  
  assert!(q.count() == 2);
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
