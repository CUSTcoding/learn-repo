# Aula 01 - Primeiros Passos em Rust

## Objetivos da Aula

Nesta aula, você aprenderá os fundamentos básicos de Rust e configurará seu ambiente para começar a programar.

**Tópicos abordados:**
- Instalação e configuração do Rust
- Setup inicial do projeto
- Estrutura básica de um programa
- Variáveis e tipos primitivos
- Primeiro "Hello, World!"

---

## 1. Instalação e Setup

### Instalar Rust (Linux/macOS/Windows)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verifique a instalação:

```bash
rustc --version
cargo --version
```

### Criar Primeiro Projeto

```bash
cargo new hello_rust
cd hello_rust
```

Estrutura do projeto:

```
hello_rust/
├── Cargo.toml        # Configuração do projeto
├── Cargo.lock        # Dependências bloqueadas
└── src/
    └── main.rs       # Arquivo principal
```

---

## 2. Seu Primeiro Programa

### Hello, World!

Arquivo: `src/main.rs`

```rust
fn main() {
    println!("Hello, world!");
}
```

Executar:

```bash
cargo run
```

Saída esperada:

```
Hello, world!
```

---

## 3. Variáveis e Tipos

### Declarando Variáveis

```rust
fn main() {
    let x = 5;                    // Inferência de tipo
    let y: i32 = 10;              // Tipo explícito
    let mut z = 15;               // Mutável
    
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    
    z = 20;                        // Permitido (mut)
    println!("z alterado = {}", z);
}
```

### Tipos Primitivos

```rust
fn main() {
    // Inteiros
    let a: i32 = -42;            // Inteiro com sinal (32 bits)
    let b: u32 = 42;             // Sem sinal (32 bits)
    let c: i64 = 9223372036854775807; // 64 bits
    
    // Ponto flutuante
    let d: f32 = 3.14;
    let e: f64 = 2.71828;
    
    // Booleano
    let f: bool = true;
    
    // Caractere
    let g: char = 'A';
    
    println!("Integer i32: {}", a);
    println!("Float f64: {}", e);
    println!("Booleano: {}", f);
    println!("Char: {}", g);
}
```

---

## 4. Println! Macro

Imprimir valores na console:

```rust
fn main() {
    let nome = "Rust";
    let versao = 1;
    
    // Placeholders básicos
    println!("Olá, {}!", nome);
    
    // Múltiplos valores
    println!("{} versão {}", nome, versao);
    
    // Debug format
    println!("Debug: {:?}", nome);
}
```

---

## 5. Operações Matemáticas

```rust
fn main() {
    let x = 10;
    let y = 3;
    
    println!("Adição: {}", x + y);        // 13
    println!("Subtração: {}", x - y);     // 7
    println!("Multiplicação: {}", x * y); // 30
    println!("Divisão: {}", x / y);       // 3
    println!("Módulo: {}", x % y);        // 1
}
```

---

## 6. Estrutura do Cargo

### Arquivo: Cargo.toml

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"
authors = ["Seu Nome"]

[dependencies]
# Adicione dependências aqui
```

**edition**: Versão da linguagem Rust
- 2015, 2018, 2021 são as principais

---

## 7. Saídas do Cargo

Executar o programa:

```bash
cargo run
```

Compilar sem executar:

```bash
cargo build
```

Compilar para produção (otimizado):

```bash
cargo build --release
```

Verificar se compila:

```bash
cargo check
```

---

## Exercícios Propostos

### Exercício 1: Convertendo Temperatura

Crie um programa que converta Celsius para Fahrenheit:

```
Fórmula: F = (C × 9/5) + 32
```

### Exercício 2: Cálculo de Área

Calcule a área de um retângulo dadas as dimensões.

### Exercício 3: Tabuada

Imprima a tabuada de um número (1 a 10).

---

## Próximos Tópicos

- Funções e passagem de parâmetros
- Strings e tipos mais complexos
- Controle de fluxo (if, loops)
- Ownership e borrowing
- Estruturas e enums

---

## Recursos Úteis

- [The Rust Book - Chapter 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Rust by Example - Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [Playground Rust Online](https://play.rust-lang.org/)

---

**Última atualização**: Dezembro 2025
