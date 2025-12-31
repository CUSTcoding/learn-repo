# Rust - Aprendizado e Domínio

## Objetivo do Aprendizado

Rust é uma linguagem de sistemas compilada que oferece segurança de memória sem garbage collector, combinando segurança e performance. Estou estudando Rust com o objetivo de:

- **Desenvolvimento de Sistemas**: Criar aplicações de performance crítica e baixo nível
- **Segurança de Memória**: Garantir código seguro sem vazamentos de memória em tempo de compilação
- **WebAssembly (WASM)**: Executar código de performance crítica nos navegadores
- **Bitcoin**: Dominar linguagem usada em desenvolvimento Bitcoin
- **Ferramentas de Alta Performance**: Criar CLIs, servidores e utilitários ultra-rápidos
- **Alternativa a C/C++**: Substituir C/C++ com segurança de memória

**Foco Principal**: Sistemas, Bitcoin developer, WebAssembly 

---

## Principais Áreas de Estudo

### 1. Básico (`/basico`)
Fundamentos essenciais de Rust

**Tópicos**:
- Instalação e setup (rustup, cargo)
- Sintaxe básica e estrutura
- Tipos primitivos (i32, u64, f64, bool, char)
- Variáveis e mutabilidade
- Ownership, borrowing e lifetime
- References (& e &mut)
- Pattern matching
- Structs e enums
- Métodos e funções
- Error handling (Result, Option)
- Modules e organização
- Cargo e gerenciamento de dependências

**Aplicação Prática**:
- Programs console e scripts Rust
- Compreender system-level thinking
- Garantir segurança de memória em compile-time
- Base para projetos mais complexos

---

### 2. Estruturas de Dados (`/estrutura_de_dados`)
Implementação de estruturas clássicas em Rust

**Tópicos**:
- Vec (vetores dinâmicos)
- Arrays e slices
- HashMap e BTreeMap
- HashSet e BTreeSet
- Linked Lists (com lifetime)
- Binary Trees
- Graphs e algoritmos
- Iterators e lazy evaluation
- Análise de complexidade
- Memory layout e optimization

**Aplicação Prática**:
- Estruturas de dados memory-safe
- Performance extrema em algoritmos
- Otimização de memória
- Implementações seguras de coleções

---

### 3. Programação Orientada a Objetos (OOP) (`/oop`)
Padrões OOP em Rust (orientado a traits)

**Tópicos**:
- Structs e composição
- Traits e polimorfismo
- Encapsulamento e modularização
- Associated types
- Trait objects e dynamic dispatch
- Herança via traits
- Design Patterns em Rust
- SOLID principles adaptados
- Generics e type parameters
- Lifetime annotations avançadas

**Aplicação Prática**:
- Código flexível e reutilizável
- Abstrações seguras
- Composição sobre herança
- Padrões de design memory-safe

---

### 4. Frameworks (`/framework`)

#### **Axum** (`/framework/axum`)
Framework web moderno e ultra-rápido para Rust

**Tópicos**:
- Routing e handlers
- Extração de requisições
- Middleware e layers
- JSON e serialização
- Error handling em HTTP
- Testing de APIs
- Performance e benchmarking
- Integração com databases

**Aplicação Prática**:
- Criar APIs REST de alta performance
- Servidores web escaláveis
- Microserviços em Rust
- Backend ultra-rápido

#### **Bitcoin Dev Kit** (`/framework/bitcoin_dev_kit`)
Kit de desenvolvimento para aplicações Bitcoin

**Tópicos**:
- Criptografia e ECDSA
- Transações Bitcoin
- Carteiras digitais
- Scripts e smart contracts Bitcoin
- Redes blockchain
- Synchronização e SPV

**Aplicação Prática**:
- Desenvolvimento de wallets Bitcoin
- Aplicações blockchain
- Trading e exchanges
- DeFi e fintech

---

## Relacionamento com Outras Linguagens

### **Rust vs C/C++** (Direto Competidor)
```
C/C++ (tradicional, manual memory management)
         ↓
    Rust (segurança + performance)
```

- **Rust**: Segurança de memória garantida em compiletime
- **C/C++**: Controle total mas risco de bugs de memória
- **Rust é o futuro**: Linux kernel começou a aceitar Rust

### **Rust ↔ Python** (Complementários)
- Python para prototipagem rápida
- Rust para performance crítica


### **Rust → WebAssembly** (Conexão Natural)
- Rust compila excelentemente para WASM
- Executar código Rust em navegadores
- Performance crítica no frontend

### **Rust ↔ Bitcoin** (Ecossistema)

- Bitcoin tools frequentemente em Rust


---

## Motivação Pessoal

Estou estudando Rust porque:

1. **Performance Extrema**: Sem garbage collection, tão rápido quanto C/C++ mas seguro
2. **Segurança Garantida**: Compiler garante memory safety, eliminando classes inteiras de bugs
3. **Bitcoin**: Rust é linguagem preferida para desenvolvimento Bitcoin
4. **WebAssembly**: Rust é primeira opção para WASM, representando 70%+ do uso
5. **Carreira em Sistemas**: Alternativa moderna para quem quer trabalhar com sistemas
6. **Ferramentas Modernas**: Rust está substituindo ferramentas legadas (ripgrep, exa, etc.)
7. **Comunidade em Crescimento**: Crescimento exponencial de oportunidades e recursos
8. **Desafio Intelectual**: Aprender Rust torna melhor desenvolvedor em qualquer linguagem

**Meta**: Dominar Rust para desenvolvimento de sistemas de performance crítica, aplicações Bitcoin e WebAssembly, sendo capaz de escrever código seguro e ultra-performático.

---

## Estrutura de Pastas

```
Rust/
├── README.md (este arquivo)
├── basico/
│   ├── README.md
│   └── introducao/
│       └── README.md
├── estrutura_de_dados/
│   └── (implementações memory-safe)
├── oop/
│   └── (traits e padrões)
└── framework/
    ├── axum/
    │   └── (projetos web APIs)
    └── bitcoin_dev_kit/
        └── (projetos blockchain)
```



## Recursos Úteis

- [The Rust Book (Oficial)](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings (Interactive Exercises)](https://github.com/rust-lang/rustlings)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Rust Patterns](https://rust-lang.github.io/api-guidelines/)

