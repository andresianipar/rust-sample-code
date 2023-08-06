use std::io::stdin;

// Scalar Types > Numeric Operations
pub fn f2_1_3() {
    // addition
    let sum: i32 = 5 + 10;

    // subtraction
    let difference: f64 = 95.5 - 4.3;

    // multiplication
    let product: i32 = 4 * 30;

    // division
    let quotient: f64 = 56.7 / 32.2;
    let truncated: i32 = -5 / 3;

    // remainder
    let remainder: i32 = 43 % 5;
}

// Scalar Types > The Boolean Type
pub fn f2_1_4() {
    let t: bool = true;

    let f: bool = false;
}

// Scalar Types > The Character Type
pub fn f2_1_5() {
    let c: char = 'z';
    let z: char = 'Z';
    let marvel: &str = "Marvel";
}

// Compound Types > The Tuple Type
pub fn f2_2_1() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x, y, z are: {x}, {y}, {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred: i32 = x.0;

    let six_point_four: f64 = x.1;

    let one: u8 = x.2;

    let unit: ();
}

// Compound Types > The Array Type
pub fn f2_2_2() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [3; 5];

    print!("{:?}\n", a);
}

// Compound Types > Accessing Array Elements
pub fn f2_2_3() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first: i32 = a[0];
    let second: i32 = a[1];

    print!("first: {}\nsecond: {}\n", first, second);
}

// Compound Types > Invalid Array Element Access
pub fn f2_2_4() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index: String = String::new();

    stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: i32 = a[index];

    println!("The value of the element at index {index} is: {element}");
}
