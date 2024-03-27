use std::{any::type_name, collections::{HashMap, HashSet}, mem, ptr::null, vec};
mod sh;
mod pm;
use crate::{pm::pm::{pattern_matching, Point, function_generic}, sh::sh::print_array};
#[allow(unused_imports)]
use crate::sh::sh::{stack_and_heap, entry_password, IntOrFloat, change_array2, get_all_local_memory_array, slice, sum_and_product};

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

    let x = 3.0;
    let y = 1.0;
    let result = 
        if y!= 0.0 {Some(x/y)} else {None};
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Error")
    }
    if let Some(z) = result{
        println!("Result = {}", z);
    }

    let mut array = [1, 2, 3, 4, 5];
    print_array(array);
    change_array2(&mut array, 1, 13);
    print_array(array);
    let array_memory = get_all_local_memory_array(&mut array);
    for i in array_memory{
        println!("Endereço de memória: {:?}", i);
    }
    slice(&mut array[1..2]);
    let teste = sum_and_product(5, 5);
    println!("{:?}", &mut array[4..5]);

    pattern_matching();

    let p1 = Point{x: 6, y: 7};
    let p2 = Point{x: 6.0, y: 7.0};
    let x = 5;
    function_generic(5, 6);
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(15);
    v1.push(25);
    v1.push(35);
    v1.push(45);

    let index: usize = 0;
    match v1.get(6){
        Some(x) => println!("{}", x),
        None => println!("Error.")
    }
    while let Some(x) = v1.pop(){println!("{}", x)}

    let mut h = HashMap::new();
    h.insert( "Kaio", 1);
    {
        //Se encontrar a key 'Kaio', será alterado o valor se não insere uma nova key com um novo valor
        let mut actual = h.entry("Kaio".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", h);

    let mut h = HashSet::new();
    let added = h.insert("value");
    let mut h1: HashSet<_> = (1..=5).collect();
    let mut h2: HashSet<_> = (2..=4).collect();
    println!("Is {:?} a subset {:?}? {}", h2, h1, h2.is_subset(&h1));

}
