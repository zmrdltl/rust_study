//카고는 src/main.rs를 바이너리 크레이트의 루트 파일로 취급하는데, 
//이 바이너리 크레이트는 src/lib.rs가 루트 파일인 
//이미 있던 라이브러리 크레이트는 별개
//대부분의 기능은 라이브러리 크레이트 안에 있고, 
//바이너리 크레이트는 이 라이브러리 크레이트를 이용

//러스트의 모든 코드의 기본 상태는 비공개입니다: 
// 즉, 다른 사람은 이 코드를 사용할 수 없습니다. 
// 만일 여러분의 프로그램 내에서 비공개 함수를 이용하지 않는다면, 
// 여러분의 프로그램이 그 함수를 이용할 수 있는 유일한 곳이기 때문에, 
// 러스트는 그 함수가 사용된 적이 없다며 경고해줄 것입니다.

extern crate communicator;


pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

//use a::series::of; //이렇게 모듈 경로가 길어지는 걸 막아줄 수 있다
//함수 자체도 가능

use a::series::of::nested_modules;

fn main() {
    communicator::client::connect();
    // of::nested_modules();//of로 치환가능
    //함수 자체도 가능
    nested_modules();
}