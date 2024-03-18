use std::{io::stdin, mem};
mod sh;
#[allow(unused_variables)]
#[warn(unused_unsafe)]

enum State{
    Locked,
    Failed,
    Unlocked
}

pub fn entry_password(){
    
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    loop{
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {entry.push_str(&input.trim_end());}
                    Err(_) => continue
                }
                if entry == code{
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry){
                    state = State::Failed;
                }
            },
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            },
            State::Unlocked => {
                println!("UNLOCKED");
                break;
            }
        }
    }
}
#[allow(unused_variables)]
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
    sh::stack_and_heap();

    entry_password();
    
}
