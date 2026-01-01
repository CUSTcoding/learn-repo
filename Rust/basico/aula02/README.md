# 📝 Aula 02 - Variáveis e Mutabilidade em Rust

## O que você vai aprender

1. Como declarar **variáveis** em Rust
2. **Tipagem automática** vs **tipagem explícita**
3. **Imutabilidade** - o padrão do Rust
4. Como usar `mut` para variáveis **mutáveis**
5. **Escopo** de variáveis (scope)
6. O sistema de **ownership** introdutório

---

## 1️⃣ Declarando Variáveis

### Sintaxe Básica

```rust
let nome_da_variavel = valor;
```

### Exemplo Simples

```rust
fn main() {
    let nome = "Custódio";  // String
    let idade = 20;         // Inteiro (i32)
    
    println!("Olá {} você tem {} anos", nome, idade);
}
```

**Saída:**
```
Olá Custódio você tem 20 anos
```

---

## 2️⃣ Tipagem em Rust

### Tipagem Automática (Type Inference)

Rust **adivinha** o tipo pela atribuição:

```rust
fn main() {
    let numero = 42;              // i32 (inteiro de 32 bits)
    let preco = 19.99;            // f64 (float de 64 bits)
    let ativo = true;             // bool
    let letra = 'A';              // char
    let texto = "Rust";           // &str (string literal)
}
```

Rust é **fortemente tipado**, mas faz **type inference**!

### Tipagem Explícita (Type Annotation)

Você pode ser explícito:

```rust
fn main() {
    let numero: i32 = 42;         // Explicitamente i32
    let preco: f64 = 19.99;       // Explicitamente f64
    let nome: &str = "Alice";     // Explicitamente &str
}
```

### Quando usar tipagem explícita?

```rust
// ✅ Bom: Quando pode haver ambiguidade
let numero: i32 = "42".parse().unwrap();

// ✅ Bom: Para documentação
let temperatura: f64 = 36.5;

// ✅ Quando trabalhando com APIs
let resultado: Result<i32, _> = valor.parse();
```

---

## 3️⃣ Imutabilidade - O Padrão do Rust

### ⚠️ Variáveis são IMUTÁVEIS por padrão!

```rust
fn main() {
    let x = 5;
    x = 10;  // ❌ ERRO! Não pode reatribuir
}
```

**Erro de compilação:**
```
error[E0384]: cannot assign twice to immutable variable `x`
```

### Por que imutabilidade é importante?

```
Imutabilidade é SEGURANÇA:
- Código previsível
- Menos bugs
- Segurança em concorrência
- Mais fácil de raciocinar
```

---

## 4️⃣ Tornando Variáveis Mutáveis com `mut`

### Sintaxe

```rust
let mut nome_da_variavel = valor;
```

### Exemplo

```rust
fn main() {
    let mut contador = 0;
    println!("Valor inicial: {}", contador);
    
    contador = 1;  // ✅ OK! Agora posso modificar
    println!("Novo valor: {}", contador);
    
    contador = 2;
    println!("Valor final: {}", contador);
}
```

**Saída:**
```
Valor inicial: 0
Novo valor: 1
Valor final: 2
```

### Boas Práticas

```rust
// ❌ Evite: Tudo mutável
let mut x = 5;
let mut y = 10;

// ✅ Prefira: Apenas o necessário
let x = 5;              // Imutável
let mut contador = 0;   // Mutável quando necessário
```

---

## 5️⃣ Escopo de Variáveis (Scope)

### O que é Escopo?

Escopo é a **região do código** onde uma variável é válida.

```rust
fn main() {  // <-- Escopo externo começa
    let x = 5;
    
    {  // <-- Novo escopo interno
        let y = 10;
        println!("x = {}, y = {}", x, y);  // ✅ OK
    }  // <-- Escopo interno termina, y é destruída
    
    println!("x = {}", x);      // ✅ OK (x ainda existe)
    // println!("y = {}", y);   // ❌ ERRO! y não existe mais
}  // <-- Escopo externo termina, x é destruída
```

### Shadowing (Sombra de Variável)

