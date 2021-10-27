/* server */

use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::Read;
use std::process;
use std::env;


fn get_mensagem(mut stream: TcpStream) ->String {
    let mut buffer = [0; 128];

    let _ = match stream.read(&mut buffer) {
        Ok(num) => num,
        Err(_) => {println!("error processing message"); 0},
    };
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => {}
        Err(_) => {println!("error closing stream"); process::exit(0x01);}
    }

    return String::from_utf8_lossy(&buffer).to_string();
}

fn trata(mensagem: String) {
    /// Trata a mensagem, executando as funções abaixo para cada caso:
    ///   "insere--chave--valor" -> chama *calcula*, e depois *roteia* ou *coloca*
    ///   "insere_no--chave--valor--no" -> chama *roteia* ou *coloca*
    ///   "consulta--valor--endereço" -> chama *calcula*, e depois *roteia* ou *procura*
    ///   "consulta_no--valor--no--endereco" -> chama *roteia* ou *procura*

    // retira os escapes
    let m = mensagem.replace("\\--", "--");
    let v: Vec<&str> = m.split("--").collect();

    // decide para onde ir
    if m.starts_with("insere--") {
        if v.len() != 3 {println!("mensagem de insercao invalida !"); return;}
        let chave = v[1].to_string();
        let valor = v[2].to_string();
        // TODO: chamar calcula, e depois roteia ou coloca
    }
    else if m.starts_with("insere_no") {
        if v.len() != 4 {println!("mensagem de insercao com no invalida !"); return;}
        let chave = v[1].to_string();
        let valor = v[2].to_string();
        let no = v[3].to_string();
        // TODO: chamar roteia ou coloca
    }
    else if m.starts_with("consulta--") {
        if v.len() != 3 {println!("mensagem de insercao invalida !"); return;}
        let valor = v[1].to_string();
        let endereco = v[2].to_string();
        // TODO: chamar calcula, e depois roteia ou procura
    }
    else if m.starts_with("consulta_no") {
        if v.len() != 4 {println!("mensagem de insercao com no invalida !"); return;}
        let valor = v[1].to_string();
        let endereco = v[2].to_string();
        let no = v[3].to_string();
        // TODO: chama roteia ou procura
    }
    else {
        println!("mensagem invalida !");
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: {} node_number port", &args[0]);
        process::exit(0x01);
    }
    let temp_node = &args[1].parse::<i32>();
    let temp_port = &args[2].parse::<i32>();

    let node = match temp_node {
        Ok(num) => num,
        Err(_) => {println!("invalid node argument"); process::exit(0x01);}
    };
    let port = match temp_port {
        Ok(num) => num,
        Err(_) => {println!("invalid port argument"); process::exit(0x01);}
    };

    // iniciando listener TCP
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("node {} running on port {}", node, port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("new conn: {}", stream.peer_addr().unwrap());
        let mensagem = get_mensagem(stream);
        println!("message: {}", mensagem);
        // TODO: passar mensagem para tratar()
    }
}
