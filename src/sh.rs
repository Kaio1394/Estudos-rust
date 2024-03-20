#![allow(dead_code)]
pub mod sh {
    use std::mem;
    struct Point
    {
        x: f64,
        y: f64
    }
    struct Line{
        start: Point,
        end: Point
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
                    match std::io::stdin().read_line(&mut input) {
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
    // Estrutura que aloca tipos diferentes e memória
    pub union IntOrFloat{
        pub i: i32,
        pub f: f32
    }
    pub fn print_array(array: [i32;5]){
        for i in array{
           println!("{}", i); 
        }
    }
    pub fn change_array(array: [i32;5], index: usize, new_value: i32) -> [i32;5]{
        let mut new_array: [i32;5] = array.clone();
        if index as i32 <= array.len() as i32{
            new_array[index] = new_value;
        }
        else {
            panic!("Índice fora dos limites da array.");
        }
        return new_array;
    }
    pub fn change_array2(array: &mut [i32;5], index: usize, new_value: i32){
        if index as i32 <= array.len() as i32{
            array[index] = new_value;
        }
        else {
            panic!("Índice fora dos limites da array.");
        }
    }
    pub fn get_all_local_memory_array(array: &mut [i32; 5]) ->  [*mut i32; 5] {
        let mut array2: [*mut i32; 5] = [std::ptr::null_mut(); 5]; 
        let mut index = 0;
        for i in array.iter_mut() {
            array2[index] = i as *mut i32; 
            index += 1;
        }
        return array2;
    }    
}
