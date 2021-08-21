// 명시적인 사전동의 Debug 출력포멧 기능을 사용하겠다
#[derive(Debug)]

struct Rectangle {
    length: i32,
    width: i32,
}

fn main() {
    let rec1 = Rectangle { length: 50, width: 30 };


    //참조형태로 구조체 함수에 인자로 전달 가능
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec1)
    );

    //debug annotaion 추가 후
    println!("rec1 is {:?}", rec1);

    //큰 구조체를 더 깔끔하게 보려면?
    println!("rec1 is {:#?}", rec1);

}

fn area(r: &Rectangle) -> i32 {
    r.length * r.width
}