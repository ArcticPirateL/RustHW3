use std::ops::Deref;
use std::cell::RefCell;

struct MyRc<T> {
    value: T,
    count: RefCell<usize>,
}

impl<T: Clone> MyRc<T> {
    fn new(value: T) -> Self {
        MyRc {
            value,
            count: RefCell::new(1),
        }
    }

    fn clone(&self) -> Self {
        *self.count.borrow_mut() += 1;
        MyRc {
            value: self.value.clone(),
            count: self.count.clone(),
        }
    }
    fn strong_count(&self) -> usize {
        return *self.count.borrow();
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let mut count = self.count.borrow_mut();
        *count -= 1;
        if *count == 0 {
            drop(&self.value);
        }
    }
}

fn main() {
    let rc1 = MyRc::new(42);
    println!("Count1: {}", MyRc::strong_count(&rc1));
    let rc2 = MyRc::clone(&rc1);
    println!("Count2: {}", MyRc::strong_count(&rc1));
    println!("rc1: {}, rc2: {}", *rc1, *rc2);
}
