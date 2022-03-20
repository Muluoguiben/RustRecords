fn main() {
    another_function(5, 'h');

    let x = five();

    let y = {
        let x = 3; // statement
        x + 1 // expression
    };

    let z = plus_one(5);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32, _unit_label: char) {
    println!("Another function value x is {} ", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
