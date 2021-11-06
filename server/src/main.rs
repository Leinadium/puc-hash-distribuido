/* server */

use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::Read;
use std::process;
use std::env;
use std::thread;
use std::collections::HashMap;
use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SendError};



struct SendHash {
    /// Estrutura que o loop_hash recebe como mensagem
    tipo: bool,                         // false para procurar, true para inserir
    chave: String,                      // chave para procurar ou inserir
    valor: String,                      // valor para ser inserido (nao é usado se for procurar)
    endereco: String,                   // endereço para guardar se nao tiver
    tx_resposta: Sender<ResponseHash>   // sender do canal para responder
}

struct ResponseHash {
    /// Estrutura que o loop_hash retorna como mensagem
    sucesso: bool,                  // true se encontrou ou inseriu com sucesso
    valor: String,                  // se foi uma busca, retorna o valor
    vetor: Vec<String>              // se foi uma inserção, retorna uma lista contendo os endereços pra responder
}

struct EsperaHash {
    /// Estrutura para a lista de espera de chaves e valores sendo procurados
    content: String,            // chave ou valor
    endereco: String,           // endereço de callback
}


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
    /// Trata a mensagem, executando as funções abaixo para cada caso
    ///   "insere--chave--valor" -> chama *calcula*, e depois *roteia* ou *coloca*
    ///   "insere_no--chave--valor--no" -> chama *roteia* ou *coloca*
    ///   "consulta--valor--endereço" -> chama *calcula*, e depois *roteia* ou *procura*
    ///   "consulta_no--valor--no--endereco" -> chama *roteia* ou *procura*
    ///   "fecha" -> fecha o servidor

    // retira os escapes
    let m = mensagem.replace("\\--", "--");
    let v: Vec<&str> = m.split("--").collect();

    // decide para onde ir
    if m.starts_with("insere--") {
        if v.len() != 3 {println!("mensagem de insercao invalida !"); return;}
        // let chave = v[1].to_string();
        // let valor = v[2].to_string();
        // TODO: chamar calcula, e depois roteia ou coloca
    }
    else if m.starts_with("insere_no") {
        if v.len() != 4 {println!("mensagem de insercao com no invalida !"); return;}
        // let chave = v[1].to_string();
        // let valor = v[2].to_string();
        //let no = v[3].to_string();
        // TODO: chamar roteia ou coloca
    }
    else if m.starts_with("consulta--") {
        if v.len() != 3 {println!("mensagem de insercao invalida !"); return;}
        // let valor = v[1].to_string();
        // let endereco = v[2].to_string();
        // TODO: chamar calcula, e depois roteia ou procura
    }
    else if m.starts_with("consulta_no") {
        if v.len() != 4 {println!("mensagem de insercao com no invalida !"); return;}
        // let valor = v[1].to_string();
        // let endereco = v[2].to_string();
        // let no = v[3].to_string();
        // TODO: chama roteia ou procura
    }
    else if m.starts_with("fecha") {
        println!("closing server");
        exit(0);
    }
    else {
        println!("mensagem invalida !");
    }
}


fn loop_hash(receiver: Receiver<SendHash>) {
    /// Loop para thread de alteração da hashlist
    /// a resposta a ser enviada depende do tipo.
    /// se for uma procura, retorna o valor encontrado, ou ""

    let mut hashmap: HashMap<String, String> = HashMap::new();
    let mut espera_list: Vec<EsperaHash> = Vec::new();

    println!("hashloop: listening for operations");
    loop {
        let m = receiver.recv();
        let mensagem_hash = match m {
            Ok(x) => x,
            Err(_) => {
                println!("hashloop: recv error. stopping hashloop");
                break ;     // erro, fecha o loop
            },
        };
        println!("hashloop: parsing message");
        let mut response = ResponseHash { sucesso: true, valor: "".to_string(), vetor: Vec::new() };

        // procura na hashmap
        if !mensagem_hash.tipo {
            if hashmap.contains_key(&mensagem_hash.chave) {
                response.valor = hashmap.get(&mensagem_hash.chave).unwrap().clone();
            } else {
                response.sucesso = false;
                // salvando na espera_list
                let espera = EsperaHash {content: mensagem_hash.chave, endereco: mensagem_hash.endereco};
                espera_list.push(espera);
            }
        }
        // insere na hashlist
        else {
            hashmap.insert(mensagem_hash.chave.clone(), mensagem_hash.valor.clone());

            // verificando a lista de espera
            for valor_esp in espera_list.iter() {
                if valor_esp.content == mensagem_hash.chave {
                    response.vetor.push(valor_esp.endereco.clone());
                }
            }
            // removendo da lista de espera, se houver
            espera_list.retain(|x| *x.content != mensagem_hash.chave);

        }
        println!("hashloop: finished parsing");
        mensagem_hash.tx_resposta.send(response).expect("hashloop: callback fail");
    }
}


fn main() {
    // verificando node e portas
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

    // criando o hash
    println!("creating channel for hash_loop");
    let (tx_hash, rx_hash) = mpsc::channel();
    println!("starting hash thread");
    thread::spawn(move|| { loop_hash(rx_hash); });


    // iniciando listener TCP
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("node {} running on port {}", node, port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("new conn: {}", stream.peer_addr().unwrap());
        let mensagem = get_mensagem(stream);

        // trata a mensagem
        thread::spawn(move || {trata(mensagem); });
    }
}
