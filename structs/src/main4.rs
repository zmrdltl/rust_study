#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
//멤버 함수 만들기
impl Rectangle {
    //self 앞에 & 주의
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, r:&Rectangle) -> bool {
        self.length > r.length && self.width > r.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {length: size, width: size}
    }
}
fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    //can_hold 함수 : Rectangle 인스턴스를 가져와서 이 두번째 Rectangle이 self내에 완전히 안에 들어갈 수 있다면 true
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let sq = Rectangle::square(3);
    println!("SQUARE {:?}!", sq);
    println!("SQUARE {:#?}!", sq);
}
    
