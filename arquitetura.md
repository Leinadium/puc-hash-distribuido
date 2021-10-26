# Arquitetura do Projeto

## Funções da biblioteca

### Função *inicia(n)* :
    Cria n instâncias do servidor.
    O servidor i ouvirá na porta 5000 + i.
    Cada servidor representa um nó.
    Cada servidor é um processo do server

### Função *fecha(n)* :
    Fecha os n servidores abertos.
    Tenta abrir uma conexão TCP com cada server em suas respectivas portas,
        enviando uma mensagem de fechar.

### Função *insere(no_inicial, chave, valor)* :
    Abre uma conexão TCP com o respectivo server, enviando uma mensagem contendo a chave e o valor.
    A mensagem enviada deve escapar qualquer "--" contido na string da chave e valor.
    Assim que confirmar o recebimento, fecha a conexão.

### Função *consulta(id, no_inicial, chave, endereço)* :
    Abre uma conexão TCP com o respectivo server, enviando uma mensagen contendo 
        o id, chave e endereço de retorno.
    A mensagem enviada deve escapar qualquer "--" contido na string da chave e valor.

## Funcionalidade do servidor

    Para cada nova conexão TCP:
        pega mensagem e fecha conexão
        inicia thread da função trata(mensagem)

### Função *trata(mensagem)*:
    Trata a mensagem, executando as funções abaixo para cada caso:
        "insere--chave--valor" -> chama [calcula], e depois [roteia] ou [coloca]
        "insere_no--chave--valor--no" -> chama [roteia] ou [coloca]
        "consulta--valor--endereço" -> chama [calcula], e depois [roteia] ou [procura]
        "consulta_no--valor--no--endereco" -> chama [roteia] ou [procura]

### Função *calcula(chave)*:
    Calcula o nó destino para a chave, ou seja, se deve rotear a chave ou usar no próprio nó;

### Função *roteia(no, chave, tipo, valor_ou_endereço)*:
    Define qual nó mais próximo receberá a mensagem.
    Faz uma conexão TCP com o nó e envia a mensagem apropriada (de acordo com o tipo, que é
        consulta/insere).

### Função *procura(chave, endereço)*:
    Sobe semáforo de exclusão
    Procura a chave na hashlist
    Se a chave não existe, coloca a chave e endereço em uma lista de espera 
    Desce semáforo de exclusão
    Se encontrou a chave, chama [callback]

### Função *coloca(chave, valor)*:
    Sobe semáforo de exclusão
    Coloca a chave e seu valor na hashlist
    Desce semáforo de exclusão
    Dá uma olhada na lista de espera. Se a chave colocada estiver na lista de espera,
        chama [callback] para ela

### Função *callback(valor, endereço)*: 
    Envia para o endereço o valor encontrado, por TCP

## Exemplos

* *Inserção no servidor*
```text
...
no_destino = calcula(chave);
if (no_destino == no_atual):
    coloca(chave, endereço);
else:
    roteia(no_destino, chave, "insere", endereço);
endif
```

* *Consulta no servidor*
```text
...
no_destino = calcula(chave);
if (no_destino == no atual):
    procura(chave, endereço);
else:
    roteia(no_destino, chave, "consulta", endereço);
endif
```

