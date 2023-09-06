# Rust HW3



3210104927   刘子鸣



## Test 1



实现一个名为`hash_map!`的宏，它接受偶数个参数，并生成一个`std::collections::HashMap`，其中第一个参数是键，第二个参数是值，以此类推。



- 代码：

  ```rust
  #[macro_use]
  extern crate std;
  use std::collections::HashMap;
  
  macro_rules! hash_map {
      ($($key: expr => $val: expr), *) => {
          {
              let mut map = HashMap::new();
              $(
                  map.insert($key, $val);
              )*
              map
          }
      };
  }
  
  fn main() {
      let map = hash_map! {
          "one" => 1,
          "two" => 2,
          "three" => 3
      };
      println!("{:?}", map);
  }
  ```

- 运行结果：

  ![image-20230906162010710](/images/image-20230906162010710.png)



## Test 2



实现一个简易的引用计数器智能指针`MyRc`，类似于`std::rc::Rc`。



- 代码：

  ```rust
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
  ```

- 运行结果：

  ![image-20230906211838494](/images/image-20230906211838494.png)

  



## Test 3



实现一个简易的栈（`LIFO`）数据结构，支持`push`和`pop`操作，使用`RefCell`来实现内部可变性。



- 代码：

  ```rust
  use std::cell::RefCell;
  
  #[derive(Debug)]
  struct SimpleStack<T> {
      stack: RefCell<Vec<T>>,
  }
  impl<T> SimpleStack<T> {
      fn new() -> SimpleStack<T> {
          SimpleStack {
              stack: RefCell::new(Vec::new()),
          }
      }
      fn push(&self, value: T) {
          self.stack.borrow_mut().push(value);
      }
      fn pop(&self) -> Option<T> {
          self.stack.borrow_mut().pop()
      }
  }
  fn main() {
      let stack = SimpleStack::new();
      stack.push(1);
      stack.push(2);
      stack.push(3);
      
      println!("Popped value: {:?}", stack.pop());
      println!("Popped value: {:?}", stack.pop());
  
      stack.push(4);
      println!("Popped value: {:?}", stack.pop());
      println!("Popped value: {:?}", stack.pop());
      println!("Popped value: {:?}", stack.pop());
  
  }
  ```

- 运行结果：

  ![image-20230906163018987](/images/image-20230906163018987.png)

  







