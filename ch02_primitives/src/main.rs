/// Rust provides access to a wide variety of `primitives`. A sample includes:
///
/// ## Scalar Types
///
/// - signed integers: `i8`, `i16`, `i32`, `i64`, `i128` and `isize`(pointer size)
/// - unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`(pointer size)
/// - floating point: `f32`, `f64`
/// - `char` Unicode scalar values like `'a'`, `'α'` and `'∞'`(4 bytes each)
/// - `bool` either `true` or `false`
/// - and the unit type `()`, whose only possible value is an empty tuple: `()`
///
/// Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.
///
/// ## Compound Types
///
/// - arrays like `[1, 2, 3]`
/// - tuples like `(1, true)`
///
/// Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to `i32` and floats to `f64`. Note that Rust can also infer types from context.

fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;     // Regular annotation
    let an_integer = 5i32;  // Suffix annotation

    let default_float = 3.0;    // `f64`
    let default_integer = 7;    // `i32`

    let mut inferred_type = 12;     // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    let mut mutable = 12;   // Mutable `i32`
    mutable = 21;
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
}