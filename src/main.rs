use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct MySt {
    lol: i32,
    asd: i32,
    kek: String,
    qwe: f32
}

#[derive(Debug)]
struct qwe_asd {
    lol: i32,
    asd: i32,
    kek: String,
    qwe: f32
}

fn main() {
    let mut st: MySt = MySt {
        lol: 32,
        asd: 12,
        kek: String::from("qwe"),
        qwe: 123.0
    };

    let mut qwe: MySt = MySt {
        lol: 32,
        asd: 500,
        kek: String::from("qwe"),
        qwe: 123.0
    };

    let mut lol: qwe_asd = qwe_asd {
        lol: 32,
        asd: 999,
        kek: String::from("qwe"),
        qwe: 123.0
    };

    let mut asd = &mut st;

    asd.asd = 234444444;

    print!("{:?}\n", asd);

    asd = &mut qwe;

    print!("{:?}\n", asd);

    let asd = &mut lol;

    println!("{}", asd.asd);



    let mut vec: Vec<i32> = vec![1,2,3];

    for el in vec.iter_mut() {
        println!("{}", el);
        *el *= 2;
    }

    for el in &vec {
        // println!("{}", el)
    }

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
