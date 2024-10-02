fn main() {
    println!("Hello, world!");

    // Declare the variable using `let` and add a semicolon.
    let add_result = add(2, 4);
    
    // Correct the `println!` syntax with proper formatting for variables.
    println!("The result is: {}", add_result);

    let sub_result: i32 = sub(2,4);
    println!("The result is: {}", sub_result);


    let mul_result :i32 = mul(2,4);
    println!("The result is: {}",mul_result);

    let div_result :i32 = div(4,4);
    println!("The result is : {}",div_result);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x:i32, y:i32) -> i32 {
    x - y
}

fn mul(x:i32 , y:i32) -> i32 {
    x * y
}

fn div(x:i32, y: i32) -> i32 {
    x / y
}