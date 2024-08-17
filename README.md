# Game of life with websockets 

![GitHub repo size](https://img.shields.io/github/repo-size/RaoniSilvestre/game-of-life?style=for-the-badge)
![GitHub language count](https://img.shields.io/github/languages/count/?style=for-the-badge)

<img src="" alt="Exemplo de funcionamento">

> Utilização de websockets para enviar sinais do servidor com estados do game-of-life para clientes que exibem o estado atual do jogo.

### Ajustes e melhorias

O projeto está num bom estado, mas caso eu ainda volte no futuro, os planos de melhoria são:

- [ ] Adicionar estado inicial do jogo a partir de um arquivo
- [ ] Permitir alteração do estado por meio do teclado enquanto o jogo funciona

## 💻 Pré-requisitos

Antes de começar, verifique se você atendeu aos seguintes requisitos:

- Você instalou a versão mais recente do [rust](https://www.rust-lang.org/tools/install) . 
- Você tem uma máquina linux. (Não testei em outros ambientes) 

## Rodando o projeto

Para rodar o projeto apenas localmente, rode o seguinte comando:

```
cargo run --bin local
```

Para rodar no modo de websocket, abra dois terminais, no primeiro, rode:

```
cargo run --bin client
```

E no segundo:

```
cargo run --bin server
```


## Parâmetros opcionais

Esses parâmetros apenas funcionam nos binários local e server.

```bash
$ cargo run
Usage: local [OPTIONS]

Options:
  -m, --mode <MODE>          Modo de inicialização do jogo [default: random] [possible values: test, random]
  -x, --dx <DX>              Tamanho do eixo X do jogo [default: 40]
  -y, --dy <DY>              Tamanho do eixo Y do jogo [default: 20]
  -f, --fps <FPS>            Frames por segundo [default: 1]
  -r, --random <RANDOM>      Quantidade de bolinhas aleatórias, usado apenas no modo "random" [default: 200]
  -c, --def-char <DEF_CHAR>  Caractere que apareçerá na tela como célula viva [default: #]
  -d, --duration <DURATION>  Duração do jogo
  -h, --help                 Print help (see more with '--help')
  -V, --version              Print version
```

