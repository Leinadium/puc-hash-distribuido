pub mod disthash {
    use std::io::Write;
    use std::net::{Shutdown, TcpStream};
    use std::process::Command;


    pub fn inicia(n: i32, path_para_servidor: &String) {
        /// Inicia os nós. Cada nó é um processo rodando server

        let porta = 7000;
        let no = 0;

        for i in 0..n {
            // iniciando um processo
            Command::new(path_para_servidor)
                .arg(format!("{}", no + i))
                .arg(format!("{}", porta + i))
                .spawn()
                .expect("server failed to start. Maybe check the path");
        }
    }

    pub fn fecha(n: i32) {
        /// Envia um comando de fechar para cada servidor.
        let porta = 7000;
        let no = 0;
        for i in 0..n-1 {
            // envia a mensagem
            let mut stream = match TcpStream::connect(format!("127.0.0.1:{}", porta + i)) {
                Ok(s) => s,
                Err(_) => {println!("error connecting with node {}", no + i); return;}
            };
            stream.write(b"fecha").expect("error writing 'fecha'");
            // a conexao sera fechada pelo proprio servidor
        }
    }

    pub fn insere(no_inicial: i32, chave: &String, valor: &String) {
        /// Abre uma conexão TCP com o respectivo server, enviando uma mensagem contendo a chave e o valor.

        let porta = 7000 + no_inicial;
        let chave_safe = chave.replace("--", "\\--");
        let valor_safe = valor.replace("--", "\\--");

        let mensagem = format!("insere--{}--{}", chave_safe, valor_safe);
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", porta))
            .expect("error connecting with server");

        stream.write(mensagem.as_bytes()).
            expect("error writing to server");
    }

    pub fn consulta(_id: i32, no_inicial: i32, chave: &String, endereco: &String) {
        /// Abre uma conexão TCP com o respectivo server, enviando uma mensagem
        /// contendo o id, chave e endereco de retorno

        // o id nao esta sendo usado por enquanto
        let porta = 7000 + no_inicial;
        let chave_safe = chave.replace("--", "\\--");
        let endereco_safe = endereco.replace("--", "\\--");

        let mensagem = format!("consulta--{}--{}", chave_safe, endereco_safe);
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", porta))
            .expect("error connecting with server");

        stream.write(mensagem.as_bytes()).
            expect("error writing to server");
    }
}


#[cfg(test)]
mod tests {
    use crate::disthash::{inicia, fecha};

    fn teste_inicia() {
        let path = "C:\\Users\\Daniel\\IdeaProjects\\puc-hash-distribuido\\server\\target\\release\\server.exe";
        inicia(1, &path.to_string());
    }

    fn teste_fecha() {
        let path = "C:\\Users\\Daniel\\IdeaProjects\\puc-hash-distribuido\\server\\target\\release\\server.exe";
        inicia(1, &path.to_string());
        fecha(1);
    }
}