# Hash Distribuido

* Alexandre Heine
* Daniel Guimarães


## Conteúdo

### Servidor (server)
O server representa um nó do anel. Ou seja, o anel completo possui vários processos
do servidor. A interação dele é por meio de conexões TCP.

O servidor recebe três argumentos: o numero do seu nó, sua porta, e o total de nós na árvore

### Cliente (client)
O client é um processo simples, que faz poucas operações. Ele simula uma consulta ou inserção no anel.
Ele também foi implementado para rodar os comandos de iniciar e fechar o anel, apesar de não precisa do Rust para isso,
pois a iniciação é feita rodando o server.exe varias vezes, e o fechamento é mandando
mensagens TCP para cada servidor, mandando cada um fechar.

O client também usa uma API, disponível em ```client/api/src/lib.rs``` , com as funções 
solicitadas.

O client recebe até três argumentos, na forma: 
```text
    fechar q_nos  -> fecha q nós do anel (de preferencia, todos)
    iniciar q_nos   -> cria um anel com q nós
    insere no chave valor -> insere a chave e valor começando no nó especificado
    consutla no chave -> consulta a chave começando no nó especificado
```

## Execução

Primeiro, compile o servidor para um executável:

```bash
cd server
cargo build --release
```

Esse comando deve gerar um executável em ```/server/target/release/``` . 
Guarde o path desse executável