/* client */

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::{env, process};
use std::io;

use api::disthash;

fn main() {
    let args:  Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} path_to_server_executable", &args[0]);
        process::exit(0x01);
    }
}

fn main_antiga() {
    let mut buffer = String::new();

    println!("Digite a porta para enviar uma mensagem: ");
    io::stdin().read_line(&mut buffer).expect("erro lendo string");
    let port :i32 = match buffer.trim_end().parse::<i32>() {
        Ok(num) => num,
        Err(erro) => {println!("porta invalida: {}", erro); process::exit(0x01);}
    };

    println!("Iniciando envio de mensagem de bom dia para {}", port);

    // iniciando conexao TCP
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();

    println!("Conectado a porta {}", port);

    // enviando mensagem
    stream.write(b"bom dia").expect("erro escrevendo bom dia");

    println!("Enviado! esperando fechar...");

    let mut temp = [0; 128];
    while stream.read(&mut temp).expect("Erro") > 0 {}

    println!("Finalizando");
}
