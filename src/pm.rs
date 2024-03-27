pub mod pm{
    use std::{any::type_name, mem};

    pub fn how_many(x: i32) -> &'static str{
        match x{
            0 => "no",
            1 | 2 => "one or two",
            _ => "a few"
        }
    }
    //Generic
    pub struct Point<T>{
        pub x: T,
        pub y: T
    }
    pub fn pattern_matching(){
        for i in 0..13{
            println!("{}", how_many(i));
        }
    }
    pub fn function_generic<T>(x: T, y: T){
        let typeX = mem::size_of_val(&x);
        println!("Type variable {}", typeX);
    }
}