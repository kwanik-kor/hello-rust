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
}
