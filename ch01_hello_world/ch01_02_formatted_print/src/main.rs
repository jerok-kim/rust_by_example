fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");

    println!("Base 10:               {}", 69420);    // 69420
    println!("Base 2 (binary):       {:b}", 69420);    // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420);    // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420);    // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420);    // 10F2C

    println!("{number:>5}", number = 1);
    println!("{number:<5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);
    println!("This is struct `{:?}` won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    /// `std::fmt` contains many `traits` which govern the display of text. The base form of two important ones are listed below:
    /// 
    /// - `fmt::Debug`: Uses the `{:?}` marker. Format text for debugging purposes.
    /// - `fmt::Display`: Uses the `{}` marker. Format text in a more elegant, user friendly fashion.
    /// 
    /// Here, we used `fmt::Display` because the std library provides implementations for these types. To print text for custom types, more steps are required.
    /// 
    /// Implementing the `fmt::Display` trait automatically implements the `ToString` trait which allows us to `convert` the type to `String.
}