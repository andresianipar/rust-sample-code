// Variables and Mutability
pub fn f1_0() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// Shadowing
pub fn f1_2() {
    let x: i32 = 5;

    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces: &str = " ";

    println!("The value of spaces is: {spaces}");

    let spaces: usize = spaces.len();

    println!("The value of spaces is: {spaces}");
}
