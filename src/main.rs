#![allow(unused_variables, dead_code)]

use std::fmt;
use crate::Cars::{Ford, Hyundai, Kia, Toyota, Volkswagen};

// Aliases for common primitive types

// i = signed
// u = unsigned

type Byte = u8;
type SByte = i8;
type Double = f64;
type Float = f32;
type Int = i32;
type Uint = u32;
type Long = i64;
type Ulong = u64;
type Short = i16;

// Prints Number
fn greet_number(x: Long) {
    println!("Hello to number {}", x);
}

// String
fn greet_string(x: String) {
    println!("Hello to string {}", x);
}

// String Slice
fn greet_str(x: &str) {
    println!("Hello to string {}", x);
}

fn greet_format(x: &str) -> String {
    return format!("Hello to string {}", x);
}

// All the primitive types in rust
struct TypeExample {
    boolean: bool,
    byte: Byte,
    // Unsigned 8-bit integer
    s_byte: SByte,
    // Signed 8-bit integer
    char: char,
    double: Double,
    // no built in decimal type
    float: Float,
    int: Int,
    uint: Uint,
    long: Long,
    ulong: Ulong,
    short: Short,
    string: String,
}

struct Simple {
    id: Int,
    name: String,
}

impl fmt::Display for Simple {
    fn fmt(&self, mutable_formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(mutable_formatter, "Simple {{ id: {}, name: {}}}", self.id, self.name)
    }
}

struct PassingString {
    value: String,
}

impl PassingString {
    fn value(&self) -> &str {
        &self.value
    }

    fn clone(&self) -> String {
        self.value.clone()
    }
}

enum Cars {
    Volkswagen,
    Ford,
    Toyota,
    Kia,
    Hyundai,
}


fn main() {
    greet_number(100);

    // "String" lives in heap
    let my_greeting: String = "World".to_string();
    greet_string(my_greeting);

    // using String from
    let alternative_string = String::from("this works too");
    println!("{}", alternative_string.as_str());

    // &str refer to String slice
    let my_greeting_str: &str = "Hello World";
    greet_str(my_greeting_str);

    // implicit &str literal
    let test_string = "Hello";
    println!("{}", test_string);

    // &str refer to String slice
    let result: String = greet_format(my_greeting_str);
    println!("result: {}", result);

    // Print out struct
    let simple = Simple { name: "Hello".to_string(), id: 10 };
    println!("simple: {}", simple);

    let car = Volkswagen;

    match car {
        Volkswagen => { println!("Volkswagen"); }
        Ford => { println!("Ford"); }
        Toyota => { println!("Toyota"); }
        Kia => { println!("Kia"); }
        Hyundai => { println!("Hyundai"); }
    }

    let passing_string = PassingString { value: String::from("Hello") };
    // pass as reference
    println!("{}", passing_string.value());

    // pass as copy / clone
    let clone_string: String = passing_string.clone();

    println!("{}", clone_string);

    // Fixed sized array with explicit type
    let how_many_cars_manufactures: [Cars; 5] = [Volkswagen, Ford, Toyota, Kia, Hyundai];

    // implicit array literal
    let how_many_car_brands = [Volkswagen, Ford, Toyota, Kia, Hyundai];
}

