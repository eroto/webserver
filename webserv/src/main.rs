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
    fs, io::{prelude::*, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration, u8
};
use webserv::ThreadPool;
use mime_guess::from_path;

fn main()
{
    let listener:TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    println!("Web server started");
    
    for stream_result in listener.incoming()
    {
        match stream_result{
            Ok(stream)=> {
                pool.execute(||{let hdl_connect_result  = handle_connection(stream);
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
    println!("Shutting down...");
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
        "GET / HTTP/1.1" =>
        {
            let file = "index.html";
            (ok_resp,file)
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
