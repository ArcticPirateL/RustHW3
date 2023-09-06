use std::ops::Deref;

struct MyRc<T> {
    value: *mut T,
    count: *mut usize,
}
impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        let value_box = Box::new(value);
        let count_box = Box::new(1);
        MyRc {
            value: Box::into_raw(value_box),
            count: Box::into_raw(count_box),
        }
    }
    fn clone(&self) -> Self {
        unsafe {
            *self.count += 1;
        }
        MyRc {
            value: self.value,
            count: self.count,
        }
    }
    fn strong_count(&self) -> usize {
        unsafe {*self.count}
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.value }
    }
}
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            if *self.count > 1 {
                *self.count -= 1;
            }
            else {
                let _ = Box::from_raw(self.value);
                let _ = Box::from_raw(self.count);
            }
        }
    }
}
fn main() {
    let rc1 = MyRc::new(42);
    println!("count1 = {}", MyRc::strong_count(&rc1));
    let rc2 = MyRc::clone(&rc1);
    println!("count1 = {}", MyRc::strong_count(&rc1));
    println!("rc1: {}, rc2: {}", *rc1, *rc2);
}