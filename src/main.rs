use std::{env, net::{TcpListener, TcpStream}, io};
use std::io::prelude::*;


mod lib;


use lib::*;



fn main(){
    //load env file
    helpers::get_env();

    //get the listening address
    let address = env::var("HOST").unwrap() + ":" + &env::var("PORT").unwrap();

    //bind tcp address
    let listener = TcpListener::bind(&address).unwrap();
    println!("Sever has started on {}", address);
    for stream in listener.incoming() {
        println!("{:#?}", stream);
        handle_stream(&mut stream.unwrap());
    }
}



fn handle_stream(s: &mut TcpStream){
    loop {
        let mut empty_buffer = Vec::from([0; 1024]);
        s.read(&mut empty_buffer).unwrap();
        io::stdout().write(&mut empty_buffer).unwrap();
    
        //then write it back to the stream
        s.write(&mut empty_buffer).unwrap();
        s.flush();
    }
}