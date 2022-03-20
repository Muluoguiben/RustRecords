use std::io;

fn main() {
    let x = 5;
    println!("The value of x is {} ", x);
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of inner x is {} ", x);
    }

    println!("The value of x is {} ", x);

    {
        let spaces = "abcdefg";
        let spaces = spaces.len();
        println!("The len of spaces is {} ", spaces)
    }

    {
        // tuple
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;
        println!("nums are {}, {}, {} ", five_hundred, six_point_four, one);
    }

    {
        //array
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!(
            "The value of the element at index {} is: {}",
            index, element
        );
    }
}
