//library crate란 다른 사람들이 자신들의 프로젝트에 디펜던시(dependency)로 추가할 수 있는 프로젝트
//src/main.rs 파일이 없기 떄문에, cargo run 커맨드로 카고가 실행할 것이 없습니다. 따라서, 여기서는 라이브러리 크레이트의 코드를 컴파일하기 위해 cargo build를 사용할 것


//이 모듈에서 clien는 network의 자식이다.
// mod network{
//     fn connect(){
//     }
    
//     mod client {
//         fn connect(){
    
//         }
//     }
// }

//이 경우를 해보자
mod client; //이 모듈은 다른 위치에서 찾아야한다.

mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
