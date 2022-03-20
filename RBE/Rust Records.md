Rust Records

### 11.28

#### 宏：

宏看起来和函数很像，只不过名称末尾有一个感叹号 `!` 。宏并不产 生函数调用，而是**展开成源码**，并和程序的其余部分一起被编译。

 Rust 的宏会**展开为抽象语法树**（AST，abstract syntax tree），而不是像字符串预处理那样直接替换成代码，这样就**不会产生无法预料的优先权** 错误。

```rust
// 这是一个简单的宏，名为 `say_hello`。
macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数。
    () => (
        // 此宏将会展开成这个代码块里面的内容。
        println!("Hello!");
    )
}

fn main() {
    // 这个调用将会展开成 `println("Hello");`!
    say_hello!()
}

Hello！
```

为什么宏是有用的？

1. 不写重复代码（DRY，Don't repeat yourself.）。很多时候你需要在一些地方针对不同 的类型实现类似的功能，这时常常可以使用宏来避免重复代码。
2. 领域专用语言（DSL，domain-specific language）。宏允许你为特定的目的创造特定的 语法（稍后详述）。
3. 可变接口（variadic interface）。有时你需要能够接受不定数目参数的接口，比如 `println!`，根据格式化字符串的不同，它需要接受任意多的参数。



#### 格式化输出：

