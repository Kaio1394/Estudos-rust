pub mod pm{
    pub fn how_many(x: i32) -> &'static str{
        match x{
            0 => "no",
            1 | 2 => "one or two",
            _ => "a few"
        }
    }
    pub fn pattern_matching(){
        for i in 0..13{
            println!("{}", how_many(i));
        }
    }
}