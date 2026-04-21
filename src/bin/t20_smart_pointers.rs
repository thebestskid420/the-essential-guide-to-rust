//! Topic 20 — Box, Deref, Drop, Rc/Arc, Cell/RefCell (skeleton — grow here).

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

struct LoudDrop(&'static str);
impl Drop for LoudDrop {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let _a = LoudDrop("a");
    let b = MyBox(String::from("boxed"));
    println!("deref: {}", b.len());

    let r = Rc::new(RefCell::new(0));
    *r.borrow_mut() += 1;
    println!("{}", r.borrow());
}
