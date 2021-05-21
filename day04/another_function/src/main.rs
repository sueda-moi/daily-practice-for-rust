fn main() {
    println!("Hello, world!");
    another_function_one(55);
    another_function_two(5, 6);
}

fn another_function_one(x: i32) {
    println!("another function begin x is : {}", x);
}

fn another_function_two(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
