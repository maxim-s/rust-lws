use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;
use std::io::net::tcp::TcpStream;
use std::str::from_utf8;
use std::rc::Rc;
use std::cell::{RefCell};


pub struct Request{	
	stream : Rc<RefCell<TcpStream>>
}

impl Request {
	pub fn new(stream :  TcpStream) -> Request {
		Request {
			stream : Rc::new(RefCell::new(stream))
		}
	}
	pub fn read(&self) {
		let mut buf = [0u8, ..1024];
		match self.stream.borrow_mut().read(buf){
			Ok(length) => {
				println!("Request length is : {}", length);
				let req = from_utf8(buf).expect("unable to parse buf to a utf8 string");
				let mut batches = req.split_str("\n");
				for batch in batches {
					println!("{}",batch);
				}
				//println!("{:s}", req);
			},
			Err(e) =>  println!("Failed read request : {}!", e), 
		};

	}
}

pub struct Response{
    stream : Rc<RefCell<TcpStream>>
}

impl Response {
	pub fn new(stream : TcpStream) -> Response {
		Response {
			stream :Rc::new(RefCell::new(stream)),
		}
	}
	fn write(&self, content:&[u8] ){
		self.stream.borrow_mut().write(content);
	}
}


fn main() {
    let mut acceptor = TcpListener::bind("192.168.56.102:8081").listen();
    println!("listening....");
    for stream in acceptor.incoming() {
    	match stream {
	        Err(e) => println!("Failed to initiate server {}", e),
	        Ok(stream) => spawn(proc() {	 
	        	let request = Request::new(stream.clone());      
	        	request.read(); 	
	        	let content = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\ncontent-length: 20\r\n\r\n<h1>Hello world</h1>";	      	
	        	let response = Response::new(stream.clone());
	        	response.write(content);
    			drop(stream);
	        })
	    }
    }
    drop(acceptor);
}