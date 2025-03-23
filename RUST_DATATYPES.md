# Rust Data Types: From Basics to Advanced

This guide explains Rust's data types system in detail, from basic primitives to advanced type concepts. It's designed to help beginners understand how Rust handles data and gradually introduces more complex type concepts.

## Table of Contents

1. [Primitive Types](#primitive-types)
2. [Compound Types](#compound-types)
3. [Type Inference and Explicit Typing](#type-inference-and-explicit-typing)
4. [Custom Types with `struct` and `enum`](#custom-types-with-struct-and-enum)
5. [Advanced Type Features](#advanced-type-features)
6. [Generic Types](#generic-types)
7. [Trait Objects and Dynamic Dispatch](#trait-objects-and-dynamic-dispatch)
8. [Common Collections](#common-collections)

## Primitive Types

Rust has several built-in primitive types:

### Integer Types

Integers are numbers without fractional components. Rust provides signed and unsigned integers of various sizes:

| Type  | Size (bits) | Range                                        | Default Value |
|-------|------------|----------------------------------------------|--------------|
| i8    | 8          | -128 to 127                                  | 0            |
| i16   | 16         | -32,768 to 32,767                            | 0            |
| i32   | 32         | -2,147,483,648 to 2,147,483,647              | 0            |
| i64   | 64         | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 | 0 |
| i128  | 128        | -(2^127) to 2^127 - 1                        | 0            |
| isize | arch-dependent | depends on target architecture            | 0            |
| u8    | 8          | 0 to 255                                     | 0            |
| u16   | 16         | 0 to 65,535                                  | 0            |
| u32   | 32         | 0 to 4,294,967,295                           | 0            |
| u64   | 64         | 0 to 18,446,744,073,709,551,615              | 0            |
| u128  | 128        | 0 to 2^128 - 1                               | 0            |
| usize | arch-dependent | 0 to architecture maximum                 | 0            |

```rust
// Declaring integer variables
let a: i32 = 42;            // Explicit typing
let b = 42;                 // Implicitly i32 (default integer type)
let c = 42_u8;              // Type suffix syntax
let d = 0xff;               // Hexadecimal
let e = 0o77;               // Octal  
let f = 0b1111_0000;        // Binary (underscores for readability)
```

### Floating-Point Types

Floating-point numbers are numbers with decimal points:

| Type  | Size (bits) | Precision           | Default Value |
|-------|------------|---------------------|--------------|
| f32   | 32         | Single precision    | 0.0          |
| f64   | 64         | Double precision    | 0.0          |

```rust
// Declaring floating-point variables
let x: f64 = 2.71828;       // Explicit typing
let y = 3.14159;            // Implicitly f64 (default float type)
let z = 1.0e10;             // Scientific notation
```

### Boolean Type

The boolean type has two possible values: `true` and `false`.

| Type  | Size (bits) | Values             | Default Value |
|-------|------------|-------------------|--------------|
| bool  | 1          | true or false     | false        |

```rust
let t: bool = true;
let f = false;              // Type inference
```

### Character Type

Rust's `char` type represents a Unicode scalar value, which means it can represent far more than just ASCII:

| Type  | Size (bits) | Range               | Default Value |
|-------|------------|---------------------|--------------|
| char  | 32         | Unicode scalar values | '\0'        |

```rust
let c = 'z';
let z: char = 'ℤ';         // Unicode character
let heart_eyed_cat = '😻';  // Emoji
```

### Unit Type

Rust has a special type called the unit type `()`, which has exactly one value, also written `()`:

```rust
// Functions that don't return a value implicitly return the unit type
fn do_something() {
    // The return type is ()
}

// Explicitly returning unit
fn do_something_else() -> () {
    // Do stuff
    ()  // Return unit
}
```

## Compound Types

Rust has two primitive compound types:

### Tuples

Tuples are fixed-size collections of values of different types:

```rust
// Default value: Each element takes its type's default value
let tup: (i32, f64, u8) = (500, 6.4, 1);  

// Destructuring
let (x, y, z) = tup;

// Accessing by index
let five_hundred = tup.0;
let six_point_four = tup.1;
```

### Arrays

Arrays are fixed-size collections of values of the same type:

```rust
// An array of 5 integers
// Default value: [0; 5] for an i32 array
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Array with repeated values
let zeros = [0; 10];  // Creates [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

// Accessing by index (zero-based)
let first = a[0];
let second = a[1];
```

## Type Inference and Explicit Typing

Rust has a strong, static type system with type inference:

```rust
let x = 42;        // Rust infers x is i32
let y = 3.14;      // Rust infers y is f64

// Explicit typing
let z: u16 = 100;
```

### Type Conversions

Rust requires explicit conversions between types:

```rust
let a: i32 = 10;
let b: u16 = a as u16;  // Explicit cast

// Numeric conversions with potential precision loss
let floating = 3.14;
let integer = floating as i32;  // Truncates to 3

// String conversions
let num_str = "42";
let num = num_str.parse::<i32>().expect("Not a valid number");
```

## Custom Types with `struct` and `enum`

### Structs

Structs allow you to create custom data types by grouping related values:

```rust
// Define a struct
struct Person {
    name: String,
    age: u8,
    email: String,
}

// Default value: None (structs don't have default values unless implemented)
// Create an instance
let alice = Person {
    name: String::from("Alice"),
    age: 30,
    email: String::from("alice@example.com"),
};

// Accessing fields
println!("Name: {}", alice.name);
```

#### Tuple Structs

```rust
struct Point(i32, i32, i32);

let origin = Point(0, 0, 0);
let x = origin.0;  // Access like a tuple
```

#### Unit-Like Structs

```rust
struct AlwaysEqual;  // No fields

// Default value: AlwaysEqual
let subject = AlwaysEqual;
```

### Enums

Enums allow you to define a type that can be one of several variants:

```rust
// Simple enum
enum Direction {
    North,
    South,
    East,
    West,
}

// Default value: None (enums don't have default values unless implemented)
let player_direction = Direction::North;

// Enum with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("Hello"));
```

#### The `Option` Enum

Rust's `Option` type is a standard enum for values that might be absent:

```rust
// Option is defined as:
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

#### The `Result` Enum

`Result` is used for operations that might fail:

```rust
// Result is defined as:
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Example with file opening
use std::fs::File;

fn open_file() {
    let f = File::open("hello.txt");
    
    match f {
        Ok(file) => println!("File opened: {:?}", file),
        Err(error) => println!("Problem opening file: {:?}", error),
    }
}
```

## Advanced Type Features

### Type Aliases

Create an alias (another name) for an existing type:

```rust
type Kilometers = i32;

let distance: Kilometers = 5;
```

### Never Type (`!`)

The never type is used for functions that never return:

```rust
fn never_returns() -> ! {
    panic!("This function never returns!");
}

// Also used in infinite loops
fn forever() -> ! {
    loop {
        // code that runs forever
    }
}
```

### Sized and Dynamically Sized Types

Most types in Rust have a known size at compile time (they implement the `Sized` trait):

```rust
// These all have known sizes
let i: i32;            // 4 bytes
let f: f64;            // 8 bytes
let b: bool;           // 1 byte
let c: char;           // 4 bytes
let t: (i32, f64);     // 12 bytes (4 + 8)

// str is dynamically sized, so we use &str (a reference)
let s: &str = "Hello"; // Size not known at compile time
```

## Generic Types

Rust allows you to define types that can work with different underlying types:

```rust
// Generic struct
struct Container<T> {
    item: T,
}

let int_container = Container { item: 42 };
let string_container = Container { item: String::from("Hello") };

// Generic function
fn first<T>(list: &[T]) -> &T {
    &list[0]
}
```

### Phantom Types

Types that use a generic parameter without actually using the type in its fields:

```rust
use std::marker::PhantomData;

struct Phantom<T> {
    _marker: PhantomData<T>,
    // No actual T value stored
}
```

## Trait Objects and Dynamic Dispatch

Trait objects enable polymorphism in Rust:

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a square with side {}", self.side);
    }
}

// Using trait objects for dynamic dispatch
fn draw_shapes(shapes: &[&dyn Draw]) {
    for shape in shapes {
        shape.draw();
    }
}
```

## Common Collections

Standard library collections with dynamic sizes:

### Vec<T>

A growable array type:

```rust
// Default value: Vec::new() (empty vector)
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);

// Using the vec! macro
let more_numbers = vec![1, 2, 3, 4, 5];

// Accessing elements
let third = &more_numbers[2];
```

### String

A growable, mutable, UTF-8 encoded string type:

```rust
// Default value: String::new() (empty string)
let mut greeting = String::new();
greeting.push_str("Hello, ");
greeting.push_str("world!");

// From a string literal
let s = String::from("Initial content");

// String concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 is moved here and can no longer be used
```

### HashMap<K, V>

A hash map collection for storing key-value pairs:

```rust
use std::collections::HashMap;

// Default value: HashMap::new() (empty hash map)
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Accessing values
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

## Type Conversions and Coercions

### The `From` and `Into` Traits

```rust
let my_str = "hello";
let my_string = String::from(my_str);  // Using From

let num = 5;
let into_string: String = num.into();  // Using Into
```

### Deref Coercions

```rust
let s = String::from("Hello");
let s_slice: &str = &s;  // String gets coerced to &str
```

### Type Casting with `as`

```rust
let a = 15;      // i32
let b = a as u8; // Cast to u8
```

## Summary

Rust's type system is designed to be both safe and flexible. It provides primitive types like integers and floats, compound types like tuples and arrays, and advanced features like generics and trait objects. Understanding the type system is fundamental to writing effective Rust code.

For more information, refer to:

1. [The Rust Book - Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
2. [The Rust Book - Using Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
3. [The Rust Book - Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
4. [The Rust Book - Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
