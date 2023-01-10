/// All types which want to use `std::fmt` formatting `traits` require an implementation to be printable. Automatic implementations are only provided for types such as in the `std` library. All others must be manually implemented somehow.
///
/// The `fmt::Debug` `trait` makes this very straightforward. All types can `derive`(automatically create) the `fmt::Debug` implementation. This is not true for `fmt::Display` which must be manually implemented.

struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

/// All `std` library types are automatically printable with `{:?}` too:
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor = "actor's");

    println!("Now {:?} will print!", Structure(3));

    // println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    
    println!("{:#?}", peter);
}