- `format!`：将格式化文本写到[`字符串`](https://rustwiki.org/zh-CN/rust-by-example/std/str.html)（String）。（译注：`字符串`是返 回值不是参数。）
- `print!`：与 `format!` 类似，但将文本输出到控制台（io::stdout）。
- `println!`: 与 `print!` 类似，但输出结果追加一个换行符。
- `eprint!`：与 `print!` 类似，但将文本输出到标准错误（io::stderr）。
- `eprintln!`：与 `eprint!` 类似，但输出结果追加一个换行符。

[`std::fmt`](https://doc.rust-lang.org/std/fmt/) 包含多种 [`traits`](https://rustwiki.org/zh-CN/rust-by-example/trait.html)（trait 有 “特征，特性” 等意思） 来控制文字显示，其中重要的两种 trait 的基本形式如下：

- `fmt::Debug`：使用 `{:?}` 标记。格式化文本以供调试使用。
- `fmt::Display`：使用 `{}` 标记。以更优雅和友好的风格来格式化文本。

```rust
fn main() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
	println!("{} days", 31);
    // 31 days
	// 不加ln 只将文本输出到控制台中
	print!("{}", 31);
	// 31
    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
	// Alice, this is Bob. Bob, this is Alice
    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
	// the quick brown fox jumps over the lazy dog
    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
	//1 of 10 people know binary, the other half don't
    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number=1, width=6);
	//      1
    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number=01, width=6);
	// 000001
    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond","James");
	// My name is Bond, James Bond
    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
		struct Structure(i32);
		
	println!("the value of Pi {}", Pi = 3.1415926)
	// the value of Pi 3.1415926
    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
}

```



### 11.29

使用 Rust 迈出第一步 https://docs.microsoft.com/zh-cn/learn/paths/rust-first-steps/

#### Rust优点：

Rust 还具有以下优点，非常适合各种应用程序：

- 类型安全：编译器可确保不会将任何操作应用于错误类型的变量。
- 内存安全：Rust 指针（称为“引用”）始终引用有效的内存。
- 无数据争用：Rust 的 borrow 检查器通过确保程序的多个部分不能同时更改同一值来保证线程安全。
- 零成本抽象：Rust 允许使用高级别概念，例如迭代、接口和函数编程，将性能成本控制在最低，甚至不会产生成本。 这些抽象的性能与手工编写的底层代码一样出色。
- 最小运行时：Rust 具有非常小的可选运行时。 为了有效地管理内存，此语言也不具有垃圾回收器。 在这一点上，Rust 非常类似于 C 和 C++ 之类的语言。
- 面向裸机：Rust 可以用于嵌入式和“裸机”编程，因此适合用于编写操作系统内核或设备驱动程序。



#### Crates(箱)

有数以万计的库和第三方箱可用于 Rust 程序，其中大部分可以通过 Rust 的第三方箱存储库 [crates.io](https://crates.io/) 进行访问。 

- \- Rust 标准库。 在 Rust 练习中，你将会注意到以下模块：
  - std::collections - 集合类型的定义，如 `HashMap`。
  - std::env - 用于处理环境的函数。
  - std::fmt - 控制输出格式的功能。
  - std::fs - 用于处理文件系统的功能。
  - std::io - 用于处理输入/输出的定义和功能。
  - std::path - 支持处理文件系统路径数据的定义和功能。
- [structopt](https://crates.io/crates/structopt) - 用于轻松分析命令行参数的第三方 crate。
- [chrono](https://crates.io/crates/chrono) - 用于处理日期和时间数据的第三方箱。
- [regex](https://crates.io/crates/regex) - 用于处理正则表达式的第三方箱。
- [serde](https://crates.io/crates/serde) - 适用于 Rust 数据结构的序列化和反序列化操作的第三方箱。

默认情况下，`std` 库适用于所有 Rust 箱。 若要访问箱或库中的可重复使用代码，请使用 `use` 关键字。 通过 `use` 关键字，箱或库中的代码就会“进入范围”，这样你就可以访问程序中的定义和功能。 

标准库是在路径 `std` 的 `use` 语句中访问的，如 `use std::fmt` 所示。 

其他箱或库是通过其名称访问的，例如 `use regex::Regex`。

​	

#### Cargo(管理依赖)

Cargo 可以为你做许多事情，包括：

- 使用 `cargo new` 命令创建新的项目模板。
- 使用 `cargo build` 编译项目。
- 使用 `cargo run` 命令编译并运行项目。
- 使用 `cargo test` 命令测试项目。
- 使用 `cargo check` 命令检查项目类型。
- 使用 `cargo doc` 命令编译项目的文档。
- 使用 `cargo publish` 命令将库发布到 crates.io。
- 通过将箱的名称添加到 Cargo.toml 文件来将依赖箱添加到项目。



#### Struct(结构)

1. 结构是多个其他类型的组合体
2. 结构中的元素称为字段
3. 结构中的字段可以具有不同的数据类型
4. 结构类型的一个显著好处是，可以命名每个字段，以便清楚展示相应值的含义



Rust 支持三种结构类型：经典结构、元组结构和单元结构。 这些结构类型支持使用各种方式对数据进行分组和处理。

- “经典 [C 结构](https://wikipedia.org/wiki/Struct_(C_programming_language))”最为常用。 结构中的每个字段都具有名称和数据类型。 定义经典结构后，可以使用语法 `<struct>.<field>` 访问结构中的字段。
- 元组结构类似于经典结构，但字段没有名称。 要访问元组结构中的字段，请使用索引元组时所用的语法：`<tuple>.<index>`。 与元组一样，元组结构中的索引值从 0 开始。
- “单元结构”最常用作标记

```rust
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Unit struct
struct Unit;
```

##### 实例化结构

定义结构类型后，可以通过创建该类型的实例并为每个字段指定值来使用该结构。 设置字段值时，无需按照字段的定义顺序对其进行指定。

以下示例使用为 Student 和 Grades 结构类型创建的定义。

```rust
// Classic struct with named fields
struct Student {
    name: String,
    level: u8,
    remote: bool
}

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

fn main() {
    // Instantiate a classic struct, specify the fields in random order
    let student_1 = Student {
        name: String::from("Constance Sharma"),
        remote: true,
        level: 2
    };

    // Instantiate a tuple struct, pass the values in the same order as the types are defined
    let grades_1 = Grades('A', 'A', 'B', 'A', 3.75);

    // Instantiate another classic struct, specify the field values in order
    let student_2 = Student {
        name: String::from("Dyson Tan"),
        level: 5,
        remote: false
    };

    // Instantiate another tuple struct, pass the values in the same order as defined
    let grades_2 = Grades('B', 'A', 'A', 'C', 3.25);

    // Show the student information
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
             student_1.name, student_1.level, student_1.remote, grades_1.0, grades_1.1, grades_1.2, grades_1.3, grades_1.4);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
             student_2.name, student_2.level, student_2.remote, grades_2.0, grades_2.1, grades_2.2, grades_2.3, grades_2.4);
    
}

// Constance Sharma, level 2. Remote: true. Grades: A, A, B, A. Average: 3.75
// Dyson Tan, level 5. Remote: false. Grades: B, A, A, C. Average: 3.25
```

#### 

#### Enum(枚举)(代数数据类型)

 枚举中的每个变体都是独立的，可存储**不同数量**和**类型**的值。

```rust
enum WebEvent {
    // An enum variant can be like a unit struct without fields or data types
    WELoad,
    // An enum variant can be like a tuple struct with data types but no named fields
    WEKeys(String, char),
    // An enum variant can be like a classic struct with named fields and their data types
    WEClick { x: i64, y: i64 }
}
```

示例中的枚举具有三种不同类型的变体：

- `WELoad` 没有关联的数据类型或数据。
- `WEKeys` 具有两个数据类型分别为 `String` 和 `char` 的字段。
- `WEMClick` 包含命名字段为 `x` 和 `y` 以及字段的数据类型为 `i64` 的匿名结构。

我们采用与定义不同结构类型类似的方式定义包含各种变体的枚举。 

**所有变体都归为同一个 `WebEvent` 枚举类型**。 

**任何使用 `WebEvent` 枚举变体的函数都必须接受枚举中的所有变体**。 不存在只接受 `WEClick` 变体而不接受其他变体的函数。

 枚举中的每个变体均不是其自己的类型。



##### 使用结构定义枚举

解决枚举变体要求的一种方法是为枚举中的每个变体定义单独的结构。

然后，枚举中的每个变体都使用相应的结构。 

结构容纳的数据与相应枚举变体所容纳的数据相同。 

用户可借此定义样式单独引用每个逻辑变体。

结构被定义为容纳数据。 枚举中的变体被定义为引用结构。

```rust
// Define a tuple struct
struct KeyPress(String, char);

// Define a classic struct
struct MouseClick { x: i64, y: i64 }

// Redefine the enum variants to use the data from the new structs
// Update the page Load variant to have the boolean type
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
```



##### 实例化枚举

 为了访问枚举定义中的特定变体，我们使用带有双冒号 `::` 的语法 `<enum>::<variant>`。

`WebEvent` 枚举中的第一个变体具有一个布尔值 `WELoad(bool)`。 以类似于在上一单元中使用布尔值的方式实例化此变体：

```rust
let we_load = WebEvent::WELoad(true);
```

第二个变体包含经典结构 `WEClick(MouseClick)`。 该结构有两个命名字段（`x` 和 `y`），并且这两个字段都具有 `i64` 数据类型。 要创建此变体，首先实例化结构。 然后在调用中将结构作为参数传递以实例化变体。

```rust
// Instantiate a MouseClick struct and bind the coordinate values
let click = MouseClick { x: 100, y: 250 };

// Set the WEClick variant to use the data in the click struct
let we_click = WebEvent::WEClick(click);
```

最后一个变体包含元组 `WEKeys(KeyPress)`。 元组具有两个使用 `String` 和 `char` 数据类型的字段。 要创建此变体，首先实例化元组。 然后在调用中将元组作为参数传递以实例化变体。

```rust
// Instantiate a KeyPress tuple and bind the key values
let keys = KeyPress(String::from("Ctrl+"), 'N');
    
// Set the WEKeys variant to use the data in the keys tuple
let we_key = WebEvent::WEKeys(keys);
```

请注意，我们在这段代码 `String::from("<value>")` 中使用的是新语法。 此语法通过调用 Rust `from` 方法创建 `String` 类型的值。 

All：

```rust
// Define a tuple struct
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
struct KeyPress(String, char);

// Define a classic struct
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
struct MouseClick { 
    x: i64, 
    y: i64 
}

// Define an enum with three variants
// Set the Debug flag so we can check the enum data in the output
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress)
}

fn main() {
    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick {
        x: 100,
        y: 250
    };
    
    // Print the MouseClick coordinate values
    println!("Mouse click location: {}, {}", click.x, click.y);
    
    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    
    // Print the KeyPress values
    println!("\nKeys pressed: {}{}", keys.0, keys.1);
    
    // Instantiate the WebEvent enum variants
    // Set the boolean WELoad value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);
    
    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
}

Output:
Mouse click location: 100, 250
Keys pressed: Ctrl+N
WebEvent enum structure: 
 WELoad(
    true,
) 
 WEClick(
    MouseClick {
        x: 100,
        y: 250,
    },
) 
 WEKeys(
    KeyPress(
        "Ctrl+",
        'N',
    ),
)
```

##### 	Debug语句

代码的多个位置使用了该语句。

```rust
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
```

通过 `#[derive(Debug)]` 语法可以在**代码执行期间查看某些在标准输出中无法查看的值**。 要使用 `println!` 宏查看调试数据，请使用语法 `{:#?}` 以可读的方式格式化数据。

##### Car_Factory 例子一

```rust
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
// Corrected code: Enum definition uses commas to separate values
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

// Build a new "Car" using the values of three input arguments
// - Color of the car (String)
// - Transmission type (enum)
// - Convertible (boolean, true if the car is a convertible)
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    // Create a new "Car" instance with requested characteristics
    // - Corrected code: return a "Car" struct
    // - Bind first three fields to value of corresponding input argument
    // - Set mileage to 0
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0
    }
}

fn main() {
    // Order three cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
    
    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    
    
    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    
}

Car 1 = Red, Manual transmission, convertible: false, mileage: 0
Car 2 = Silver, Automatic transmission, convertible: true, mileage: 0
Car 3 = Yellow, SemiAuto transmission, convertible: false, mileage: 0
```

##### 发现的error

![image-20211129223054036](C:\Users\Lan\Desktop\2021 Aut\Rust\Take your first steps with Rust\problem0.png)

![image-20211129223134618](C:\Users\Lan\Desktop\2021 Aut\Rust\Take your first steps with Rust\problem1.png)

正确形式：

![image-20211129223201292](C:\Users\Lan\Desktop\2021 Aut\Rust\Take your first steps with Rust\solution1.png)

或者

![image-20211129223245227](C:\Users\Lan\Desktop\2021 Aut\Rust\Take your first steps with Rust\solution2.png)





### 11.30

#### 	Array

An array can be defined in two ways:

- A comma-separated list of values, where the length isn't specified.
- The initial value followed by a semicolon, and then the array length.

In both cases, the content is enclosed in square brackets `[]`.

```rust
// Declare array, initialize all values, compiler infers length = 7
let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  
// Declare array, first value = "0", length = 5
let bytes = [0; 5];
```

At compile time, the signature of an array is defined as `[T; size]`:

- `T` is the data type for all elements in the array.
- `size` is a nonnegative integer that represents the array length.

The signature reveals two important characteristics about arrays:

- Every element of an array has the **same data type**. The data type never changes.
- The size of an array is fixed. The length never changes.

Only one thing about an array can change over time: **the values for the elements in the array**. The data type remains constant and the number of elements (length) remains constant. Only the values can change.

```rust
fn main() {
    // Declare array, don't specify size - compiler will infer length = 7
    // Initialize array elements using comma-separated list of values
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    println!("Days of the week: {:?}", days);
      
    // Declare array, specify length = 5, specify first element value = "0"
    // Declaration initializes every array element with value = "0"
    // Short form of: let bytes = ["0", "0", "0", "0", "0"]
    let bytes = [0; 5];
    println!("Byte buffer: {:?}", bytes);
    
    // Use indexing
    // Set the first day of the week
    let first  = days[0];
    // Set the second day of the week
    let second = days[1];
    println!("First = {:?}, Second = {:?}", first, second);
    
    // Declare MUTABLE array, number of days in february changes
    let mut february = [28; 1];
    println!("February days: {:?}", february[0]);
    // Change value of element
    february[0] = 29;
    println!("February leap days: {:?}", february[0]);
        
    // Test out-of-bounds index
    // Set seventh day of week, use wrong index - should be 6
    // REMOVE COMMENT SLASH MARKS on next line to test
    // let seventh  = days[7]; // returns compiler error
}

Days of the week: ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]
Byte buffer: [0, 0, 0, 0, 0]
First = "Sunday", Second = "Monday"
February days: 28
February leap days: 29
```



#### Vector

##### 	Generic Type(泛型)

When you read code in the Rust language, you'll notice the syntax `<T>`. This syntax represents the use of a generic type `T`. We use a generic type declaration when we don't yet know the actual data type.

The generic type syntax is used to declare vectors. The syntax `<vector><T>` declares a vector type composed of a generic (not yet known) data type `T`. To actually create a vector, we use a concrete type like `<vector>u32`, a vector of type u32, or `<vector>String`, a vector of type String.

##### Define Vector

A common way to declare and initialize a vector is with the `vec!` macro. This macro also accepts the same syntax as the array constructor.

```rust
// Declare vector, initialize with three values
let three_nums = vec![15, 3, 46];
println!("Initial vector: {:?}", three_nums);  
  
// Declare vector, first value = "0", length = 5
let zeroes = vec![0; 5];
println!("Zeroes: {:?}", zeroes); 

Initial vector: [15, 3, 46]
Zeroes: [0, 0, 0, 0, 0]
```



Vectors can also be created by using the `Vec::new()` method. This method of vector creation lets us add and remove values at the end of the vector. To support this behavior, we declare the vector variable as mutable with the `mut` keyword.

```rust
// Create empty vector, declare vector mutable so it can grow and shrink
let mut fruit = Vec::new();

// Push values onto end of vector, type changes from generic `T` to String
fruit.push("Apple");
fruit.push("Banana");
fruit.push("Cherry");
println!("Fruits: {:?}", fruit);

// Fruits: ["Apple", "Banana", "Cherry"]

// Pop off value at end of vector
// Call pop() method from inside println! macro
println!("Pop off: {:?}", fruit.pop());
println!("Fruits: {:?}", fruit);

// Pop off: Some("Cherry")
// Fruits: ["Apple", "Banana"]
```

After the type of a vector is set to a concrete type, only values of that specific type can be added to the vector. If we try to add a value of a different type, the compiler returns an error.

```rust
fn main()
{
    // Declare vector with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
  
    // Declare vector of length = 5, specify first element value = "0"
    // Short form of: let zeroes = vec!["0", "0", "0", "0", "0"]
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes); 
    
    // Create empty vector, make vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push some values onto the end of the vector
    // Adding values changes the type from generic to the date type of the value: String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit); 

    // Push integer value, but vector expects String (&str) type value
    // REMOVE COMMENT SLASH MARKS to test
    // fruit.push(1); // returns error
    
    // Pop off value at end of vector
    // We can call the pop() method from inside the println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);
    
    // Declare vector with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);  
    
    // Add 5 to the value at index 1, 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec); 
    
    // Try to access the vector with an out-of-bounds index = 10
    // Program compiles, but panics and stops at the invalid expression
    let beyond = index_vec[10];
    println!("Vector: {:?}, {}", index_vec, beyond);
}

Initial vector: [15, 3, 46]
Zeroes: [0, 0, 0, 0, 0]
Fruits: ["Apple", "Banana", "Cherry"]
Pop off: Some("Cherry")
Fruits: ["Apple", "Banana"]
Vector: [15, 3, 46], three = 3
Vector: [15, 8, 46]
```

使用 `Vector.get` 方法会返回 `Option` 类型，并且永远不会 panic。

### 12.1

#### 	Car_Factory 例子二

```rust
#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
// Corrected code: "mileage" u32 field removed, "age" tuple field added
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
// Corrected code: Declare enum for Car age
enum Age {
    New,
    Used,
}

//////////////////////////////////////////////////

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the age ("New" or "Used") and miles
// Return a tuple with the arrow `->` syntax
fn car_quality(miles: u32) -> (Age, u32) {
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    // Corrected code: Define "quality"
    // - Set the value to a "New" car
    // - Set the mileage using the "miles" input argument
    let quality = (Age::New, miles);

    // Corrected code: Return tuple, no need for "return" keyword or semicolon
    quality
}

//////////////////////////////////////////////////

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // Corrected code: "mileage" is replaced with "age"
    // Corrected code: Bind "age" to tuple returned from car_quality(miles)
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

//////////////////////////////////////////////////

fn main() {
    // Create car color array
    // Corrected code: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    // Corrected code: Declare "car" as mutable "Car" struct
    // Corrected code: Declare "engine" as mutable "Transmission" enum, initialize to "Manual"
    let mut car: Car;
    let mut engine = Transmission::Manual;

    //////////////////////////////////////////////////

    // Order 3 cars, one car for each type of transmission
    // Corrected code: Index into `colors` array and vary color for the orders

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: New, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: New, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}


Car order 1: New, Hard top = true, Manual, Blue, 0 miles
Car order 2: New, Hard top = false, SemiAuto, Green, 100 miles
Car order 3: New, Hard top = true, Automatic, Red, 200 miles
```



#### If expression

Unlike most other languages, `if` blocks in Rust can also act as expressions. All execution blocks in the condition branches must return the same type for the code to compile.

```rust
let formal = true;
let greeting = if formal { // if used here as an expression
    "Good day to you."     // return a String
} else {
    "Hey!"                 // return a String
};
println!("{}", greeting)   // prints "Good day to you."
```



#### Car_Factory 例子三

```rust
#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car age
enum Age {
    New,
    Used,
}

//////////////////////////////////////////////////

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality(miles: u32) -> (Age, u32) {
    // Corrected code: Check if car has accumulated miles
    // Corrected code: Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }
    // Corrected code: Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

//////////////////////////////////////////////////

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Show details about car order
    // Corrected code: If order is for Used car, check roof type, print details
    // Corrected code: Else, order is for New car, check roof tye, print details
    // Call the `println!` macro to show the car order details
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!(
                "Preparing a used car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            );
        } else {
            println!(
                "Preparing a used car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            );
        }
    } else {
        if roof {
            println!(
                "Building a new car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            );
        } else {
            println!(
                "Building a new car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            );
        }
    }

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - Bind "age" to tuple returned from car_quality(miles)
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

//////////////////////////////////////////////////

fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}


Building a new car: Manual, Orange, Hard top, 0 miles
Preparing a used car: SemiAuto, Red, Convertible, 565 miles
Preparing a used car: Automatic, White, Hard top, 3000 miles
```



#### HashMap

定义哈希映射

以下示例定义了一个哈希映射来跟踪书评。 哈希映射键为书名，值为读者评论。

```rust
use std::collections::HashMap; // 导入标准库中的collections部分中的HashMap
let mut reviews: HashMap<String, String> = HashMap::new(); // HashMap::new()创建空的哈希映射

// 添加键值对
reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

// Look for a specific review
let book: &str = "Programming in Rust";
println!("\nReview for \'{}\': {:?}", book, reviews.get(book)); // get(key)

// Remove book review
let obsolete: &str = "Ancient Roman History";
println!("\n'{}\' removed.", obsolete);
reviews.remove(obsolete);  // remove(key)


// 如果对无效的哈希映射键使用 get 方法，则 get 方法会返回“None”。
// Confirm book review removed
println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));


Review for 'Programming in Rust': Some("Great examples.")
'Ancient Roman History' removed.
Review for 'Ancient Roman History': None
```

### 12.4

#### 	Car_Factory 例子四

orders 利用hashmap

```rust
#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {

    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }
    
    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    let mut color = order as usize;
    if color > 4 {        
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }
        
    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {

    // Initialize a hash map for the car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car struct
    // Corrected code: To create a hash map, use HashMap::new()
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();
    
    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;
        
    // Order 6 cars
    // - Increment "order" after each request
    // - Add each order <K, V> pair to "orders" hash map
    // - Corrected code: Use ".insert()" method to add each order
    // - Adjust println call to show order details from the hash map
    
    // Car order #1: Used, Hard top
    car = car_factory(order, 1000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
    
    // Car order #2: Used, Convertible
    order = order + 1;
    car = car_factory(order, 2000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #3: New, Hard top
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #4: New, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #5: Used, Hard top
    order = order + 1;
    car = car_factory(order, 3000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #6: Used, Hard top
    order = order + 1;
    car = car_factory(order, 4000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
}

Car order 1: Some(Car { color: "Blue", motor: Manual, roof: true, age: (Used, 1000) })
Car order 2: Some(Car { color: "Green", motor: SemiAuto, roof: false, age: (Used, 2000) })
Car order 3: Some(Car { color: "Red", motor: Automatic, roof: true, age: (New, 0) })
Car order 4: Some(Car { color: "Silver", motor: SemiAuto, roof: false, age: (New, 0) })
Car order 5: Some(Car { color: "Blue", motor: Manual, roof: true, age: (Used, 3000) })
Car order 6: Some(Car { color: "Green", motor: Automatic, roof: true, age: (Used, 4000) })
```

#### Loop break

```rust
let mut counter = 1;
// stop_loop is set when loop stops
let stop_loop = loop {
    counter *= 2;
    if counter > 100 {
        // Stop loop, return counter value
        break counter;
    }
};
// Loop should break when counter = 128
println!("Break the loop at counter = {}.", stop_loop);
```

`break` 关键字展示了 `loop` 表达式的一项特殊功能。 通过使用 `break` 关键字，既可以停止重复表达式主体中的操作，也可以在断点处返回一个值。

`loop` 表达式主体可以有多个断点。 如果表达式有多个断点，每个断点必须返回相同类型的值。 所有值的类型都必须为整数、字符串或布尔等。 如果断点未显式返回值，程序会将表达式结果解释为空元组 `()`。



#### 循环

iter()

```rust
let big_birds = ["ostrich", "peacock", "stork"];
for bird in big_birds.iter() {
    println!("The {} is a big bird.", bird);
}

The ostrich is a big bird.
The peacock is a big bird.
The stork is a big bird.
```



a..b

从 a 值开始, step为 1 迭代到 b, 但不使用 b

```rust
for number in 0..5 {
    println!("{}", number * 2);
}

0
2
4
6
8
```

 

```rust
fn main() {

    // Loop infinite, IF you remove the break on line 8
    loop {
        // Keep printing, printing, printing...
        println!("We loop forever!");
        // On the other hand, maybe we should stop.
        break;  // WARNING! This statement is critical!
    }

    // Loop sets return value with break point
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("\nLoop break at counter value {}.\n", stop_loop);
    
    // Loop while counter is less than 5
    counter = 0;
    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    }
    println!(); // print empty line
 
    // Loop with iterator to print array values
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
    println!(); // print empty line
    
    // Loop for a range of values
    for number in 0..5 {
        println!("{}", number * 2);
    }
}


We loop forever!

Loop break at counter value 128.

We loop a while...
We loop a while...
We loop a while...
We loop a while...
We loop a while...

The ostrich is a big bird.
The peacock is a big bird.
The stork is a big bird.

0
2
4
6
8
```

#### Car_Factory 例子五

```rust
#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {

    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }
    
    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index, reset as needed
    // Valid colors = 1, 2, 3, or 4
    // Corrected code: Replace if/else with loop to reduce color to lowest divisor of 4
    let mut color = order as usize;
    while color > 4 {
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {

    // Initialize a hash map for the car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car struct
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();
    
    // Declare a car as mutable "Car" struct
    let mut car: Car;
    
    // Order 11 cars
    // Corrected code: Use "for" loop to fulfill orders for 11 cars
    // Corrected code: "order" variable initialized and incremented in "for" loop

    // Start with zero miles
    let mut miles = 0;

    for order in 1..12 {
    
        // Call car_factory to fulfill order
        // Add order <K, V> pair to "orders" hash map
        // Call println! to show order details from the hash map        
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));
        
        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}


Car order 1: Some(Car { color: "Blue", motor: Manual, roof: true, age: (New, 0) })
Car order 2: Some(Car { color: "Green", motor: SemiAuto, roof: false, age: (Used, 700) })
Car order 3: Some(Car { color: "Red", motor: Automatic, roof: true, age: (Used, 1400) })
Car order 4: Some(Car { color: "Silver", motor: SemiAuto, roof: false, age: (Used, 2100) })
Car order 5: Some(Car { color: "Blue", motor: Manual, roof: true, age: (New, 0) })
Car order 6: Some(Car { color: "Green", motor: Automatic, roof: true, age: (Used, 700) })
Car order 7: Some(Car { color: "Red", motor: Manual, roof: true, age: (Used, 1400) })
Car order 8: Some(Car { color: "Silver", motor: SemiAuto, roof: false, age: (Used, 2100) })
Car order 9: Some(Car { color: "Blue", motor: Automatic, roof: true, age: (New, 0) })
Car order 10: Some(Car { color: "Green", motor: SemiAuto, roof: false, age: (Used, 700) })
Car order 11: Some(Car { color: "Red", motor: Manual, roof: true, age: (Used, 1400) })
```



#### [在 Rust 中处理错误](https://docs.microsoft.com/zh-cn/learn/modules/rust-error-handling/?ns-enrollment-type=LearningPath&ns-enrollment-id=learn.languages.rust-first-steps)

#### panic

可以使用 `panic!` 宏来使当前线程 panic。 它将**输出一条错误消息、清理资源，然后退出程序**。

这个简单的示例演示了如何调用 `panic!` 宏：

```rust
fn main() {
    panic!("Farewell!");
}
```

此程序将以状态代码 101 退出，并输出以下消息：

```output
thread 'main' panicked at 'Farewell!', src/main.rs:2:5
```

上述 panic 消息的最后一部分显示了发生 panic 的位置。 它发生在 src/main.rs 文件中第二行的第五个字符处。



通常情况下，如果程序进入不可恢复状态，这意味着在任何情况下都无法从错误中恢复，则应使用 `panic!`。

Rust 在执行某些操作（例如被零除或试图访问数组、矢量或哈希映射中不存在的索引）时崩溃，如以下代码所示：

```rust
let v = vec![0, 1, 2, 3];
println!("{}", v[6]); // this will cause a panic!

thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 6', src/main.rs:3:16
```



#### Option 泛型

使用 `Option` 枚举来表示缺少值的可能性。

`Option<T>` 在 Rust 代码中的使用非常广泛。 它可用于处理可能存在或可能为空的值。

Rust 中没有 null

`Option<T>` 将自身列为两个变体之一：

```rust
enum Option<T> {
    None,     // The value doesn't exist
    Some(T),  // The value exists
}
```

```rust
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

// pick the first item:
let first = fruits.get(0);
println!("{:?}", first);

// pick the third item:
let third = fruits.get(2);
println!("{:?}", third);

// pick the 99th item, which is non-existent:
let non_existent = fruits.get(99);
println!("{:?}", non_existent);

Some("banana") // Option::Some(value)
Some("coconut") // Option::Some(value)
None // Option::None
```

Person 例子

```rust
struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    // middle 逻辑处理
    if let Some(middle) = &person.middle {
        full_name.push_str(&middle);
        full_name.push_str(" ")
    }

    full_name.push_str(&person.last);
    full_name
}

fn main() {
    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}

```

#### Result

Rust 提供了用于返回和传播错误的 `Result<T, E>` 枚举。 按照惯例，`Ok(T)` 变量表示成功并包含一个值，而变量 `Err(E)` 表示错误并包含一个错误值。

`Result<T, E>` 枚举定义为：

```rust
enum Result<T, E> {
    Ok(T):  // A value T was obtained.
    Err(E): // An error of type E was encountered instead.
}
```

不同于描述缺少某个值的可能性的 `Option` 类型，`Result` 类型最适合在可能会失败时使用。

`Result` 类型还具有 `unwrap` 和 `expect` 方法，这些方法执行以下操作之一：

- 如果是这种情况，则返回 `Ok` 变量中的值。
- 如果变体是 `Err`，则导致程序 panic。

```rust
#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}

Ok(3.0)
Err(DivisionByZeroError)
Ok(0.0)
```

例子

```rust
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, IoError> {
    let mut string = String::new();

    // TODO #1: Handle this match expression.
    // --------------------------------------
    // Pass the variable to the `file` variable on success, or
    // Return from the function early if it is an error.
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    // TODO #2: Handle this error.
    // ---------------------------
    // The success path is already filled for you.
    // Return from the function early if it is an error.
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    // TODO #3: return the `string` variable as expected by this function signature.
    Ok(string)
}

fn main() {
    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}

The program found the main file.
The program reported an error for the file that doesn't exist.
```

- `open` 方法会返回 `Result<File, Error>` 枚举。 换而言之，如果未发生错误，它将返回包装在 `Ok` 变体中的文件句柄。
- 

#### Match

```rust
fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 1, 2, 3, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}

It's a delicious banana!
It's a delicious apple!
Coconuts are awesome!!!
It's a delicious orange!
There is no fruit! :(
```

请注意，当字符串值为 `"coconut"` 时，将匹配第一个 arm，然后使用它来确定执行流。

当你使用 match 表达式时，请记住以下规则：

- 按照从上到下的顺序对 `match` arm 进行评估。 必须**在一般事例之前定义具体事例**，否则它们将无法进行匹配和评估。
- `match` arm 必须涵盖输入类型可能具有的每个可能值。 如果你尝试根据非详尽模式列表进行匹配，则会出现编译器错误。

```rust
let a_number: Option<u8> = Some(7);
match a_number {
    Some(7) => println!("That's my lucky number!"),
    _ => {},
}

That's my lucky number!
```

在这种情况下，我们想忽略 `None` 变体以及与 `Some(7)` 不匹配的所有 `Some<u8>` 值。 通配符模式适用于此类情况。 你可以在所有其他模式之后添加 `_`（下划线）通配符模式，以匹配任何其他项，并使用它来满足编译器耗尽 match arm 的需求。



#### If let

```rust
let a_number: Option<u8> = Some(7);
if let Some(7) = a_number {
    println!("That's my lucky number!");
}
```

if let 表达式的好处是，当你关注的是要匹配的单个模式时，你不需要 match 表达式的所有样板代码。



#### unwrap & expect

可以尝试使用 `unwrap` 方法直接访问 `Option` 类型的内部值。 但是要小心，因为如果变体是 `None`，则此方法将会 panic。

```rust
let gift = Some("candy");
assert_eq!(gift.unwrap(), "candy");

let empty_gift: Option<&str> = None;
assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:6:27
--------------------------------------------------------------------
let a = Some("value");
assert_eq!(a.expect("fruits are healthy"), "value");

let b: Option<&str> = None;
b.expect("fruits are healthy"); // panics with `fruits are healthy`

// thread 'main' panicked at 'fruits are healthy', src/main.rs:6:7
```

因为这些函数可能会崩溃，所以不建议使用它。 请改为考虑使用下列方法之一：

- 使用模式匹配并显式处理 `None` 案例。
- 调用类似的非 panic 方法，例如 `unwrap_or`。如果变体为 `None`，则该方法会返回默认值；如果变体为 `Some(value)`，则会返回内部值。

### 12.5

#### Ownership

Whenever an object goes out of scope, it's "dropped." Dropping a variable means releasing any resources that are tied to it. For variables of files, the file ends up being closed. For variables that have allocated memory associated with them, the memory is freed.

In Rust, bindings that have things "associated" with them that they will free when the binding is dropped are said to "own" those things.

```rust
{
    let mascot = String::from("ferris");
    // mascot dropped here. The string data memory will be freed here.
}
```

In our example above, the `mascot` variable owns the String data associated with it. The `String` itself owns the heap-allocated memory that holds the characters of that string. At the end of the scope, `mascot` is "dropped", the `String` it owns is dropped, and finally the memory that `String` owns is freed.

#### Move semantics

Sometimes though we don't want the things associated with a variable to be dropped at the end of scope. Instead, we want to transfer ownership of an item from one binding to another.

```rust
{
    let mascot = String::from("ferris");
    // transfer ownership of mascot to the variable ferris.
    let ferris = mascot;
    // ferris dropped here. The string data memory will be freed here.
}
```

A key thing to understand is that once ownership is transferred, the old variable is no longer valid. In our example above, after we transfer ownership of the `String` from `mascot` to `ferris`, we can no longer use the `mascot` variable.

In Rust, "transferring ownership" is known as "moving". In other words, in the example above, the `String` value has been *moved* from `mascot` to `ferris`.

If we try to use `mascot` after the `String` has been moved from `mascot` to `ferris`, the compiler will not compile our code:

```rust
{
    let mascot = String::from("ferris");
    let ferris = mascot;
    println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
}

error[E0382]: borrow of moved value: `mascot`
 --> src/main.rs:4:20
  |
2 |     let mascot = String::from("ferris");
  |         ------ move occurs because `mascot` has type `String`, which does not implement the `Copy` trait
3 |     let ferris = mascot;
  |                  ------ value moved here
4 |     println!("{}", mascot);
  |                    ^^^^^^ value borrowed here after move

This result is known as a "use after move" compile error.
```

**In Rust, only one thing can ever *own* a piece of data at a time.**

#### Ownership in Functions

Passing something as argument to function *moves* that thing into the function.

```rust
fn process(input: String) {}

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Ownership of the string in `s` moved into `process`
    process(s); // Error! ownership already moved.
}

  error[E0382]: use of moved value: `s`
     --> src/main.rs:6:13
      |
    4 |     let s = String::from("Hello, world!");
      |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    5 |     process(s); // Transfers ownership of `s` to `process`
      |             - value moved here
    6 |     process(s); // Error! ownership already transferred.
      |             ^ value used here after move
```

In other programming languages, the `String` value of the `s` variable can be implicitly copied before being passed to our function. But in Rust, this action doesn't happen.

In Rust, ownership transfer (that is, moving) is the default behavior.

#### Copy

the `Copy` trait was mentioned. We haven't talked about traits yet, but values that implement the `Copy` trait, don't get moved but are rather copied.

The following code mirrors our broken code from above, but it compiles without issue.

```rust
fn process(input: u32) {}

fn caller() {
    let n = 1u32;
    process(n); // Ownership of the number in `n` copied into `process`
    process(n); // `n` can be used again because it wasn't moved, it was copied.
}
```

Simple types like numbers *copy* types. They implement the `Copy` trait, **which means they're copied rather than moved**. The same action occurs for most **simple types**. Copying numbers is inexpensive, so it makes sense for these values to be copied. Copying **strings or vectors or other complex types can be very expensive**, so they don't implement the `Copy` trait and are instead moved.

#### clone()

One way to work around the errors we saw above is by *explicitly* copying types before they are moved: known as cloning in Rust. A call to `.clone` will duplicate the memory and produce a new value. The new value is moved meaning the old value can still be used.

RustCopy

```rust
fn process(s: String) {}

fn main() {
    let s = String::from("Hello, world!");
    process(s.clone()); // Passing another value, cloned from `s`.
    process(s); // s was never moved and so it can still be used.
}
```

This approach can be useful, but it can make your code **slower** as every call to `clone` **makes a full copy of the data**. This method often includes memory allocations or other expensive operations. We can avoid these costs if we "borrow" values by using *references*. 

#### Borrowing references

Wouldn't it be nice to be able to allow functions and other variables to use certain data without fully owning it?

This type of functionality is available by using *references*. References allow us to "borrow" values without taking ownership of them.

```rust
let greeting = String::from("hello");
let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
println!("Greeting: {}", greeting); // We can still use `greeting`

// hello
```

In the above code, `greeting` was borrowed using the `&`. `greeting_reference` was of type `&String`. Since we only borrowed `greeting` and did not move ownership, `greeting` could still be used after we created `greeting_reference`.

#### References in functions

```rust
fn print_greeting(message: &String) {
  println!("Greeting: {}", message);
}

fn main() {
  let greeting = String::from("Hello");
  print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
  print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
}
```

Borrowing allows us to use a value without taking full ownership. However, as we'll see, borrowing a value means we can't do everything we can do with a fully owned value.



#### Mutate borrowed values

```rust
fn main() {
    let mut greeting = String::from("hello"); // value should be mutable
    change(&mut greeting);
}

fn change(text: &mut String) {
    text.push_str(", world");
}
```

With only `&` borrows, known as "immutable borrows," we can read the data but we can't change it. 

With `&mut` borrows, known as "mutable borrows," we can both read and write the data.



#### Borrowing and mutable references

Now we get to the real core of Rust's memory management story. Immutable and mutable references differ in one other way that has radical effects on how we build our Rust programs. 

When borrowing a value of any type `T`, the following rules apply:

Your code must implement *either* of the following definitions, but **not both at the same time**:

- One or more immutable references (`&T`)
- Exactly one mutable reference (`&mut T`)

The following code doesn't have allowed definitions, so the compilation fails:

```rust
fn main() {
    let mut value = String::from("hello");

    let ref1 = &mut value;
    let ref2 = &mut value;

    println!("{}, {}", ref1, ref2);
}

    error[E0499]: cannot borrow `value` as mutable more than once at a time
     --> src/main.rs:5:16
      |
    4 |     let ref1 = &mut value;
      |                ---------- first mutable borrow occurs here
    5 |     let ref2 = &mut value;
      |                ^^^^^^^^^^ second mutable borrow occurs here
    6 |
    7 |     println!("{}, {}", ref1, ref2);
      |                        ---- first borrow later used here
```



We can even try to mix **immutable references** with **mutable references**, but the compiler will still complain:

```rust
fn main() {
    let mut value = String::from("hello");

    let ref1 = &value;
    let ref2 = &mut value;

    println!("{}, {}", ref1, ref2);
}

  error[E0502]: cannot borrow `value` as mutable because it is also borrowed as immutable
     --> src/main.rs:5:16
      |
    4 |     let ref1 = &value;
      |                ------ immutable borrow occurs here
    5 |     let ref2 = &mut value;
      |                ^^^^^^^^^^ mutable borrow occurs here
    6 |
    7 |     println!("{}, {}", ref1, ref2);
      |                        ---- immutable borrow later used here
```

This restriction may seem harsh at first, but this aspect of borrowing prevents Rust code from a whole host of issues, including never having data races.



```rust
fn main() {
    let mut value = String::from("hello");

    let ref1 = &value;
    let ref2 = &value;
    

    println!("{}, {}, {}", value, ref1, ref2);
}

hello, hello, hello
```



#### Lifetimes

Consider the following snippet, which tries to use a reference whose value has gone out of scope:

```rust
fn main() {
    let x;
    {
        let y = 42;
        x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    }
    println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
}

error[E0597]: `y` does not live long enough
     --> src/main.rs:6:17
      |
    6 |             x = &y;
      |                 ^^ borrowed value does not live long enough
    7 |         }
      |         - `y` dropped here while still borrowed
    8 |         println!("x: {}", x);
      |                           - borrow later used here
```

This error occurs because a value was dropped while it was still borrowed. In this case, `y` is dropped at the end of the inner scope, but `x` borrows it until the `println` call. Because `x` is still valid for the outer scope *(because its scope is larger)*, we say that it "lives longer."

The Rust compiler can verify if the borrows are valid by using the *borrow checker*. The borrow checker compares the two lifetimes at compile time. In this scenario, `x` has a lifetime of `'a` but it refers to a value with a lifetime of `'b`. The reference subject *(`y` at lifetime `'b`)* a shorter time than the reference *(`x` at lifetime `'a`)* so the program doesn't to compile.



#### Annotating lifetimes in functions

As with types, lifetime durations are inferred by the Rust compiler.

There may be multiple lifetimes. When that occurs, annotate the lifetimes to help the compiler understand which lifetime it will use to ensure the references are valid at runtime.

For example, consider a function that takes two strings as its input parameters and returns the longest of them:

```rust
fn main() {
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);
}

fn longest_word(x: &String, y: &String) -> &String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

error[E0106]: missing lifetime specifier
     --> src/main.rs:9:38
      |
    9 | fn longest_word(x: &String, y: &String) -> &String {
      |                    ----        ----        ^ expected named lifetime parameter
      |
      = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
    help: consider introducing a named lifetime parameter
      |
    9 | fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
      |                ^^^^    ^^^^^^^        ^^^^^^^        ^^^
```

The help text says Rust can't tell whether the reference that's being returned refers to `x` or `y`. Neither can we. So, to help identify what the reference is, annotate the return type with a generic parameter to represent the lifetime.

The borrow checker can't determine if the reference will be a valid one either. It doesn't know how the input parameters' lifetime relate to the return value's lifetime. This is why we need to annotate the lifetimes manually.



We can add generic lifetime parameters to our function signature. These parameters define the relationship between the references so the borrow checker can complete its analysis:

```rust
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**Make sure to declare generic lifetime parameters inside angle brackets, and add the declaration between the parameter list and the function name.**

In the signature,the return value and all the parameter references must have the same lifetime. As such, use the same lifetime name, for example `'a`. Then, add the name to each reference in the function signature.

The important thing to keep in mind is that all parameters and the returned value will live at least as long as the lifetime associated with each of them.

The reference's lifetime that the function returns **matches the smaller of the references' lifetimes that are passed in**. As such, the code possibly includes an invalid reference and the borrow checker will disallow it.



#### Annotating lifetimes in types

Whenever a struct or enum holds a reference in one of its fields, we must annotate that type definition with the lifetime of each reference that it carries along with it.

For example, consider the following example code. We have a `text` string *(which owns its contents)* and a `Highlight` tuple struct. The struct has one field, `part`, that holds a string slice. The slice is a borrowed value from another part of our program.

```rust
#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}

Highlight("quick brown fox")
Highlight("lazy dog")
```

We place the name of the generic lifetime parameter inside angle brackets after the name of the struct. This placement so we can use the lifetime parameter in the body of the struct definition. This instance of `Highlight` can't live longer than the reference in the `part` field because of the declaration.

In the preceding code, we annotated our struct with a lifetime called `'document`. This annotation is a reminder that the `Highlight` struct can't outlive the source of the `&str` that it borrows, a supposed document.

Also, `Highlight` goes out of scope before `text` goes out of scope. This means that the `Highlight` instance is valid.



As an experiment, try to move the value held by `text` out of the scope and see what kinds of complaints the borrow checker issues:

```rust
#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: String) { }

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    erase(text);

    println!("{:?}", fox);
    println!("{:?}", dog);
}
error[E0505]: cannot move out of `text` because it is borrowed
  --> src/main.rs:11:11
   |
8  |     let fox = Highlight(&text[4..19]);
   |                          ---- borrow of `text` occurs here
...
11 |     erase(text);
   |           ^^^^ move out of `text` occurs here
12 | 
13 |     println!("{:?}", fox);
   |                      --- borrow later used here
```





### 12.6

#### 泛型(generic type)

泛型数据类型是根据其他部分未知类型定义的类型

- `Option<T>` 枚举在类型 `T` 上是泛型类型，后者是其 `Some` 变体包含的值。
- `Result<T, E>` 在其成功和失败类型上都是泛型类型，分别包含在其 `Ok` 和 `Err` 变体中。
- 矢量类型 `Vec<T>`、数组类型 `[T; n]` 以及哈希映射 `HashMap<K, V>` 在其所包含的类型上是泛型类型。

使用泛型类型时，可以指定所需操作，而不必考虑定义类型持有的内部类型。

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let boolean = Point { x: true, y: false };   // <bool, bool>
    let integer = Point { x: 1, y: 9 }; // <integer, integer>
    let float = Point { x: 1.7, y: 4.3 }; // <f64, f64>
    let string_slice = Point { x: "high", y: "low" }; // <&'static str, &'static str>
}

```



我们在两个类型之上显示一个 `Point<T, U>` 泛型，使得 `x` 和 `y` 可以是不同类型的值。

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_and_boolean = Point { x: 5, y: false };
    let float_and_string = Point { x: 1.0, y: "hey" };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let both_integer = Point { x: 10, y: 30 };
    let both_boolean = Point { x: true, y: true };
}
```

前面的所有 `Point` 类型都具有不同的具体类型。 顺序如下：

- `Point<integer, bool>`
- `Point<f64, &'static str>`
- `Point<integer, f64>`
- `Point<integer, integer>`
- `Point<bool, bool>`



#### traits

A trait is a common interface that a group of types can implement

Each trait definition is a collection of methods defined for an unknown type, usually representing a capability or behavior that its implementors can do.

To represent the concept of "having a two-dimensional area," we can define the following trait:

```rust
trait Area {
    fn area(&self) -> f64;
}
```

Here, we declare a trait by using the `trait` keyword and then the trait's name, which is `Area` in this case.

 The compiler will then check that each type implementing this trait must provide its own custom behavior for the body of the method.

```rust
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}

```

To implement a trait for a type, we use the keywords `impl Trait for Type`, 

where `Trait` is the name of the trait being implemented and `Type` is the name of the implementor struct or the enum.

Within the `impl` block, we put the method signatures that the trait definition has required, filling the method body with the specific behavior that we want the methods of the trait to have for the particular type.

#### Derive trait

This simple `Point` struct can't be compared to other `Point` instances or displayed in the terminal. Because of this difficulty, we might want to use the *derive* attribute to allow new items to automatically be generated for the struct.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!

}

error[E0277]: `Point` doesn't implement `std::fmt::Display`
      --> src/main.rs:10:20
       |
    10 |     println!("{}", p1);
       |                    ^^ `Point` cannot be formatted with the default formatter
       |
       = help: the trait `std::fmt::Display` is not implemented for `Point`
       = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
       = note: required by `std::fmt::Display::fmt`
       = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

    error[E0277]: `Point` doesn't implement `Debug`
      --> src/main.rs:11:22
       |
    11 |     println!("{:?}", p1);
       |                      ^^ `Point` cannot be formatted using `{:?}`
       |
       = help: the trait `Debug` is not implemented for `Point`
       = note: add `#[derive(Debug)]` or manually implement `Debug`
       = note: required by `std::fmt::Debug::fmt`
       = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

    error[E0369]: binary operation `==` cannot be applied to type `Point`
      --> src/main.rs:13:11
       |
    13 |     if p1 == p2 {
       |        -- ^^ -- Point
       |        |
       |        Point
       |
       = note: an implementation of `std::cmp::PartialEq` might be missing for `Point`

    error: aborting due to 3 previous errors#+end_example
```

This code fails to compile because our `Point` type doesn't implement the following traits:

- The `Debug` trait, which allows a type to be formatted by using the `{:?}` format specifier, is used in a programmer-facing, debugging context.
- The `Display` trait, which allows a type to be formatted by using the `{}` format specifier, is similar to `Debug`. But `Display` is better suited for user-facing output.
- The `PartialEq` trait, which allows implementors to be compared for equality.

the `Debug` and `PartialEq` traits can be automatically implemented for us by the Rust compiler by using the `#[derive(Trait)]` attribute, if each of its fields implements the trait:

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

Our code will still fail to compile because Rust's standard library **doesn't provide automatic implementation** for the `Display` trait, because it's meant for end users. But if we comment out that line, our code now produces this output:

```output
    not equal!
    Point { x: 1, y: 2 }
```

Nevertheless, we can implement the `Display` trait for our type by ourselves:

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

#### trait json example

```rust
trait AsJson {
    fn as_json(&self) -> String;
}

fn send_data_as_json(value: &impl AsJson) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

struct Cat {
    name: String,
    color: String,
    sharp_claws: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
            self.name, self.color, self.likes_petting
        )
    }
}

impl AsJson for Cat {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "cat", "name": "{}", "color": "{}", "sharp_claws": {} }}"#,
            self.name, self.color, self.sharp_claws
        )
    }
}

fn main() {
    let laura = Person {
        name: String::from("Laura"),
        age: 31,
        favorite_fruit: String::from("apples"),
    };

    let fido = Dog {
        name: String::from("Fido"),
        color: String::from("Black"),
        likes_petting: true,
    };

    let kitty = Cat {
        name: String::from("Kitty"),
        color: String::from("Blue"),
        sharp_claws: false,
    };

    send_data_as_json(&laura);
    send_data_as_json(&fido);

    // The Cat type does not implement the trait AsJson.
    send_data_as_json(&kitty); // uncomment this line to see the compiler error.
}

Sending JSON data to server...
-> { "type": "person", "name": "Laura", "age": 31, "favoriteFruit": "apples" }
Done!

Sending JSON data to server...
-> { "type": "dog", "name": "Fido", "color": "Black", "likesPetting": true }
Done!

Sending JSON data to server...
-> { "type": "cat", "name": "Kitty", "color": "Blue", "sharp_claws": false }
Done!
```



#### Iterator

Standard library trait

The core of that trait looks like this:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

An `Iterator` has a method, `next`, which when called returns an `Option<Item>`. The `next` method will return `Some(Item)` as long as there are elements. After they've all been exhausted, it will return `None` to indicate that iteration is finished.

Notice this definition uses some new syntax: `type Item` and `Self::Item`, which define an associated type with this trait. This definition means that every implementation of the `Iterator` traits also requires the definition of the associated `Item` type, which is used as the return type of the `next` method. In other words, the `Item` type will be the type returned from the iterator inside the `for` loop block.

counter example

```rust
struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter { count: 0, length }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    for number in Counter::new(10) {
        println!("{}", number);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);
}

1
2
3
4
5
6
7
8
9
10
```



### 12.8

#### 泛型例子

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}
fn main() {
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}

```

#### 迭代器例子

```rust
struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;
	// todo! code
    fn next(&mut self) -> Option<Self::Item> {
        // if the inner vector is empty, we are done
        if self.inner.is_empty() {
            return None;
        }

        // lets check the span of equal items
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }

        // we use the `Vec::drain` to extract items up until the cursor
        let items = self.inner.drain(0..cursor).collect();

        // return the extracted items
        Some(items)
    }
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    )
}
```

#### package, crate, module

- A package:
  - Contains functionality within one or more *crates*.
  - Includes information about how to build those *crates*. The information is in the `Cargo.toml` file.
- A crate:
  - Is a **compilation unit**, which is the **smallest amount of code** that the Rust compiler can operate on.
  - Once compiled, produces either an executable or a library.
  - Contains an implicit, unnamed top-level *module*.
- A module(mod):
  - Is a *(possibly nested)* unit of code organization inside a *crate*.
  - Can have recursive definitions that span additional modules.



#### 	mod example

```rust
mod text_processing {

    pub mod letters {
        pub fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    pub mod numbers {
        pub fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
}

fn main() {
    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
}

```

