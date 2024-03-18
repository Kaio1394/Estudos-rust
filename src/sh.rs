#![allow(dead_code)]
use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
   return Point{x: 0.0, y: 0.0}
}
#[allow(unused_variables)]
pub fn stack_and_heap(){
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes up {}", mem::size_of_val(&p1));
    println!("p2 takes up {}", mem::size_of_val(&p2));
}