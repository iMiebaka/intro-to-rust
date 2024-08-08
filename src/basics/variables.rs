
use std::mem;

// `fn` <function_name> ( <input params> ) -> <return_type> { <body> }
pub fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to learn Rust!"
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn string_expresso() {
    let message = String::from("Welcome Miebaka");
    println!("{}", message);
}

pub fn let_sum_number(a: i8, b: i8) -> i8 {
    a + b
}

pub fn var_mutation() {
    let _immutable_binding: u32 = 1;
    let mut mutable_binding: i32 = 1;
    let _big = 100; //This will not come with warnings
    println!("We are bidding here ===========");
    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += -1;

    println!("After mutation: {}", mutable_binding);
    println!("Ending Binding ===========")

    // Error! Cannot assign a new value to an immutable variable
    // _immutable_binding += 1; //❌❌❌
}

// fn data_type_overview() {
// // Variables can be type annotated.
// let logical: bool = true;

// let a_float: f64 = 1.0;  // Regular annotation
// let an_integer   = 5i32; // Suffix annotation

// // Or a default will be used.
// let default_float   = 3.0; // `f64`
// let default_integer = 7;   // `i32`

// // A type can also be inferred from context.
// let mut inferred_type = 12; // Type i64 is inferred from another line.
// inferred_type = 4294967296i64;

// // A mutable variable's value can be changed.
// let mut mutable = 12; // Mutable `i32`
// mutable = 21;

// // Error! The type of a variable can't be changed.
// mutable = true; //❌

// // Variables can be overwritten with shadowing.
// let mutable = true;
// }

pub fn literal_operators() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    println!("1 - 2 = {}", 3i32 + 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000);
}

pub fn tups() {
    let long_tuple = (
        1u8,
        2u16,
        3u32,
        4u64,
        -1i8,
        -2i16,
        -3i32,
        -4i64,
        0.1f32,
        0.2f64,
        'a',
        true,
        (1, 100),
    );
    // println!("{:?}", long_tuple); //This will not work because of the nested tuple
    println!("{:?}", long_tuple.12);
}

fn analyze_slice(slice: &[u32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn arrays() {
    let xs: [u32; 5] = [1, 2, 3, 4, 5];
    println!("{}", xs[0]);
    println!("{}", xs.len());
    println!("{}", mem::size_of_val(&xs));
    analyze_slice(&xs);


    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
}
