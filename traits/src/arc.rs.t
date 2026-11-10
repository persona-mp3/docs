use std::sync::atomic::AtomicUsize;
use std::sync::atomic::*;
struct Arc<T>
where
    T: Clone,
{
    inner: T,
    // not sure, but im sure each clone also gets a pointer to this? or
    // the drop does that
    count: AtomicUsize,
}
// assert_eq!(x.update(Ordering::SeqCst, Ordering::SeqCst, |x| x + 1), 7);

impl<T> Clone for Arc<T>
where
    T: Clone + Copy + std::ops::Deref<Target = T>,
{
    fn clone(&self) -> Self {
        // if self.count.load(Ordering::Relaxed) == 0 {
        let c = self
            .count
            .update(Ordering::SeqCst, Ordering::SeqCst, |c| c + 1);
        // };
        Arc {
            inner: *self.inner,
            count: AtomicUsize::new(1),
        }
    }
}

impl<T> Drop for Arc<T>  where T: Clone {
    fn drop(&mut self) {
    }
}

// or something like that, was just spittballing based of how I've interacted with
// Arc
