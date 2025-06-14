// struct MyBox<T> {
//     v: T,
// }

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox { v: x }
//     }
// }

// use std::ops::Deref;

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.v
//     }
// }

// use std::ops::DerefMut;

// impl<T> DerefMut for MyBox<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.v
//     }
// }

// fn main() {
//     let mut s = MyBox::new(String::from("hello, "));
//     display(&mut s)
// }

// fn display(s: &mut String) {
//     s.push_str("world");
//     println!("{}", s);
// }

// use std::rc::Rc;
// fn main() {
//   let s = Rc::new(String::from("hello world"));
//   let s1 = Rc::clone(&s);
//   println!("{} , {}" , s , s1);
// }

#[allow(unused)]
// struct MyBox<T>{
//   v: T
// }
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox{v: x}
//     }
// }
// use std::ops::Deref;
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.v
//     }
// }
// fn display(s: String) {
//     println!("{}", s);
// }
use std::rc::Rc;
fn main() {
    let s = Rc::new(Box::new(String::from("hello")));
    let s1 = Rc::clone(&s);
    println!("{} , {}" , s , *s1);
}