//Created by: Enrique Rodriguez Toscano
//Date: 2023-10-01
//Description: Simple web server in Rust that serves static files and determines the MIME type of the files being served.
//License: GNU GPLv3
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.

use std::
{
    fs, io::{prelude::*, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::{Duration, SystemTime}, u8
};
use crate::ThreadPool;
use mime_guess::from_path;

pub struct WebServer
{
    address: String,
    port: u16,
    start_time: std::time::SystemTime,
    connections_handled: u64,
}

fn handle_connection(mut stream: TcpStream) -> Result<u8,String>
{
    println!("New client connected");
    let buf_reader = BufReader::new(&mut stream);

    let http_line_request= match buf_reader.lines().next(){
        Some(Ok(line))=> line,
        Some(Err(e))=> return Err(format!("Error reading http header line{}",e)),
        None => return Err("No https request line received".to_string()),
    };

    let req_parts: Vec<&str> = http_line_request.split_whitespace().collect();

    let file:&str = if req_parts.len() > 1
    {
        req_parts[1]
    }
    else //Invalid request
    {
        return Err("Invalid request received".to_string())
    };
    println!("file:{}",file);

    let ok_resp: &str = "HTTP/1.1 200 OK";
    let nok_resp: &str = "HTTP/1.1 404 NOT FOUND";
        
    let(status_line, file_name) = match &http_line_request[..]
    {
        "GET / HTTP/1.1" | "GET /index.html HTTP/1.1" =>
        {
            let file = "index.html";
            (ok_resp, file)
        },
        
        "GET /pitaya.jpeg HTTP/1.1" =>
        {
            let file= "pitaya.jpeg";
            (ok_resp,file)
        },

        "GET /maiz.jpg HTTP/1.1" =>
        {
            let file= "maiz.jpg";
            (ok_resp,file)
        },

        "GET /pitaya.html HTTP/1.1" =>
        {
            let file= "pitaya.html";
            (ok_resp,file)
        },

        "GET /sleep HTTP/1.1" =>
            {thread::sleep(Duration::from_secs(5));
            (ok_resp,"index.html")}

        _ =>
        {
            let file = "err404.html";
            (nok_resp,file)
        },
            
    };
        let content= fs::read(file_name).unwrap();
        let length = content.len();
        let mime_type = from_path(file_name).first_or_octet_stream();
        println!("{}",mime_type);

        let response= format!("{status_line}\r\nContent-Type:{mime_type}\r\nContent-Length{length}\r\n\r\n");

        stream.write(response.as_bytes()).expect("Failed to write response");
        stream.write(&content).expect("Failed to write content");

        Ok(1)
}

impl WebServer
{
    pub fn new(address: String, port: u16) -> WebServer
    {
        //Print attributes of the server
        println!("Web server created");
        println!("Address: {}",address);
        println!("Port: {}",port);
        println!("Server started at: {}",std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        print!("Call run() to start the Web server");

        let up_time: SystemTime = std::time::SystemTime::now();
        
        //Return a new instance of WebServer
        WebServer
            {
                address:address,
                port:port,
                start_time: up_time,
                connections_handled: 0,
            }
    }

    pub fn run(&mut self)
    {
        //Create a TCP listener
        let listener = TcpListener::bind(format!("{}:{}",self.address,self.port)).unwrap();
        println!("Listening on {}:{}",self.address,self.port);

        //Create a thread pool with 4 threads
        let pool = ThreadPool::new(4);

        for stream_result in listener.incoming()
        {
            match stream_result
            {
                Ok(stream)=> { pool.execute(||{let hdl_connect_result = handle_connection(stream);
                    match hdl_connect_result
                    {
                        Ok(ret_val) => {println!("Connection handled correctly: {}",ret_val);}   
                        Err(error_txt)=> {println!("handle_connecion error:{}",error_txt);}
                    }
                
                    });
                }
            Err(e)=>{eprintln!("Erro handling incomming correction: {}",e);}
            }
        }
        //call the method to increment the number of connections handled
        self.increment_connections_handled();
    }

    fn get_uptime(&self) -> u64
    {
        let elpased = self.start_time.elapsed().unwrap();
        let seconds = elpased.as_secs();
        seconds
    }
    fn get_connections_handled(&self) -> u64
    {
        self.connections_handled
    }
    
    fn increment_connections_handled(&mut self)
    {
        self.connections_handled += 1;
    }
    
}

impl Drop for WebServer
{
    fn drop(&mut self)
    {
        println!("Shutting down... Web server dropped");
        println!("Server was running for {} seconds",self.get_uptime());
        println!("Number of connections handled: {}",self.get_connections_handled());
    }
    
}
