//Created by: Enrique Rodriguez Toscano
//Date: 2023-10-01
//Description: Simple web server in Rust that serves static files and determines the MIME type of the files being served.
//License: GNU GPLv3
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.


use webserv::wsrv::WebServer;
fn main()
{
    start_server("127.0.0.1".to_string(),7878);

    println!("Shutting down...");
}

fn start_server(address:String, port: u16)
{
    let mut www_server = WebServer::new(address.to_string(),port);

    www_server.run();
}
