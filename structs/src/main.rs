struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


//원하는 정보로 구조체 인스턴스 생성 후 반환
fn build_user(email: String, username: String) -> User{
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//변수명 filed명 같으면 자동으로 됨
// fn build_user(email: String, username: String) -> User{
//     User {
//         email,
//         username,
//         acrive,
//         sign_in_count,
//     }
// }

fn main(){
        
    //make instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    //구조체 일부 재사용 가능
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    //구조체 갱신법은
    //입력으로 주어진 인스턴스와 변화하지 않는 필드들을 명시적으로 할당하지 않기 위해 .. 구문
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     ..user1
    // };


    //이름 없이 튜플 구조체 사용하기

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


