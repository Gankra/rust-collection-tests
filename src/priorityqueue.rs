use std::default::Default;
use test::{Bencher, black_box};
use super::utils::Countable;
use StdQueue = std::collections::PriorityQueue;

// Gotta make our own trait since libcollections doesn't yet!
pub trait PriorityQueue <T> : Mutable {
    fn push (&mut self, val: T);
    fn pop (&mut self) -> Option<T>;
    fn peek <'a> (&'a self) -> Option<&'a T>;
}

// Impl it for std, because we're nice like that
mod hack {
    use std::collections::PriorityQueue;
    #[inline]
    pub fn push <T:Ord> (queue: &mut PriorityQueue<T>, val: T) { queue.push(val); }
    #[inline]
    pub fn pop <T:Ord> (queue: &mut PriorityQueue<T>) -> Option<T> { queue.pop() }    
}

impl <T: Ord> PriorityQueue <T> for StdQueue <T> {
    #[inline]
    fn push (&mut self, val: T) { hack::push(self, val); }
    #[inline]
    fn pop (&mut self) -> Option<T> { hack::pop(self) }
    #[inline]
    fn peek <'a> (&'a self) -> Option<&'a T> { self.top() }
}




// Testing

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





// Benching

// iteratively inserts all elements, and then iteratively pops them all off
pub fn bench_fill_and_drain <Q: PriorityQueue<N> + Default, N:Countable, I:Iterator<N> + Clone> 
    (seq: I, bencher: &mut Bencher) {
    bencher.iter(||{
        let mut queue:Q = Default::default();
        for item in seq.clone() {
            queue.push(item);
        }
        while !queue.is_empty() {
            black_box(queue.pop());
        }
    });
}

// iteratively inserts all elements, and then pops one value off
pub fn bench_fill_and_pop <Q: PriorityQueue<N> + Default, N:Countable, I:Iterator<N> + Clone> 
    (seq: I, bencher: &mut Bencher) {
    bencher.iter(||{
        let mut queue:Q = Default::default();
        for item in seq.clone() {
            queue.push(item);
        }
        black_box(queue.pop());
    });
}

// alternates between pushing 2 elements, and poppping 1
pub fn bench_mixed_access <Q: PriorityQueue<N> + Default, N:Countable, I:Iterator<N> + Clone> 
    (seq: I, bencher: &mut Bencher) {
    bencher.iter(||{
        let mut queue:Q = Default::default();
        let mut count = 0u;
        for item in seq.clone() {
            count += 1;
            if count > 2 {
                count = 0;
                black_box(queue.pop());
            } else {
                queue.push(item);
            }
        }
    });
}

#[macro_export]
macro_rules! bench_priorityqueue(
    ($queue_type:ty) => (  

    type ToTest = $queue_type;

    #[bench]
    fn from_iter_ord_small (bencher: &mut Bencher) {
        collection::bench_from_iter::<ToTest, _, _>(utils::ordered_sequence::<uint>(2), bencher);
    }

    #[bench]
    fn from_iter_ord_big (bencher: &mut Bencher) {
        collection::bench_from_iter::<ToTest, _, _>(utils::ordered_sequence::<uint>(4), bencher);
    }

    #[bench]
    fn from_iter_unord_small (bencher: &mut Bencher) {
        collection::bench_from_iter::<ToTest, _, _>(utils::unordered_sequence::<uint>(2), bencher);
    }

    #[bench]
    fn from_iter_unord_big (bencher: &mut Bencher) {
        collection::bench_from_iter::<ToTest, _, _>(utils::unordered_sequence::<uint>(4), bencher);
    }

    

    #[bench]
    fn fill_and_pop_ord_small (bencher: &mut Bencher) {
        priorityqueue::bench_fill_and_pop::<ToTest, _, _>(utils::ordered_sequence::<uint>(2), bencher);
    }

    #[bench]
    fn fill_and_pop_ord_big (bencher: &mut Bencher) {
        priorityqueue::bench_fill_and_pop::<ToTest, _, _>(utils::ordered_sequence::<uint>(4), bencher);
    }

    #[bench]
    fn fill_and_pop_unord_small (bencher: &mut Bencher) {
        priorityqueue::bench_fill_and_pop::<ToTest, _, _>(utils::unordered_sequence::<uint>(2), bencher);
    }

    #[bench]
    fn fill_and_pop_unord_big (bencher: &mut Bencher) {
        priorityqueue::bench_fill_and_pop::<ToTest, _, _>(utils::unordered_sequence::<uint>(4), bencher);
    }



    #[bench]
    fn fill_and_drain_ord_small (bencher: &mut Bencher) {
        priorityqueue::bench_fill_and_drain::<ToTest, _, _>(utils::ordered_sequence::<uint>(2), bencher);
    }

    #[bench]
    fn fill_and_drain_ord_big (bencher: &mut Bencher) {
        priorityqueue::bench_fill_and_drain::<ToTest, _, _>(utils::ordered_sequence::<uint>(4), bencher);
    }

    #[bench]
    fn fill_and_drain_unord_small (bencher: &mut Bencher) {
        priorityqueue::bench_fill_and_drain::<ToTest, _, _>(utils::unordered_sequence::<uint>(2), bencher);
    }

    #[bench]
    fn fill_and_drain_unord_big (bencher: &mut Bencher) {
        priorityqueue::bench_fill_and_drain::<ToTest, _, _>(utils::unordered_sequence::<uint>(4), bencher);
    }



    #[bench]
    fn mixed_access_ord_small (bencher: &mut Bencher) {
        priorityqueue::bench_mixed_access::<ToTest, _, _>(utils::ordered_sequence::<uint>(2), bencher);
    }

    #[bench]
    fn mixed_access_ord_big (bencher: &mut Bencher) {
        priorityqueue::bench_mixed_access::<ToTest, _, _>(utils::ordered_sequence::<uint>(4), bencher);
    }

    #[bench]
    fn mixed_access_unord_small (bencher: &mut Bencher) {
        priorityqueue::bench_mixed_access::<ToTest, _, _>(utils::unordered_sequence::<uint>(2), bencher);
    }

    #[bench]
    fn mixed_access_unord_big (bencher: &mut Bencher) {
        priorityqueue::bench_mixed_access::<ToTest, _, _>(utils::unordered_sequence::<uint>(4), bencher);
    }

    );
)

#[cfg(test)]
mod bench {
    use super::super::priorityqueue;
    use super::super::collection;
    use super::super::utils;
    use test::Bencher;

    use std::collections::PriorityQueue;
    bench_priorityqueue!(PriorityQueue<uint>)
}