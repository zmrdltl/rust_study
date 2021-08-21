//lifetime error 발생 경우 
//구조체가 소유권이 없는 데이터의 참조를 저장할수는 있지만, 10장에서 언급 될 라이프타임(lifetimes) 의 사용을 전제로 함

//라이프타임은 구조체가 존재하는동안 참조하는 데이터를 계속 존재할 수 있도록 합니다. 라이
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}