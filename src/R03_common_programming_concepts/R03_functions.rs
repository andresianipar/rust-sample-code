// Statements and Expressions
pub fn f3_1_2() {
    let y: i32 = 6;

    // let x = (let y = 5);

    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Functions with Return Values
pub fn f3_1_3() {
    let x: i32 = five();

    println!("The value of x is: {x}");

    let x: i32 = plus_one(5);

    println!("The value of x is: {x}");
}
