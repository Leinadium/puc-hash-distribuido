use std::process;
use std::process::{Command, Child};
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn create_server_ring(mut process_vec: Vec<Child>, power2_nodes: i32) -> Vec<Child> {
    let node_quantity = i32::pow(2, power2_nodes as u32);
    for i in 0..node_quantity {
        process_vec.push(Command::new("server.exe").arg(format!("{}", i)).arg(format!("{}", 7000 + i)).arg(format!("{}", power2_nodes)).spawn().expect("erro ao executar comando"));
    }
    return process_vec;
}

fn create_client_fechar(mut process_vec: Vec<Child>, q_nos: i32) -> Vec<Child> {
    process_vec.push(Command::new("client.exe").arg("fechar").arg(format!("{}", q_nos)).spawn().expect("erro ao executar comando"));
    return process_vec;
}

fn create_client_insere(mut process_vec: Vec<Child>, no: i32, chave: String, valor: String) -> Vec<Child> {
    process_vec.push(Command::new("client.exe").arg("insere").arg(format!("{}", no)).arg(chave).arg(valor).spawn().expect("erro ao executar comando"));
    return process_vec;
}

fn create_client_consulta(mut process_vec: Vec<Child>, no: i32, chave: String) -> Vec<Child> {
    process_vec.push(Command::new("client.exe").arg("consulta").arg(format!("{}", no)).arg(chave).spawn().expect("erro ao executar comando"));
    return process_vec;
}

fn test_case_1() {
    /// Servidor: Testa a criação de um anel servidor de 16 nós
    /// Clientes: Testa uma inserção e uma consulta
    let mut process_vec = Vec::new();
    process_vec = create_server_ring(process_vec, 4);
    sleep(Duration::new(3,0));
    process_vec = create_client_insere(process_vec, 3, "teste".to_string(), "teste".to_string());
    sleep(Duration::new(3,0));
    process_vec = create_client_consulta(process_vec, 3, "teste".to_string());
    sleep(Duration::new(10,0));
    process_vec = create_client_fechar(process_vec, 16);

    // espera até todas as childs acabarem
    for mut child in process_vec {
        child.wait().unwrap();
    };
}

fn test_case_2() {
    /// Servidor: Testa a criação de um anel servidor de 16 nós
    /// Clientes: Testa uma consulta antes de ter a chave e então insere
    let mut process_vec = Vec::new();
    process_vec = create_server_ring(process_vec, 4);
    sleep(Duration::new(3,0));
    process_vec = create_client_insere(process_vec, 3, "teste".to_string(), "teste".to_string());
    sleep(Duration::new(3,0));
    process_vec = create_client_consulta(process_vec, 3, "teste".to_string());
    sleep(Duration::new(10,0));
    process_vec = create_client_fechar(process_vec, 16);

    // espera até todas as childs acabarem
    for mut child in process_vec {
        child.wait().unwrap();
    };
}

fn test_case_3() {
    /// Servidor: Testa a criação de um anel servidor de 32 nós
    /// Clientes: Testa uma inserção por vários nós e faz consultas das chaves simultâneamente e desordenadamente
    let mut process_vec = Vec::new();
    let string_vec = vec!["teste1".to_string(), "teste2".to_string(), "teste3".to_string(), "teste4".to_string(), "teste5".to_string()];

    // cria os nós do servidor
    process_vec = create_server_ring(process_vec, 4);
    sleep(Duration::new(3,0));

    // começa a consultar e inserir desordenadamente (todas as consultas tem de funcionar eventualmente)
    process_vec = create_client_consulta(process_vec, 22, string_vec.get(3).unwrap().clone());
    process_vec = create_client_consulta(process_vec, 31, string_vec.get(2).unwrap().clone());
    process_vec = create_client_insere(process_vec, 0, string_vec.get(0).unwrap().clone(), "1".to_string());
    process_vec = create_client_insere(process_vec, 17, string_vec.get(1).unwrap().clone(), "2".to_string());
    process_vec = create_client_consulta(process_vec, 15, string_vec.get(1).unwrap().clone());
    process_vec = create_client_insere(process_vec, 12, string_vec.get(2).unwrap().clone(), "3".to_string());
    process_vec = create_client_consulta(process_vec, 4, string_vec.get(0).unwrap().clone());
    process_vec = create_client_insere(process_vec, 25, string_vec.get(3).unwrap().clone(), "4".to_string());

    sleep(Duration::new(5,0));
    process_vec = create_client_fechar(process_vec, 16);

    // espera até todas as childs acabarem
    for mut child in process_vec {
        child.wait().unwrap();
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} test_number", &args[0]);
        process::exit(0x01);
    }
    let temp_test_number = args[1].parse::<i32>();

    let test_number = match temp_test_number {
        Ok(num) => num,
        Err(_) => {println!("invalid node argument"); process::exit(0x01);}
    };
    println!("teste run");
    match test_number {
        1 => test_case_1(),
        2 => test_case_2(),
        3 => test_case_3(),
        _ => println!("There are only 3 tests available")
    }
}
