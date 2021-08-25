fn main() {
    //let mut v: Vec<i32> = Vec::new();
    //let v = vec![1,2,3]; //선언과 동시에 값 할당
    
    //vector 삽입
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    //println!("{:?}",v);

    //index는 0부터시작함
    //let v = vec![1, 2, 3, 4, 5];

    //원소에 접근하는 두 가지 방법
    //벡터가 가지고 있지 않은 인덱스값을 사용하고자 했을 때 
    //프로그램이 어떻게 동작할 것인지 여러분이 선택할 수 있도록 하기 위해서
    // let third: &i32 = &v[2];
    // let third: Option<&i32> = v.get(2);
    // println!("{:?}",v);

    //error code
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);
    //왜 첫번째 요소에 대한 참조자가 벡터 끝에 대한 변경을 걱정해야 하죠? 
    //이 에러에 대한 내막은 벡터가 동작하는 방법 때문입니다: 

    //새로운 요소를 벡터의 끝에 추가하는 것은 새로 메모리를 할당하여 예전 요소를 새 공간에 복사하는 일을 필요로 할 수 있는데, 
    //이는 벡터가 모든 요소들을 붙여서 저장할 공간이 충분치 않는 환경에서 일어날 수 있습니다. 
    //이러한 경우, 첫번째 요소에 대한 참조자는 할당이 해제된 메모리를 가리키게 될 것입니다. 
    //빌림 규칙은 프로그램이 이러한 상황에 빠지지 않도록 해줍니다.

    //vector원소 출력하기
    let v = vec![100,32,57];
    for i in &v{
        println!("{}", i);
    }

    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}",i);
    }

    //enum 으로 여러타입 저장
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
}
