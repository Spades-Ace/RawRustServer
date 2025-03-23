# Learn Rust Through a Raw HTTP Server

This document provides an in-depth explanation of both Rust basics and the HTTP server implementation in this project. It's designed to help beginners understand the code line by line and learn Rust concepts along the way.

## Table of Contents

1. [Rust Basics](#rust-basics)
2. [Understanding Rust Tools](#understanding-rust-tools)
3. [Rust Syntax Explained](#rust-syntax-explained)
4. [Line-by-Line Code Explanation](#line-by-line-code-explanation)
5. [HTTP Server Concepts](#http-server-concepts)
6. [Further Learning](#further-learning)

## Rust Basics

### What is Rust?

Rust is a systems programming language focused on safety, speed, and concurrency. It provides memory safety without using garbage collection, making it ideal for performance-critical applications.

### Key Features of Rust

- **Memory Safety**: Prevents common bugs like null pointer dereferencing and buffer overflows
- **Zero-Cost Abstractions**: High-level features with no runtime overhead
- **Fearless Concurrency**: Makes multithreaded programming safer and more approachable
- **Type System**: Strong, static typing with powerful inference

### Basic Syntax

```rust
// Variables are immutable by default
let x = 5;

// Use 'mut' to make a variable mutable
let mut y = 10;
y = 20; // This works because y is mutable

// Functions are defined with 'fn'
fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon means this is the return value
}

// Structs define custom data types
struct Person {
    name: String,
    age: u32,
}
```

## Understanding Rust Tools

### `rustc` (Rust Compiler)

`rustc` is the Rust compiler that converts your Rust source code (`.rs` files) into executable binaries.

```bash
# Compile a Rust file directly
rustc main.rs

# Run the compiled program
./main  # or .\main.exe on Windows
```

When you use `rustc` directly:

- You need to manage dependencies manually
- No standardized project structure
- Must compile each file separately

### `cargo` (Package Manager & Build Tool)

`cargo` is Rust's all-in-one package manager, build system, and project manager.

```bash
# Create a new Rust project
cargo new project_name

# Build your project
cargo build
cargo build --release  # Optimized build for production

# Run your project
cargo run

# Update dependencies
cargo update

# Run tests
cargo test
```

Key benefits of using Cargo:

- Standardized project structure
- Automated dependency management via `Cargo.toml`
- Simplified build process
- Integrated testing framework
- Publishing to crates.io (Rust's package registry)

### Crates vs. Modules

- **Crates**: Compilation units or packages of code. A crate can be a binary (executable) or a library.
  - **Binary Crates**: Have a `main()` function and produce an executable
  - **Library Crates**: Provide functionality to be used by other crates

- **Modules**: Organize and structure code within a crate for better readability and maintainability.
  - Defined with the `mod` keyword
  - Can be nested
  - Control visibility with `pub` keyword

## Rust Syntax Explained

### Common Operators and Symbols

#### `::` (Path Separator)

The double colon is a namespace operator used to access items (functions, structs, etc.) from modules, similar to `.` in other languages.

```rust
std::io::Read  // Accessing the Read trait from the io module in the std library
```

#### `:` (Type Annotation)

Used to specify the type of a variable, function parameter, or return value.

```rust
let age: u32 = 30;  // age is a 32-bit unsigned integer
fn add(a: i32, b: i32) -> i32 { ... }  // Parameters a and b are 32-bit signed integers
```

#### `&` (Reference)

Creates a reference to a value without taking ownership.

```rust
fn print_length(s: &String) {  // Takes a reference to a String
    println!("{}", s.len());
}
// After this function call, the original String is still usable
```

#### `&mut` (Mutable Reference)

Creates a mutable reference that allows modification of the referenced value.

```rust
fn add_suffix(s: &mut String) {
    s.push_str("_suffix");  // Modifies the original String
}
```

#### `match` (Pattern Matching)

A powerful control flow operator that allows you to compare a value against a series of patterns.

```rust
match number {
    0 => println!("Zero"),
    1 => println!("One"),
    _ => println!("Something else"),  // _ is a catch-all pattern
}
```

#### `=>` (Fat Arrow)

Used in match expressions and closures to separate patterns from code blocks.

```rust
// In match expressions:
match value {
    pattern => expression,
}

// In closures:
let add = |a, b| a + b;  // The expression after | | is the body
```

#### `let` (Variable Binding)

Introduces a new variable.

```rust
let x = 5;  // Immutable by default
let mut y = 10;  // Mutable variable
```

#### `mut` (Mutability Marker)

Indicates that a binding is mutable.

```rust
let mut count = 0;
count += 1;  // This works because count is mutable
```

### Common Types

#### `&str` (String Slice)

A reference to a part of a string. It's a view into string data owned by someone else.

```rust
let greeting: &str = "Hello, world!";
```

#### `String` (Owned String)

A growable, heap-allocated string type.

```rust
let mut s = String::from("Hello");
s.push_str(", world!");  // Can be modified
```

#### `Result<T, E>` (Error Handling)

An enum that represents either success (`Ok(T)`) or failure (`Err(E)`).

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Usage with match:
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

#### `'static` (Static Lifetime)

Indicates that a reference is valid for the entire program's duration.

```rust
let greeting: &'static str = "Hello, world!";
```

## Line-by-Line Code Explanation

Now, let's dive into our HTTP server implementation line by line:

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
```

- `use` brings symbols into scope (similar to import statements in other languages)
- `std` is the standard library module (built into Rust)
- `::` accesses submodules or items within modules
- `io`, `net`, and `fs` are modules within the standard library:
  - `io`: Input/output functionality
  - `net`: Network-related functionality
  - `fs`: Filesystem operations
- `{Read, Write}` imports specific traits from the `io` module
- `{TcpListener, TcpStream}` imports specific structs from the `net` module

```rust
const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const HTTP_VERSION: &str = "HTTP/1.1";
const SERVER_NAME: &str = "RustRawHTTP/1.0";
```

- `const` defines a constant (immutable value known at compile time)
- `SERVER_ADDRESS: &str` declares a constant of type string slice (`&str`)
- `&str` is a reference to a string slice (efficient, immutable)
- The constants store configuration values for the HTTP server

```rust
fn main() {
```

- `fn` defines a function
- `main()` is the entry point for Rust programs (similar to main in C/C++)
- No return type is specified, which implies the return type is `()` (unit type, similar to void)

```rust
    println!("Starting HTTP server at {}", SERVER_ADDRESS);
```

- `println!` is a macro (note the `!`), not a function
- Macros are expanded at compile time and can generate code
- `{}` is a placeholder for formatting, similar to `printf` in C

```rust
    let listener = match TcpListener::bind(SERVER_ADDRESS) {
```

- `let` introduces a new variable binding
- `TcpListener::bind(SERVER_ADDRESS)` calls the static method `bind` on the `TcpListener` struct
- `bind` returns a `Result<TcpListener, Error>` (either success or failure)
- `match` begins a pattern match expression on the Result

```rust
        Ok(listener) => {
            println!("Successfully bound to address");
            listener
        },
```

- `Ok(listener)` is a pattern that matches if binding succeeded
- `=>` separates the pattern from the code to execute if it matches
- The last expression (`listener`) is implicitly returned from this match arm
- The comma separates match arms

```rust
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return;
        }
```

- `Err(e)` matches if binding failed, capturing the error in variable `e`
- `eprintln!` prints to standard error instead of standard output
- `return` exits the function early in case of an error

```rust
    for stream in listener.incoming() {
```

- `for` begins a loop that iterates over values produced by `listener.incoming()`
- `incoming()` returns an iterator of `Result<TcpStream, Error>` for each connection attempt
- `stream` is a variable that will hold each result one at a time

```rust
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_connection(stream);
            },
```

- Another `match` expression to handle the Result
- `stream.peer_addr()` returns information about the remote peer (client)
- `unwrap()` extracts the value from a Result or Option (will panic if there's an error)
- `handle_connection(stream)` calls our function to process the connection, passing ownership of the stream

```rust
fn handle_connection(mut stream: TcpStream) {
```

- Function to handle a client connection
- `mut stream` means we receive a mutable TcpStream parameter
- `mut` is necessary because we'll be reading from the stream (which mutates its internal state)

```rust
    let mut buffer = [0; 1024];
```

- Creates a mutable buffer of 1024 bytes, all initialized to zero
- `[0; 1024]` is array syntax for "1024 elements, all with value 0"

```rust
    match stream.read(&mut buffer) {
```

- `stream.read()` attempts to read data from the connection into our buffer
- `&mut buffer` passes a mutable reference to our buffer
- The `&` creates a reference, and `mut` makes it mutable

```rust
        Ok(size) => {
```

- If reading succeeds, `size` contains the number of bytes read

```rust
            let request = String::from_utf8_lossy(&buffer[..size]);
```

- `&buffer[..size]` creates a slice of our buffer from index 0 up to `size`
- `String::from_utf8_lossy()` converts the bytes to a string, replacing invalid UTF-8 sequences
- The result is a `Cow<'_, str>` (Clone-on-write string), which behaves like a string

```rust
            let request_line = request.lines().next().unwrap_or("");
```

- `request.lines()` splits the string on newlines, returning an iterator
- `next()` gets the first line (if any)
- `unwrap_or("")` returns either the line or an empty string if there wasn't one

```rust
            let parts: Vec<&str> = request_line.split_whitespace().collect();
```

- `split_whitespace()` splits the string on whitespace, returning an iterator
- `collect()` gathers the iterator items into a collection
- `Vec<&str>` specifies that we want a vector (dynamic array) of string slices

```rust
            if parts.len() >= 2 {
                let method = parts[0];
                let path = if parts[1] == "/" { "/index.html" } else { parts[1] };
```

- Checks if we have at least the HTTP method and path
- If the path is just "/", we default to "/index.html"
- This is a conditional expression (ternary operator)

```rust
                if method == "GET" {
                    serve_file(stream, path);
                } else {
                    send_response(stream, 405, "Method Not Allowed", "Only GET method is supported");
                }
```

- Only processes GET requests, rejecting others with 405 Method Not Allowed
- Passes ownership of the stream to either function

```rust
fn serve_file(stream: TcpStream, path: &str) {
```

- Function to serve a requested file
- `path: &str` takes a borrowed string slice (doesn't take ownership)

```rust
    let file_path = format!("public{}", path);
```

- `format!` is a macro that creates a String through formatting
- Prepends "public" to the path, making "/index.html" become "public/index.html"

```rust
    match fs::read_to_string(&file_path) {
```

- `fs::read_to_string()` attempts to read a file into a String
- Returns a Result that's either the file contents or an error

```rust
fn send_response(mut stream: TcpStream, status_code: u16, status_text: &str, body: &str) {
```

- Function to send an HTTP response
- Takes multiple parameters: the stream, status code, status text, and response body
- `status_code: u16` specifies a 16-bit unsigned integer (appropriate for HTTP status codes)

```rust
    let response = format!(
        "{} {} {}\r\n\
        Server: {}\r\n\
        Content-Length: {}\r\n\
        Content-Type: {}\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        HTTP_VERSION, status_code, status_text,
        SERVER_NAME,
        content_length,
        get_content_type(body),
        body
    );
```

- Constructs the HTTP response as a formatted string
- `\r\n` represents carriage return and line feed (standard HTTP line endings)
- `\r\n\r\n` creates the blank line that separates headers from body in HTTP

```rust
    match stream.write_all(response.as_bytes()) {
```

- `response.as_bytes()` converts the String to a byte slice (`&[u8]`)
- `stream.write_all()` attempts to write all bytes to the stream

```rust
fn get_content_type(content: &str) -> &'static str {
```

- Function to determine the content type
- Returns `&'static str`, which is a string slice with a 'static lifetime (lives for the program's duration)

```rust
    if content.trim_start().starts_with("<!DOCTYPE html>") ||
       content.trim_start().starts_with("<html") {
        "text/html; charset=utf-8"
    } else {
        "text/plain; charset=utf-8"
    }
```

- Uses string methods to check if content looks like HTML
- `trim_start()` removes leading whitespace
- `starts_with()` checks if a string begins with the specified text
- If no return statement and semicolon, the last expression is the return value

## HTTP Server Concepts

Our raw HTTP server implements several key concepts:

1. **TCP Socket Communication**: The foundation of HTTP is TCP/IP networking
   - `TcpListener` binds to an address and port
   - `incoming()` accepts client connections
   - Each connection is represented by a `TcpStream`

2. **HTTP Request Parsing**: Breaking down the raw text into meaningful parts
   - First line contains method, path, and HTTP version
   - Headers follow, one per line
   - Blank line separates headers from body

3. **HTTP Response Construction**: Building a properly formatted HTTP response
   - Status line: `HTTP/1.1 200 OK`
   - Headers: Content-Type, Content-Length, etc.
   - Blank line
   - Response body

4. **File Serving**: Reading files from disk and sending them to clients
   - Map URL paths to filesystem paths
   - Read file contents
   - Determine content type
   - Send appropriate response

5. **Error Handling**: Proper handling of various error conditions
   - File not found: 404 response
   - Method not allowed: 405 response
   - Invalid request: 400 response
   - Connection errors: Logged but not fatal

## Further Learning

To deepen your understanding of Rust and HTTP servers:

1. **The Rust Book**: <https://doc.rust-lang.org/book/>
2. **Rust by Example**: <https://doc.rust-lang.org/rust-by-example/>
3. **HTTP Made Really Easy**: <https://www.jmarshall.com/easy/http/>
4. **MDN Web Docs - HTTP**: <https://developer.mozilla.org/en-US/docs/Web/HTTP>

### Next Steps for This Project

- Add support for handling more HTTP methods (POST, PUT, DELETE)
- Implement proper MIME type detection based on file extensions
- Add multithreading for concurrent connections
- Implement HTTP request and response logging
- Add configuration options (port, document root, etc.)
