# Raw Rust HTTP Server

A raw HTTP server implementation in Rust with no external dependencies. This project serves as a learning resource to understand how HTTP servers work from the ground up.

![Ferris the Crab](https://rustacean.net/assets/cuddlyferris.svg)

## Project Overview

This project implements a basic HTTP server from scratch that:

- Listens for TCP connections on port 8080
- Parses incoming HTTP requests
- Serves static files from the `public` directory
- Handles basic HTTP response codes (200, 404, 405)
- Implements proper HTTP headers

## How It Works

The server works by:

1. Creating a TCP listener that binds to a local address (127.0.0.1:8080)
2. Accepting incoming connections in a loop
3. Reading data from each connection and parsing it as an HTTP request
4. Extracting the requested path from the HTTP request
5. Serving the requested file from the public directory
6. Sending back an appropriate HTTP response with headers

## Setting Up Rust Environment

### Installing Rust

1. **Windows**:
   - Download and run the installer from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
   - Follow the on-screen instructions
   - After installation, open a new command prompt and verify installation with `rustc --version`

2. **macOS/Linux**:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   - Follow the on-screen instructions
   - After installation, run `source $HOME/.cargo/env`
   - Verify installation with `rustc --version`

### Updating Rust

To update your Rust installation:

```bash
rustup update
```

## Running the Server

1. Clone this repository:

   ```bash
   git clone https://github.com/Spades-Ace/RawRustServer
   cd rust-raw-http-server
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the server:

   ```bash
   cargo run --release
   ```

4. Open your browser and navigate to:

   ```
   http://localhost:8080
   ```

## Project Structure

```
/
|-- src/
|   |-- main.rs      # The main server code
|-- public/
|   |-- index.html   # The HTML file served by default
|-- Cargo.toml       # Project configuration
|-- README.md        # This file
```

## Understanding the Code

### Constants

- `SERVER_ADDRESS`: The address and port the server binds to
- `HTTP_VERSION`: The HTTP protocol version used in responses
- `SERVER_NAME`: The server name sent in response headers

### Main Functions

- `main()`: Entry point - creates a TCP listener and handles incoming connections
- `handle_connection()`: Processes a single client connection by reading and parsing an HTTP request
- `serve_file()`: Attempts to serve a requested file from the filesystem
- `send_response()`: Formats and sends an HTTP response with appropriate headers
- `get_content_type()`: Determines the content type for the response based on file content

## Learning Resources

To learn more about HTTP and Rust:

1. [HTTP Protocol - MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTTP)
2. [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
3. [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

## Future Improvements

- Add support for more HTTP methods (POST, PUT, etc.)
- Implement proper MIME type detection based on file extensions
- Add multithreading to handle concurrent connections
- Implement request and response logging
- Add configuration options (port, document root, etc.)

## License

This project is open source and available under the MIT License.

---

Happy coding and learning Rust!
