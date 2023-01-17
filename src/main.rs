use std::io;

fn main() {
    // 1. Mutable Variants
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    // 2. Constants
    const MAX_POINTS: u32 = 100_000;
    println!("constant : {}", MAX_POINTS);

    // 3. Shadowing
    let spaces = "     ";
    println!("Spaces: {}", spaces);
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    // 4. Data types
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    // destructing
    let (x, y, z) = tuple;
    println!("The value of y is : {}", y);
    println!("The value of z is : {}", tuple.2);

    // 5. Functions
    another_function(5);

    // 6. Statements & Expression
    check_expressions();

    // 7. Function with return
    println!("The value of function is : {}", function_with_return());

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    // 8. Control flow
    check_control_flow();
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn check_expressions() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is : {}", y);
}

fn function_with_return() -> i32 {
    10
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn check_control_flow() {
    // if conditions
    let number = 3;

    if number < 5 {
        println!("condition was true!");
    } else if number > 7 {
        println!("condition was false!");
    } else {
        println!("condition is available");
    }

    let other_number = if number == 3 {
        5
    } else {
        6
    };
    println!("other number is : {}", other_number);

    // repetition

    // loop {
    //     println!("again!");
    // }


}