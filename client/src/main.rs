/*
client

./client fechar [q_nos]
./client insere [no] [chave] [valor]
./client consulta [no] [cjave]

*/

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{env, process};
use std::io;
use port_scanner;
use api::disthash;

fn main() {
    let args:  Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("invalid arguments! read README.md for more details");
        process::exit(0x01);
    }
    let operacao = args.get(1).unwrap();
    if operacao.starts_with("fechar") {
        let q: i32 = args.get(2)
            .expect("invalid q_nos argument")
            .parse::<i32>()
            .expect("q_nos is not a number");

        println!("closing node ring");
        disthash::fecha(q);
    }
    else if operacao.starts_with("insere") {
        let node: i32 = args.get(2)
            .expect("invalid no argument")
            .parse::<i32>()
            .expect("no is not a number");

        let chave = args.get(3)
            .expect("invalid chave argument");

        let valor = args.get(4)
            .expect("invalid valor argument");

        println!("executing insere");
        disthash::insere(node, &chave, &valor);
    }
    else if operacao.starts_with("consulta")  {
        let node: i32 = args.get(2)
            .expect("invalid no argument")
            .parse::<i32>()
            .expect("no is not a number");

        let chave = args.get(3)
            .expect("invalid chave argument");

        // port scan:
        let port = port_scanner::request_open_port().unwrap_or((15123 + node) as u16);
        let endereco = format!("127.0.0.1:{}", port);

        let listener = TcpListener::bind(&endereco)
            .expect("error opening tcplistener");

        println!("executing consulta");
        disthash::consulta(port as i32, node, &chave, &endereco);

        let mut stream = match listener.accept() {
            Ok((s, _addr)) => s,
            Err(e) => {
                println!("couldn't get server reponse: {:?}", e);
                process::exit(0x01);
            },
        };

        let mut buffer = [0; 128];
        match stream.read(&mut buffer) {
            Ok(_) => {},
            Err(e) => {println!("error processing callback {:?}", e); process::exit(0x01);}
        }
        let callback_m = String::from_utf8_lossy(&buffer).to_string();
        let callback_m_safe = callback_m.replace("\\--", "--");
        let callback_v: Vec<&str> = callback_m_safe.split("--").collect();

        let valor = callback_v.get(0).expect("invalid valor argument on callback");
        let node_valor = callback_v.get(1).expect("invalid node argument on callback");

        println!("Consulta {} enviada a nó {}: valor para '{}' armazenada no nó {}: {}", port, node, chave, node_valor, valor);
    }
    println!("closing client.")
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
