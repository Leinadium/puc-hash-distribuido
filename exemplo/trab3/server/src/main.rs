use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::process;

fn tratacon (mut s: TcpStream) {
  let mut buffer = [0; 128];
  let lidos = s.read(&mut buffer).unwrap();
  println!("recebi {} bytes", lidos);

  let st = String::from_utf8_lossy(&buffer);
  
  println!("recebeu: {}", st);

  let res = s.write(&buffer[0..lidos]);
  match res {
    Ok(num) => println!("escreveu {}", num),
    Err(_) => {println!("erro"); process::exit(0x01)},
  }
 
  // não sei transformar esse buffer em algo displayable
  //println!("recebi {} bytes: {}", lidos, buffer[0..lidos]);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println! ("vai esperar conexoes!");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        tratacon(stream);

    }
}