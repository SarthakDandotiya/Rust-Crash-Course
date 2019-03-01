// Creating a Webserver[Extremly barebones], it outputs when a request is made
// Just go to the address 127.0.0.1:8000 although the browser will give error as this is not a proper web server, this does responds or gives outputs when requests are made to it.
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("A Connection was made!");
    }
}
