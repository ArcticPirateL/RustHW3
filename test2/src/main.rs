use std::ops::Deref;
use std::ptr::NonNull;

struct MyRc<T> {
    value: NonNull<T>,
    count: NonNull<usize>,
}

impl<T> MyRc<T> {
    pub fn new(data: T) -> Self {
        let value_box = Box::new(data);
        let count_box = Box::new(1);
        MyRc {
            value: NonNull::new(Box::into_raw(value_box)).unwrap(),
            count: NonNull::new(Box::into_raw(count_box)).unwrap(),
        }
    }
    pub fn clone(&mut self) -> Self {
        unsafe {
            *self.count.as_mut() += 1;
        }
        MyRc {
            value: self.value,
            count: self.count,
        }
    }
    fn strong_count(&self) -> usize {
        unsafe {
            *self.count.as_ref()
        }
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            if *self.count.as_ref() > 1 {
                *self.count.as_mut() -= 1;
            } else {
                let _ = Box::from_raw(self.count.as_ptr());
            }
        }
    }
}
//测试
fn main() {
    let mut rc1 = MyRc::new(42);
    println!("count1 = {}", MyRc::strong_count(&rc1));
    let rc2 = MyRc::clone(&mut rc1);
    println!("count2 = {}", MyRc::strong_count(&rc1));
    println!("rc1 = {}", *rc1);
    println!("rc2 = {}", *rc2);
}