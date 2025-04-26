<!-- 
This section of the README file outlines the components of the webserver project. 
It provides a detailed breakdown of the various modules or parts that make up the system.
-->

# webserver
rust web server

# main.rs
 A simple multi-threaded web server implemented in Rust.

 This web server listens for incoming TCP connections on `127.0.0.1:7878`
 and serves static files based on HTTP GET requests. It uses a thread pool
 to handle multiple connections concurrently.

 # Features
 - Serves static files such as `index.html`, `pitaya.jpeg`, `maiz.jpg`, and `pitaya.html`.
 - Handles a special `/sleep` route that simulates a delay of 5 seconds.
 - Returns a `404 Not Found` response for unknown routes.
 - Determines the MIME type of files using the `mime_guess` crate.


 # Components
 - **ThreadPool**: A custom thread pool implementation for managing worker threads.
 - **handle_connection**: Processes individual client connections and serves appropriate responses.

 # HTTP Responses
 - `200 OK`: Returned for valid requests with the requested file content.
 - `404 NOT FOUND`: Returned for invalid or unknown routes.

 # Example Usage
 Start the server by running the program. Access it via a web browser or a tool like `curl`:
 curl http://127.0.0.1:7878/
 curl http://127.0.0.1:7878/pitaya.jpeg
 curl http://127.0.0.1:7878/sleep
 curl http://127.0.0.1:7878/unknown


 # Error Handling
 - Logs errors to the console if a connection cannot be established or if an HTTP request is malformed.
 - Returns a `404` page if the requested file is not found.

 # Dependencies
 - `mime_guess`: Used to determine the MIME type of files.
 - `webserv::ThreadPool`: A custom thread pool implementation for managing worker threads.

 # Notes
 - The server shuts down gracefully when terminated.
 - The `Content-Type` header is dynamically set based on the file's MIME type.
 - The server currently supports only a limited set of predefined routes and files.

# lib.rs
 A "ThreadPool" is a collection of worker threads that can execute tasks concurrently.
 
 # Methods
 
 ## `new`
 
 Creates a new `ThreadPool` with the specified number of threads.
 
 - **Arguments**:
   - `size`: The number of threads in the pool. Must be greater than 0.
 
 - **Returns**: 
   - A `ThreadPool` instance with the specified number of worker threads.
 
 - **Panics**:
   - If `size` is 0.
 
 - **Example**:
 ```rust
 let pool = ThreadPool::new(4);
 ```
 
 ## `execute`
 
 Executes a task on the thread pool.
 
 - **Arguments**:
   - `f`: A closure or function that implements `FnOnce() + Send + 'static`.
 
 - **Behavior**:
   - The task is sent to the thread pool and executed by one of the worker threads.
 
 - **Example**:
 ```rust
 pool.execute(|| {
     println!("Task executed in the thread pool");
 });
 ```
