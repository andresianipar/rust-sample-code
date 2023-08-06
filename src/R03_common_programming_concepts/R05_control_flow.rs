// if Expressions > Using if in a let Statement
pub fn f5_1_2() {
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // let number: i32 = if condition { 5 } else { "6" };

    // println!("The value of number is: {number}");
}

// Repetition with Loops > Returning Values from Loops
pub fn f5_2_2() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Repetition with Loops > Loop Labels to Disambiguate Between Multiple Loops
pub fn f5_2_3() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// Repetition with Loops > Conditional Loops with while
pub fn f5_2_4() {
    let mut number: i32 = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// Repetition with Loops > Looping Through a Collection with for
pub fn f5_2_5() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    // let mut index: usize = 0;

    // while index < a.len() {
    // println!("The value is: {}", a[index]);
    //
    // index += 1;
    // }

    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
