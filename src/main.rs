const C: f32 = 23.0; 
use std::io;


fn main() {

    let mut user_choise = String::new();

    io::stdin().read_line(&mut user_choise).unwrap();
    let temp = user_choise
        .trim()
        .parse::<u8>()
        .expect("ti chel..." );

    let result1 = some(123);
    let result2 = some(1);
    
    println!("{user_choise}");
    println!("{result1}");
    println!("{result2}");
}

fn some(y: i32) -> i32 {
    match y {
        1 => y
    }
}

fn get() -> f32 {
    32.0
}

/* fn get() -> String{
    return "qwe".toString();
} */
