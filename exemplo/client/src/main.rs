use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::process;

fn main () {
  if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
    println!("me conectei com o servidor!");
    let bufsend = "<FILE><name><000>".as_bytes();

    let res = stream.write(bufsend);
    match res {
      Ok(num) => println!("escreveu {}", num),
      Err(_) => {println!("erro na escrita"); process::exit(0x0)},
    }

    let mut buffer = [0; 128];
    //let lidos = stream.read(&mut buffer).expect("erro no recebimento!");
    let lidos = stream.read(&mut buffer).unwrap();

    println!("recebi de volta {} bytes: {}", lidos,
              String::from_utf8_lossy(&buffer));
  }
  else {
    println!("não consegui me conectar...");
  }
}