Você pode redeclarar variáveis no mesmo escopo:

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);  // 5
    
    let x = x + 1;          // Shadowing!
    println!("x = {}", x);  // 6
    
    {
        let x = x * 2;      // Shadowing no escopo interno
        println!("x = {}", x);  // 12
    }
    
    println!("x = {}", x);  // 6 (volta ao escopo anterior)
}
```

**Saída:**
```
x = 5
x = 6
x = 12
x = 6
```

---

## 6️⃣ Regras Importantes sobre Variáveis

### Regra 1: Variáveis sem inicialização

```rust
let x: i32;  // Declarado mas não inicializado
// println!("{}", x);  // ❌ ERRO! Não foi inicializado
x = 5;
println!("{}", x);  // ✅ OK agora
```

### Regra 2: Type mismatch é erro

```rust
let numero: i32 = "cinco";  // ❌ ERRO! "cinco" é string, não i32
```

### Regra 3: Reatribuição com tipo diferente

```rust
fn main() {
    let mut x = 5;      // x é i32
    x = "texto";        // ❌ ERRO! Não pode mudar tipo
}
```

---

## 7️⃣ RAII - Resource Acquisition Is Initialization

Rust segue o princípio RAII:

```rust
fn main() {
    {
        let recurso = "arquivo.txt";  // Recurso adquirido
        // Usar o recurso aqui
    }  // Recurso liberado automaticamente
    // recurso não pode ser usado aqui
}
```

**Benefício:** Sem vazamento de memória! Sem limpeza manual!

---

## 8️⃣ Exemplo Completo Aula 02

```rust
/*
 * AULA 02 - Variáveis e Mutabilidade
 * 
 * Conceitos:
 * 1. Declaração de variáveis
 * 2. Tipagem automática e explícita
 * 3. Imutabilidade (padrão)
 * 4. Mutabilidade com mut
 * 5. Escopo de variáveis
 */

fn main() {
    println!("=== VARIÁVEIS EM RUST ===\n");
    
    // 1. TIPAGEM AUTOMÁTICA
    println!("1. Tipagem Automática:");
    let numero = 42;
    let decimal = 3.14;
    let texto = "Rust";
    println!("  numero = {} (tipo: i32)", numero);
    println!("  decimal = {} (tipo: f64)", decimal);
    println!("  texto = {} (tipo: &str)\n", texto);
    
    // 2. TIPAGEM EXPLÍCITA
    println!("2. Tipagem Explícita:");
    let idade: i32 = 20;
    let altura: f64 = 1.75;
    let nome: &str = "Custódio";
    println!("  nome = {}, idade = {}, altura = {}\n", nome, idade, altura);
    
    // 3. IMUTABILIDADE (padrão)
    println!("3. Imutabilidade (padrão):");
    let x = 5;
    println!("  x = {} (imutável)", x);
    // x = 10;  // ❌ Erro de compilação!
    
    // 4. MUTABILIDADE com mut
    println!("\n4. Mutabilidade com 'mut':");
    let mut y = 5;
    println!("  y inicial = {}", y);
    y = 10;
    println!("  y modificado = {}\n", y);
    
    // 5. ESCOPO DE VARIÁVEIS
    println!("5. Escopo de Variáveis:");
    {
        let variavel_interna = "Só existo aqui";
        println!("  Dentro do escopo: {}", variavel_interna);
    }
    // println!("  {}", variavel_interna);  // ❌ Não existe mais!
    
    // 6. SHADOWING
    println!("\n6. Shadowing (Redeclaração):");
    let cores = 3;
    println!("  cores = {}", cores);
    let cores = cores + 1;
    println!("  cores (shadowing) = {}", cores);
}
```

**Saída esperada:**
```
=== VARIÁVEIS EM RUST ===

1. Tipagem Automática:
  numero = 42 (tipo: i32)
  decimal = 3.14 (tipo: f64)
  texto = Rust (tipo: &str)

2. Tipagem Explícita:
  nome = Custódio, idade = 20, altura = 1.75

3. Imutabilidade (padrão):
  x = 5 (imutável)

4. Mutabilidade com 'mut':
  y inicial = 5
  y modificado = 10

5. Escopo de Variáveis:
  Dentro do escopo: Só existo aqui

6. Shadowing (Redeclaração):
  cores = 3
  cores (shadowing) = 4
```

---

## 9️⃣ Resumo da Aula

### Conceitos Principais

| Conceito | Explicação |
|----------|-----------|
| `let` | Declara variável (imutável por padrão) |
| `mut` | Torna variável mutável |
| Type Inference | Rust adivinha o tipo |
| Type Annotation | Você especifica o tipo: `let x: i32` |
| Escopo | Onde a variável é válida |
| Shadowing | Redeclarar variável no mesmo escopo |

### Regras de Ouro

```
1. Variáveis são imutáveis por padrão
2. Use 'mut' apenas quando necessário
3. Cada variável tem um escopo bem definido
4. Rust é fortemente tipado (mas com type inference)
5. Shadowing é diferente de mutação
```

---



---

## 📚 Próxima Aula

Aula 03 cobrirá **Tipos Primitivos e Data Types** - Entenda os 4 tipos escalares e tipos compostos!

