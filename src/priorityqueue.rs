use std::default::Default;

// Gotta make our own since libcollections doesn't yet!
pub trait PriorityQueue <T> {
    fn push (&mut self, val: T);
    fn pop (&mut self) -> Option<T>;
    fn peek <'a> (&'a self) -> Option<&'a T>;
}

pub fn test_push <Q: PriorityQueue<uint> + Default + Collection> () {
    let mut queue: Q = Default::default();
    queue.push(1);
    assert_eq!(queue.len(), 1);
    queue.push(2);
    assert_eq!(queue.len(), 2);
}

pub fn test_pop <Q: PriorityQueue<uint> + Default + Collection> () {
    let mut queue: Q = Default::default();
    queue.push(3); queue.push(2); queue.push(4);
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.pop(), Some(4));
    assert_eq!(queue.len(), 0);
    //empty pop
    assert_eq!(queue.pop(), None);
    assert_eq!(queue.len(), 0);
}

pub fn test_peek <Q: PriorityQueue<uint> + Default + Collection> () {
    let mut queue: Q = Default::default();
    assert_eq!(queue.peek(), None);
    queue.push(3); queue.push(2); queue.push(4);
    assert_eq!(queue.peek(), Some(&2));
}