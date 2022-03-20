fn main() {
    let mut number = 5;

    if number < 4 {
        println!("condition true");
    } else {
        println!("condition false");
    }
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("condition {} value {} ", condition, number);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // 跳出内部循环 执行 count += 1
            }
            if count == 2 {
                break 'counting_up; // 跳出外部循环 执行打印 count值
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    res_calc()
}

fn res_calc() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 表达式会返回值 res = 20
        }
    };
		println!("the value of result is {} ", result);
		
		loop_array()
}

fn loop_array() {
    let a = [10, 20, 30, 40];
    let mut index = 0;

    while index < 4{
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    let b = [10, 20, 30, 40, 50];
    for element in b.iter() {
        println!("the value is {} ", element);
		}
		
		for number in (1..4).rev() {
			println!("{}", number);
		}
		println!("GG LOOP");
}
