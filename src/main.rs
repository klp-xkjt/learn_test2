use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {name}")
}

fn main() {
    let a = 5;
    let b = MyBox::new(a);
    assert_eq!(5, a);
    assert_eq!(5, *b);

    let gate = MyBox::new(String::from("Gate"));
    hello(&gate);
    hello(&*gate);

    let mut b2 = MyBox::new(200);
    *b2 = 999;
    println!("{}", *b2); 
}