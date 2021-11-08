/* server */

use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fmt::Write as wrt;
use std::process;
use std::env;
use std::thread;
use std::collections::HashMap;
use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};



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

    let size = match stream.read(&mut buffer) {
        Ok(num) => num,
        Err(_) => {println!("error processing message"); 0},
    };
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => {}
        Err(_) => {println!("error closing stream"); process::exit(0x01);}
    }

    return String::from_utf8_lossy(&buffer[0..size]).to_string();
}

fn calcula(chave: &String, power2_nodes: &i32) -> i32 {
    /// Calcula um hash para a chave para saber a qual nó enviar a mensagem

    // obtém a quantidade de nós da arquitetura
    let node_quantity = i32::pow(2, *power2_nodes as u32);

    // inicializa o somador
    let mut soma = 0;

    // para cada byte na String da chave, soma seu valor no somador
    for byte in chave.as_bytes() {

        // obtém em uma string em hexadecimal, o valor do byte
        let mut hexstring = String::new();
        write!(&mut hexstring, "{:x}", &byte).expect("unable to write"); // converte de byte para hexstring

        // converte o valor hexadecimal de string para inteiro para poder realizar a soma
        let byte_int = match i32::from_str_radix(hexstring.as_str(), 16) {
            Ok(num) => num,
            Err(_) => {println!("invalid hexstring"); process::exit(0x01);}
        };

        // adiciona o byte ao somador
        soma += byte_int;
    }

    // calcula o hash a partir de "soma mod (quantidade de nós)"
    return soma % node_quantity; // calcula o hash
}

fn roteia(no_destino: i32, no_atual: &i32, power2_nodes: &i32, chave: &String, tipo: String, valor_ou_endereco: &String) {
    /// Descobre qual o próximo nó que deve ser visitado e então faz uma conexão TCP para ele
    /// Considerou-se que as portas começam em 7001

    let mut no_prox = -1; // próximo nó
    let node_quantity = i32::pow(2, *power2_nodes as u32);

    // se o nó destino é menor que o nó atual, a soma do nó atual com as potências de 2 dará maior que 2^power2_nodes
    // assim, para facilitar a lógica, pode-se complementar o nó procurado com o valor de node_quantity
    let mut no_procurado = no_destino;
    if no_destino < *no_atual {
        no_procurado += node_quantity;
    }

    // calcula o próximo nó a ser visitado
    for i in 0..*power2_nodes {
        let no_verifica = no_atual + i32::pow(2, i as u32); // no_atual + 2^i

        // se o nó verificado é menor que o destino, então deve-se ir ao no verificado anteriormente
        if no_prox != -1 && no_verifica > no_procurado {
            break;
        }
        no_prox = no_verifica;
    }
    no_prox = no_prox % node_quantity;

    // faz conexão TCP com o próximo nó
    let connection_port = 7000 + no_prox;
    if let Ok(mut stream) = TcpStream::connect(format!("127.0.0.1:{}", connection_port)) {
        // prepara a mensagem
        let message = format!("{}--{}--{}--{}", tipo, chave, valor_ou_endereco, no_destino);
        let bufsend = message.as_bytes();

        // envia a mensagem
        let res = stream.write(bufsend);
        res.unwrap();
    }
    else {
        println!("não consegui me conectar...");
    }
}

fn callback(valor: &String, endereco: &String, no_atual: &i32) {
    let address = endereco.clone();

    if let Ok(mut stream) = TcpStream::connect(address) {
        // prepara a mensagem
        let message = format!("{}--{}", valor, no_atual);
        let bufsend = message.as_bytes();

        // envia a mensagem
        let res = stream.write(bufsend);
        res.unwrap();
    }
    else {
        println!("não consegui me conectar ao endereço informado pelo cliente...");
    }
}

fn coloca(chave: &String, valor: &String, no_atual: &i32, tx_sender: Sender<SendHash>) {
    /// Envia uma mensagem por um canal direto com uma thread que coloca o conjunto chave-valor na hashmap desse servidor.
    /// Se forem recebidos endereços que estavam esperando a inserção desse conjunto chave-valor, faz um callback para eles.

    // faz o clone para envio da chave e do valor
    let key = (*chave.clone()).to_string();
    let value = (*valor.clone()).to_string();

    // cria o canal de comunicação de resposta
    let (sender, receiver) = mpsc::channel();

    // prepara a hash de envio e envia
    let send_hash = SendHash {tipo: true, chave: key.to_string(), valor: value.to_string(), endereco: "".to_string(), tx_resposta: sender};
    tx_sender.send(send_hash).expect("coloca: erro no envio");

    // obtém a resposta
    let response = receiver.recv();
    let response_hash = match response {
        Ok(x) => x,
        Err(_) => {
            println!("hashloop: recv error. stopping hashloop");
            process::exit(0x01);
        },
    };

    // se houverem endereços esperando resposta, realiza um callback para eles
    for endereco in response_hash.vetor {
        callback(valor, &endereco, no_atual);
    }
}

