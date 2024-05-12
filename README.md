# Documentação do Projeto DNS Client em Rust

## Introdução

Este documento fornece uma visão geral do projeto DNS Client em Rust. O objetivo do projeto é criar um cliente DNS capaz de enviar consultas para servidores DNS e interpretar as respostas recebidas. O projeto consiste em dois módulos principais: o módulo `main`, responsável pela execução do cliente DNS, e o módulo `parser`, responsável por analisar as respostas DNS recebidas dos servidores.

## Módulos

### 1. Main

O módulo `main` é responsável por coordenar as operações do cliente DNS. Ele lida com a entrada do usuário, envia consultas DNS para servidores especificados e processa as respostas recebidas. Este módulo contém a lógica principal do programa.

### 2. Parser

O módulo `parser` é responsável por analisar as respostas DNS recebidas dos servidores. Ele interpreta as respostas recebidas das consultas DNS e extrai as informações relevantes das respostas.

Substitua `<endereco_dns>` pelo endereço IP do servidor DNS a ser consultado e `<nome_do_dominio>` pelo nome do domínio que deseja procurar para obter as informações desejadas.

## Como executar o projeto

Para rodar o projeto, é necessário ter o Rust e o cargo instalados na máquina, para instalar as tecnologias, é necessário seguir o guia de instalação do próprio Rust, segue link:

https://doc.rust-lang.org/cargo/getting-started/installation.html

Para rodar o programa para checar o programa podemos rodar (etapa opcional):

```
cargo check
```

E por fim rodar o programa enviando os argumentos desejados:

```
cargo run <nome_do_dominio> <endereco_dns>
```

## Exemplo de Uso

Suponha que você deseja consultar os servidores DNS para obter os registros NS (servidores de nomes) de um determinado domínio, por exemplo, `example.com`. Você pode executar o cliente DNS da seguinte maneira:

```
cargo run example.com 8.8.8.8
```

Isso enviará uma consulta DNS para o servidor DNS especificado (`8.8.8.8`) para o domínio `example.com`. O cliente processará a resposta recebida e exibirá os registros NS associados ao domínio.

## Resultado esperado

É esperado o resultado da consulta DNS caso seja feita com sucesso, ou outra mensagem de erro de acordo com a mensagem recebida na consulta DNS.

## Conclusão

O projeto DNS Client em Rust oferece uma maneira de enviar consultas DNS e interpretar as respostas recebidas. Pode ser utilizado como meio de aprendizado das consultas DNS, pois não utiliza de nenhuma biblioteca de rust para fazer o parser do recebimento do cabeçalho e dados do DNS.

## Referências Bibliográficas

- https://datatracker.ietf.org/doc/html/rfc1034
- https://datatracker.ietf.org/doc/html/rfc1035
