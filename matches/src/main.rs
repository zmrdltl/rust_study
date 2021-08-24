
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
//열거형과 열거형의 variant를 패턴으로서 사용하는 match 표현식
// fn value_in_cents(coin: Coin) -> u32 {
//     //if를 사용한 표현식과 매우 유사하지만, 큰 차이점이 있습니다: 
//     //if를 사용하는 경우, 해당 표현식은 부울린 값을 반환할 필요가 있습니다. 
//     //여기서는 어떤 타입이든 가능합니다. 
//     //위 예제에서 coin의 타입은 Listing 6-3에서 정의했던 Coin 열거형입니다.

//     match coin {
//         Coin::Penny => 1, //패턴과 실행되는 코드를 구분해주는 => 연산자
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

//다음과 같이 중괄호 생략 가능
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five : {:?}", five);

        

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //_는 특별패턴이다.나열하지 않은 모든 가능한 값들에 대해서는 아무것도 하고 싶지 않다는 것
    }
}
