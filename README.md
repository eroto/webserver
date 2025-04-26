<!-- 
This section of the README file outlines the components of the webserver project. 
It provides a detailed breakdown of the various modules or parts that make up the system.
-->

# webserver
rust web server

# main.rs
 # Simple Web Server
 This program is a simple web server written in Rust that serves static files 
 and determines the MIME type of the files being served. It is designed to run 
 on a specified IP address and port, and it uses the `WebServer` struct from 
 the `webserv` crate to handle the server functionality.
 
 ## Features
 - Serves static files.
 - Determines and handles MIME types for the served files.
 
 ## Usage
 The server is started by calling the `start_server` function with the desired 
 IP address and port number. The server will run until it is manually stopped, 
 after which a shutdown message will be printed to the console.
 
## License
This program is licensed under the GNU General Public License v3.0 (GPLv3). 
You are free to redistribute and/or modify it under the terms of the license. 
See the license text for more details.

# lib.rs

 A module that provides a simple implementation of a thread pool for managing
 concurrent execution of tasks.

 # Overview
 The `ThreadPool` struct allows you to create a pool of worker threads that
 can execute tasks concurrently. Tasks are sent to the thread pool via a
 channel, and each worker thread picks up tasks from the channel and executes
 them.

 # Structs
 - `ThreadPool`: Manages a pool of worker threads and provides an interface
   for submitting tasks.
 - `Worker`: Represents an individual worker thread in the thread pool.

 # Type Aliases
 - `Job`: A type alias for a boxed closure that represents a task to be
   executed by the thread pool.

 # Examples
 ```rust
 use webserv::ThreadPool;

 let pool = ThreadPool::new(4);

 pool.execute(|| {
     println!("Task 1 executed");
 });

 pool.execute(|| {
     println!("Task 2 executed");
 });
 ```

 # Implementation Details
 - The `ThreadPool` struct maintains a vector of `Worker` instances and a
   sender for sending tasks to the workers.
 - The `Worker` struct contains an ID and a thread handle. Each worker runs
   in a loop, receiving tasks from a shared channel and executing them.
 - The `Drop` trait is implemented for `ThreadPool` to gracefully shut down
   all worker threads when the thread pool goes out of scope.

 # Methods
 ## `ThreadPool`
   Creates a new thread pool with the specified number of worker threads.
   Panics if the size is zero.
 - `execute<F>(&self, f: F)`
   Submits a task to the thread pool for execution. The task must implement
   the `FnOnce` trait and be `Send` and `'static`.

 ## `Worker`
 - `new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker`
   Creates a new worker thread with the specified ID and a shared receiver
   for receiving tasks.

 # Notes
 - The thread pool uses an `mpsc` channel for task communication and an
   `Arc<Mutex<_>>` to share the receiver among worker threads.
 - When the `ThreadPool` is dropped, it shuts down all workers by dropping
   the sender and joining all worker threads.

# wsrv.rs
 A struct representing a simple multi-threaded web server.

 The `WebServer` struct provides functionality to create and run a web server
 that listens for incoming HTTP requests and serves static files. It also tracks
 server uptime and the number of connections handled.

 # Fields
 - `address`: The IP address or hostname where the server will listen for connections.
 - `port`: The port number where the server will listen for connections.
 - `start_time`: The time when the server was created, used to calculate uptime.
 - `connections_handled`: The number of connections successfully handled by the server.

 # Methods
 - `new(address: String, port: u16) -> WebServer`:
   Creates a new instance of the `WebServer` with the specified address and port.
   Prints server details and initialization time.
 - `run(&mut self)`:
   Starts the server, listens for incoming connections, and handles them using a thread pool.
   Increments the number of connections handled for each successful connection.
 - `get_uptime(&self) -> u64`:
   Returns the server's uptime in seconds.
 - `get_connections_handled(&self) -> u64`:
   Returns the total number of connections handled by the server.
 - `increment_connections_handled(&mut self)`:
   Increments the internal counter for the number of connections handled.

 # Drop Trait
 Implements the `Drop` trait to perform cleanup when the server is dropped.
 Prints the server's uptime and the total number of connections handled.

 # Example
 ```rust
 let mut server = WebServer::new("127.0.0.1".to_string(), 8080);
 server.run();
 ```


 