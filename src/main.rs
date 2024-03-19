use std::mem;
mod sh;

#[allow(unused_imports)]
use crate::sh::sh::{stack_and_heap, entry_password, IntOrFloat};

#[allow(unused_variables)]
#[warn(unused_unsafe)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    let mut age = 30;
    // criando um ponteiro e apontando para a variável age
    let ptr = &mut age as *mut i32;
    println!("{:?}-{}", ptr, unsafe {*ptr});
    // Trocando o valor da variável através do ponteiro
    unsafe{*ptr = 29;}    
    //Usando aritmétrica de ponteiros
    println!("{:?}-{}",  ptr.wrapping_offset(1), unsafe {*ptr});
    let ptr2 = ptr.clone();
    //Unsafe é usado para acessar um valor de um ponteiro
    println!("PTR2: {:?}-{}", ptr2, unsafe {*ptr2});
    unsafe{*ptr = 1;} 
    println!("PTR2: {:?}-{}", ptr2, unsafe {*ptr2});
    let z: isize = 100;
    println!("Size: {}", mem::size_of_val(&age) * 8);

    let f1 = f64::powf(5.5, 2.0);
    println!("Take up {} bytes", mem::size_of_val(&age));
    // stack_and_heap();
    // entry_password();
    let mut iof = IntOrFloat{i: 123};
    println!("{}", unsafe{ iof.i} );
}
