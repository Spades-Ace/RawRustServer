use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

// Constants for our HTTP server
const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const HTTP_VERSION: &str = "HTTP/1.1";
const SERVER_NAME: &str = "RustRawHTTP/1.0";

/// Main function - entry point of our HTTP server
fn main() {
    println!("Starting HTTP server at {}", SERVER_ADDRESS);
    
    // Create a TCP listener bound to the specified address
    // This is the core networking functionality that allows our program to accept connections
    let listener = match TcpListener::bind(SERVER_ADDRESS) {
        Ok(listener) => {
            println!("Successfully bound to address");
            listener
        },
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return;
        }
    };

    // Listen for incoming connections in an infinite loop
    println!("Waiting for connections...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Successfully accepted a connection, handle it
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_connection(stream);
            },
            Err(e) => {
                // Connection failed
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

/// Handles a single client connection by processing the HTTP request
/// and sending back an appropriate response
fn handle_connection(mut stream: TcpStream) {
    // Create a buffer to store the incoming data
    let mut buffer = [0; 1024];
    
    // Read data from the stream into our buffer
    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Received {} bytes", size);
            
            // Convert the buffer to a string so we can parse the HTTP request
            let request = String::from_utf8_lossy(&buffer[..size]);
            println!("Request: \n{}", request);
            
            // Parse the HTTP request to get the requested path
            // We only care about the first line which contains the HTTP method and path
            let request_line = request.lines().next().unwrap_or("");
            
            // Parse the HTTP method and path
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            if parts.len() >= 2 {
                let method = parts[0];
                let path = if parts[1] == "/" { "/index.html" } else { parts[1] };
                
                println!("Method: {}, Path: {}", method, path);
                
                // Only handle GET requests
                if method == "GET" {
                    serve_file(stream, path);
                } else {
                    // Method not supported
                    send_response(stream, 405, "Method Not Allowed", "Only GET method is supported");
                }
            } else {
                // Invalid request format
                send_response(stream, 400, "Bad Request", "Invalid request format");
            }
        },
        Err(e) => {
            eprintln!("Failed to read from connection: {}", e);
        }
    }
}

/// Attempts to serve a file from the local filesystem
fn serve_file(stream: TcpStream, path: &str) {
    // Remove the leading slash and construct the file path
    let file_path = format!("public{}", path);
    
    println!("Attempting to serve file: {}", file_path);
    
    // Try to read the file contents
    match fs::read_to_string(&file_path) {
        Ok(contents) => {
            // File found, send it with a 200 OK response
            send_response(stream, 200, "OK", &contents);
        },
        Err(_) => {
            // File not found or couldn't be read
            send_response(stream, 404, "Not Found", "The requested file was not found");
        }
    }
}

/// Sends an HTTP response with the specified status code and body
fn send_response(mut stream: TcpStream, status_code: u16, status_text: &str, body: &str) {
    // Get the content length for the response headers
    let content_length = body.len();
    
    // Create the HTTP response
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
    
    // Write the response to the stream
    match stream.write_all(response.as_bytes()) {
        Ok(_) => println!("Response sent successfully"),
        Err(e) => eprintln!("Failed to send response: {}", e)
    }
}

/// Determines the Content-Type header based on the file extension or content
fn get_content_type(content: &str) -> &'static str {
    // For simplicity, we'll just check if it looks like HTML
    if content.trim_start().starts_with("<!DOCTYPE html>") || 
       content.trim_start().starts_with("<html") {
        "text/html; charset=utf-8"
    } else {
        "text/plain; charset=utf-8"
    }
}
