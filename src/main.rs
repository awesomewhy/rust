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
    //let result2 = some(1);
    let arr = [1,2,3,4,5,6,7,8,9,10]
    let res = binrySearch(5, arr)
    println!("{res}")
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

fn binrySearch(y: i32, arr: [i32]) -> i32{
    let result = 0;
    let right = arr.len()-1;
    let left = arr[0];
    while(left < right) {
        int temp = left + (right - left) / 2;
        if(temp == y) {
            result = temp;
            return temp;
        }
        if(y < temp) {
            right--:
        } else (
            left++;
        )
        
    }
    return result;
}

/* fn get() -> String{
    return "qwe".toString();
} */
