# Introdução ao Rust - Primeiros Passos

## O que é Rust?

Rust é uma linguagem de programação de sistemas que oferece três garantias principais:

- **Segurança**: Sem vazamentos de memória, sem acessos nulos perigosos
- **Performance**: Tão rápida quanto C/C++, sem garbage collector
- **Concorrência**: Garante thread-safety em tempo de compilação

---

## Por que aprender Rust?

### 1. Segurança de Memória
- Rust impede uma classe inteira de bugs em tempo de compilação
- Sem buffer overflows, use-after-free ou null pointers perigosos
- O compilador garante que código compilado é seguro

### 2. Performance Extrema
- Compilado em código nativo
- Zero-cost abstractions
- Comparável ao C/C++ em eficiência

### 3. Concorrência Segura
- Rust garante thread-safety em tempo de compilação
- Problema da maioria das linguagens resolvido automaticamente

### 4. Versatilidade
- Sistemas embarcados e IoT
- Blockchain e criptografia
- Web e CLI
- Desktops e aplicações nativas

---

## Características Principais

### Ownership System

O sistema de propriedade de Rust é único e resolve problema de gerenciamento de memória:

```
Cada valor tem um proprietário
Quando o proprietário sai de escopo, o valor é liberado
Sem garbage collector, sem vazamento de memória
```

### Borrow Checker

Sistema de empréstimos que permite compartilhar dados com segurança:

```
Você pode ter múltiplas leituras OU uma escrita
Nunca ambas ao mesmo tempo
Validado em tempo de compilação
```

### Type System Forte

Tipagem estática que captura erros em tempo de compilação:

```
Sem type coercion implícita
Conversões explícitas obrigatórias
Máxima segurança de tipo
```

---

## Vantagens

- Segurança garantida em tempo de compilação
- Performance comparável a C/C++
- Excelente tratamento de erros
- Documentação de qualidade
- Comunidade crescente e acolhedora
- Ecossistema maduro (crates.io)

---

## Desvantagens

- Curva de aprendizado acentuada
- Tempo de compilação maior
- Borrow checker pode ser restritivo inicialmente
- Menor ecossistema que Python/JavaScript
- Desenvolvimento mais lento nos primeiros projetos

---

## Quando usar Rust?

Use Rust quando:
- Performance crítica é necessária
- Segurança de memória é essencial
- Sistemas embarcados/low-level
- Blockchain e criptografia
- CLI tools performáticos

Evite Rust quando:
- Prototipagem rápida é prioridade
- Projeto é muito simples (Python/Go seriam melhores)
- Equipe inexperiente em linguagens compiladas

---

## História Resumida

- 2006: Graydon Hoare começou o projeto na Mozilla
- 2010: Mozilla adotou Rust oficialmente
- 2015: Rust 1.0 - Primeira versão estável
- 2020: Adoção em grandes empresas
- 2023: Rust entra no Linux kernel
- 2024: Rust se torna mainstream

---

## O que vem a seguir?

Neste módulo básico, você aprenderá:

1. Instalação e primeiros passos
2. Variáveis e tipos primitivos
3. Funções e escopo
4. Controle de fluxo
5. Ownership e borrowing
6. Strings e coleções

Cada tópico tem exercícios práticos.

---

## Recursos Úteis

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Playground Rust Online](https://play.rust-lang.org/)
- [Crates.io - Repositório de Pacotes](https://crates.io/)

---

**Última atualização**: Dezembro 2025

