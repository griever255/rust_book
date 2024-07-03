use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

// Listing 15-2: The first attempt at defining an enum to 
// represent a cons list data structure of i32 values
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    // Listing 15-1: Storing an i32 value on the heap using
    // a box
    // let b = Box::new(5);
    // println!("b = {b}");   

    // Listing 15-3: Using the List enum to store the list
    // 1, 2, 3
    // Listing 15-5: Definition of List that uses Box<T> in
    // order to have a known size
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Listing 15-17: Demonstrating we’re not allowed to have two lists using
    // Box<T> that try to share ownership of a third list
    // Listing 15-18: A definition of List that uses Rc<T>
    // Listing 15-19: Printing the reference count
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after create a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after create b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after create c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Listing 15-24: Using Rc<RefCell<i32>> to create a List
    // that we can mutate


    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}