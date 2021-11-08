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

O client também usa uma API, disponível em ```client/api/src/lib.rs``` , com as funções 
solicitadas.

A API possui as funções de iniciar e fechar os nós. Porém, decidimos não utilizar a função de criar
o anel por um client, pois para depois testar, esse cliente precisaria ficar aberto (pois os nós seriam 
processos filhos desse cliente). Mas foi utilizada a função de fechar a árvore pois é só um comando.

O client recebe até três argumentos, na forma: 
```text
    fechar q_nos  -> fecha q nós do anel (de preferencia, todos)
    insere no chave valor -> insere a chave e valor começando no nó especificado
    consulta no chave -> consulta a chave começando no nó especificado
```

## Execução

### Compilações 

Primeiro, compile o servidor para um executável:

```bash
cd server
cargo build --release
```

Esse comando deve gerar um executável em ```/server/target/release/``` . 

Depois, compile o cliente para um executável:

```bash
cd ../client
cargo build --release
```

Esse comando deve gerar um executável em ```client/target/release```

### Execuções

Para rodar um nó do servidor, execute server passando como argumentos qual o seu nó, sua porta, e quantos nós
existem na rede: 
```bash
./server 1 7001 32
```

Apesar de poder colocar a porta que quiser, os nós se comunicaram um com os outros esperam que cada nó esteja na porta (7000 + node_number)

Para rodar um client, execute client passando como argumentos uma das três possíveis combinações, explicadas acima.

## Testes

Para executar o script de testes, execute o script em código rust do diretório tests pelo comando:
```bash
cd tests
cargo build --release
./tests 1
./tests 2
./tests 3
```
Cada run é seguido pelo testcase.
Testamos os casos:
1. Em que são criados 16 nós, então insere e consulta.
2. Em que são criados 16 nós, então tenta consultar e insere.
3. Em que são criados 32 nós, então insere e consulta desordenadamente concorrentemente.