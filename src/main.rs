use std::io;


fn main() {

    // let mut user_choise = String::new();
    //
    // io::stdin().read_line(&mut user_choise).unwrap();
    // let temp = user_choise
    //     .trim()
    //     .parse::<u8>()
    //     .expect("ti chel..." );

    let result1 = some(123);
    let arr = [1,2,3,4,5,6,7,8,9,10];
    let res = binary_search(3, arr);
    println!("{res}");
    // println!("{user_choise}");
    println!("{result1}");
    println!("qwe");
    println!("{res}");
}

fn some(y: i32) -> i32 {
    match y {
        1 => y,
        _ => 1,
    }
}

fn get() -> f32 {
    32.0
}

fn binary_search(y: i32, arr: [i32; 10]) -> i32 {
    let mut result = -1;
    let mut right = arr.len() as i32 - 1;
    let mut left = 0;
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid as usize] == y {
            result = mid;
            break;
        }
        if arr[mid as usize] < y {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return result;
}

// fn getq() -> String{
//     return "qwe".toString();
// }
