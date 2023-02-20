mod args;
mod handler;
use handler::socks5;

use args::parser::Args;

fn main() {
    let args = Args::get_args();
    println!("Listen and server on: {:?}", args.listen);

    let listener = std::net::TcpListener::bind(args.listen.as_str()).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(data) => {
                std::thread::spawn(move || {
                    if let Err(err) = socks5::handler(&data) {
                        println!("handler error: {:?}", err);
                    }
                });
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }
}
