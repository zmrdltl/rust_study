fn main() {
    let number = 3;
    //branches must have boolean expression
    //different from other language
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    //wrong branch expression
    // if number {
    //     println!("Number was three");
    // }

    //so have to change like this
    if number != 0 {
        println!("number was something other than zero");
    }


    //if else
    let num = 6;
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }

    let condition = false;
    let number2 = if condition {
        5
    } else {
        6
    };

    println!("The value of number2 is: {}", number2);

    //if else expression must have smae type
    //if you write the code like this get an error
    // let condition2 = true;

    // let number3 = if condition2 {
    //     5
    // } else {
    //     "six"
    // };

    // println!("The value of number is: {}", number3);

    //loop, while
    let mut number4 = 3;

    while number4 != 0 {
        println!("{}!", number4);

        number4 = number4 - 1;
    }

    println!("LIFTOFF!!!");

    //while in array, array start with index 0
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    //for loop
    let a1 = [10, 20, 30, 40, 50];

    for element in a1.iter() {
        println!("the value is: {}", element);
    }

    //3 , 2 , 1 count reverse with rev()
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
