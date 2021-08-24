//if let 문법 : 
//if와 let을 조합하여 하나의 패턴만 매칭 시키고 나머지 경우는 무시하는 값


#![allow(unused)]
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    //위 구문은 너무 길다 이를 if let하면
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    println!("{:?}",some_u8_value);
}
