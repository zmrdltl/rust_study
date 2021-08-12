fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    another_function3(5, 6);

    //wrong code 
    // let x = (let y = 6);

    // let x = 5;
    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    //println!("The value of y is: {}", y);

    //functions with return value
    // let x = five();
    // println!("The value of x is: {}", x);
    
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

//functions with return value
fn five() -> i32 { 5 }

fn plus_one(x: i32) -> i32 {
    x + 1
}

//wrong code cause of semi-colon
fn plus_one(x: i32) -> i32 {
    x + 1
}

//function's position doesn't matter
fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}


fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

