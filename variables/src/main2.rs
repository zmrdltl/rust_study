fn main(){
    //data types
    //ambiguous type have to specify
    //let guess = "42".parse().expect("Not a number!");

    //u32 -> unsigned int, so can't present negative
    //let guess: u32 = "-23".parse().expect("Not a number!");
    let guess: i32 = "-23".parse().expect("Not a number!");
    println!("{}",guess);


    //floating point
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //basic operations =======================
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;


    //boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    //char : single quotation marks, string : double
    let c = 'z';
    let z = '?';
    let heart_eyed_cat = '?';
    println!("{}, {}", z, heart_eyed_cat);

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //to use variables in a tuple, have to rescue them
    let (x,y,z) = tup;
    println!("The value of y is: {}, {}, {}", x,y,z);

    //array
    let a = [1,2,3,4,5];

    //attach to array
    let first = a[0];
    let second = a[1];

    //wrong attachment if program got an error it called panic
    // let index = 10;

    // let element = a[index];

    // println!("The value of element is: {}", element);
}