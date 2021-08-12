//3.1
fn main() {
    //immutable, mutable
    //let mut x = 5;
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    //constant 
    const MAX_POINTS: u32 = 100_000_000;
    println!("MAX POINTS : {}", MAX_POINTS);

    //shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    //shadowing 2
    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);

    //mutable variable can't shadowing
    // let mut spaces = "    "; //type already assigned
    // spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);


}