fn procura(chave: &String, endereco: &String, no_atual: &i32, tx_sender: Sender<SendHash>) {
    /// Envia uma mensagem por um canal direto com uma thread que procura o conjunto chave-valor na hashmap desse servidor.
    /// Se encontrado, faz um callback para o endereço que solicitou, senão deixa o endereço aguardando uma resposta.

    // faz o clone para envio da chave e do endereço
    let key = (*chave.clone()).to_string();
    let address = (*endereco.clone()).to_string();

    // cria o canal de comunicação de resposta
    let (sender, receiver) = mpsc::channel();

    // prepara a hash de envio e envia
    let send_hash = SendHash {tipo: false, chave: key, valor: "".to_string(), endereco: address, tx_resposta: sender};
    tx_sender.send(send_hash).expect("procura: erro no envio");

    // obtém a resposta
    let response = receiver.recv();
    let response_hash = match response {
        Ok(x) => x,
        Err(_) => {
            println!("hashloop: recv error. stopping hashloop");
            process::exit(0x01);
        },
    };

    // se encontrar a chave, realiza um callback para o endereço que solicitou a procura
    if response_hash.sucesso {
        callback(&response_hash.valor, &endereco, no_atual);
    }
}

fn trata(mensagem: String, node: &i32, power2_nodes: &i32, tx_sender: Sender<SendHash>) {
    /// Trata a mensagem, executando as funções abaixo para cada caso
    ///   "insere--chave--valor" -> chama *calcula*, e depois *roteia* ou *coloca*
    ///   "insere_no--chave--valor--no" -> chama *roteia* ou *coloca*
    ///   "consulta--chave--endereço" -> chama *calcula*, e depois *roteia* ou *procura*
    ///   "consulta_no--chave--no--endereco" -> chama *roteia* ou *procura*
    ///   "fecha" -> fecha o servidor

    // retira os escapes
    let m = mensagem.replace("\\--", "--");
    // println!("{}", m);
    let v: Vec<&str> = m.split("--").collect();

    // decide para onde ir
    if m.starts_with("insere--") {
        // chama calcula, e depois roteia ou coloca

        if v.len() != 3 {println!("mensagem de insercao invalida !"); return;}

        let chave = v[1].to_string();
        let valor = v[2].to_string();
        let next_node = calcula(&chave, power2_nodes);

        if next_node == *node {
            coloca(&chave, &valor, node, tx_sender);
        } else {
            roteia(next_node, node, power2_nodes, &chave, "insere_no".to_string(), &valor);
        }
    }
    else if m.starts_with("insere_no") {
        // chama roteia ou coloca

        if v.len() != 4 {println!("mensagem de insercao com no invalida !"); return;} // TODO TODO
        let chave = v[1].to_string();
        let valor = v[2].to_string();
        let no_destino = v[3].parse::<i32>().unwrap();

        if no_destino == *node {
            coloca(&chave, &valor, node, tx_sender);
        } else {
            roteia(no_destino, node, power2_nodes, &chave, "insere_no".to_string(), &valor);
        }
    }
    else if m.starts_with("consulta--") {
        // chama calcula, e depois roteia ou procura

        if v.len() != 3 {println!("mensagem de consulta invalida !"); return;}
        let chave = v[1].to_string();
        let endereco = v[2].to_string();
        let next_node = calcula(&chave, power2_nodes);

        if next_node == *node {
            procura(&chave, &endereco, node, tx_sender);
        } else {
            roteia(next_node, node, power2_nodes, &chave, "consulta_no".to_string(), &endereco);
        }
    }
    else if m.starts_with("consulta_no") {
        // chama roteia ou procura

        if v.len() != 4 {println!("mensagem de consulta com no invalida !"); return;}
        let chave = v[1].to_string();
        let endereco = v[2].to_string();
        let no_destino = v[3].parse::<i32>().unwrap();

        if no_destino == *node {
            procura(&chave, &endereco, node, tx_sender);
        } else {
            roteia(no_destino, node, power2_nodes, &chave, "consulta_no".to_string(), &endereco);
        }
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

    // println!("hashloop: listening for operations");
    loop {
        let m = receiver.recv();
        let mensagem_hash = match m {
            Ok(x) => x,
            Err(_) => {
                println!("hashloop: recv error. stopping hashloop");
                break ;     // erro, fecha o loop
            },
        };
        // println!("hashloop: parsing message");
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
        // println!("hashloop: finished parsing");
        mensagem_hash.tx_resposta.send(response).expect("hashloop: callback fail");
    }
}


fn main() {
    // verificando node e portas
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("usage: {} node_number port power2_nodes", &args[0]);
        process::exit(0x01);
    }
    let temp_node = args[1].parse::<i32>();
    let temp_port = args[2].parse::<i32>();
    let temp_power2_nodes = args[3].parse::<i32>();

    let node = match temp_node {
        Ok(num) => num,
        Err(_) => {println!("invalid node argument"); process::exit(0x01);}
    };
    let port = match temp_port {
        Ok(num) => num,
        Err(_) => {println!("invalid port argument"); process::exit(0x01);}
    };
    let power2_nodes = match temp_power2_nodes {
        Ok(num) => num,
        Err(_) => {println!("invalid power2_nodes argument"); process::exit(0x01);}
    };

    // criando o hash
    // println!("creating channel for hash_loop");
    let (tx_hash, rx_hash) = mpsc::channel();
    // println!("starting hash thread");
    thread::spawn(move|| { loop_hash(rx_hash); });


    // iniciando listener TCP
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("node {} running on port {}", node, port);
    for stream_enum in listener.incoming() {
        let stream = match stream_enum {
            Ok(s) => s,
            Err(_) => {
                // println!("closing server (by listener blocking call error)");
                exit(0); }
        };
        // println!("new conn: {}", stream.peer_addr().unwrap());
        let mensagem = get_mensagem(stream);
        let tx_sender = tx_hash.clone();
        // trata a mensagem
        thread::spawn(move || {trata(mensagem, &node, &power2_nodes, tx_sender); });
    }
}
