//열거형 장점 : 다른 타입, 다른 양의 연관 데이터를 하나로 묶어 저장 가능
// 어떤 타입도 열거형으로 들어갈 수 있다.
// struct Ipv4Addr {
//     // details elided
// }

// struct Ipv6Addr {
//     // details elided
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//enumerator can take every types of data
//ex1   
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//ex2

struct QuitMessage; // 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의할 수 있습니다.
        println!("hello");
    }
}

//option 개념도 있다. 
//컴파일러는 Option<T> 값을 명확하게 유효한 값처럼 사용하지 못하도록 한다.

enum Option<T> {
    Some(T),
    None,
}
fn main() {
    //열거형은 다음과 같이 저장한다.
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
        
    let home = IpAddr::V4(127,0,0,1);

    let loopback =  IpAddr::V6(String::from("::1"));
    
    
    let m = Message::Write(String::from("hello"));
    m.call();

    //option error code, Option<T>와 T는 다른 타입이다.
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}
