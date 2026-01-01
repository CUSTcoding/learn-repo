# 🔢 Aula 03 - Tipos Primitivos e Data Types em Rust

## O que você vai aprender

1. **Tipos Escalares** (Scalar Types) - Armazenam um valor
2. **Tipos Compostos** (Compound Types) - Armazenam múltiplos valores
3. **Inteiros** (Integer Types) - Signed e Unsigned
4. **Floats** (Floating-Point) - Números decimais
5. **Booleanos** (Boolean)
6. **Caracteres** (Character)
7. **Tuplas** (Tuples)
8. **Arrays** (Arrays)

---

## 1️⃣ Estrutura de Tipos em Rust

```
┌─────────────────────────────────┐
│   TIPOS DE DADOS EM RUST        │
├─────────────────────────────────┤
│  ESCALARES (1 valor)            │
│  ├─ Inteiros (i8, i16, i32...)  │
│  ├─ Floats (f32, f64)           │
│  ├─ Booleanos (bool)            │
│  └─ Caracteres (char)           │
│                                 │
│  COMPOSTOS (múltiplos valores)  │
│  ├─ Tuplas (tuple)              │
│  └─ Arrays (array)              │
└─────────────────────────────────┘
```

---

## 2️⃣ Tipos Escalares - Inteiros

### O que é um Inteiro?

Números sem decimal. Representa conjuntos matemáticos:
- **N** (Naturais): 0, 1, 2, 3...
- **Z** (Inteiros): -3, -2, -1, 0, 1, 2, 3...

### Dois Tipos de Inteiros

#### A. Signed (Com Sinal) - Podem ser negativos

| Bits | Tipo | Range |
|------|------|-------|
| 8 | `i8` | -128 a 127 |
| 16 | `i16` | -32,768 a 32,767 |
| 32 | `i32` | -2,147,483,648 a 2,147,483,647 |
| 64 | `i64` | -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807 |
| 128 | `i128` | Muito grande! |
| arch | `isize` | Depende do sistema (32/64 bits) |

**Fórmula de Range Signed:**
$$-2^{n-1} \text{ a } 2^{n-1} - 1$$

**Exemplo i8:**
$$-2^7 \text{ a } 2^7 - 1 = -128 \text{ a } 127$$

#### B. Unsigned (Sem Sinal) - Apenas positivos

| Bits | Tipo | Range |
|------|------|-------|
| 8 | `u8` | 0 a 255 |
| 16 | `u16` | 0 a 65,535 |
| 32 | `u32` | 0 a 4,294,967,295 |
| 64 | `u64` | 0 a 18,446,744,073,709,551,615 |
| 128 | `u128` | Muito grande! |
| arch | `usize` | Depende do sistema |

**Fórmula de Range Unsigned:**
$$0 \text{ a } 2^n - 1$$

**Exemplo u8:**
$$0 \text{ a } 2^8 - 1 = 0 \text{ a } 255$$

### Por que múltiplos tipos?

```
✅ Economia de memória
   - u8 usa 1 byte (0-255)
   - u64 usa 8 bytes (muito maior)
   
✅ Performance
   - Operações mais rápidas com tipos menores
   
✅ Segurança
   - Previne overflow/underflow
```

### Exemplos Práticos

```rust
fn main() {
    // Inteiros positivos e negativos
    let numero_positivo: i32 = 42;
    let numero_negativo: i32 = -17;
    
    // Apenas positivos (mais eficiente)
    let idade: u8 = 25;  // 0 a 255 - perfeito para idade!
    let quantidade: u32 = 1_000_000;  // Underscore para legibilidade
    
    // Valores padrão (32 bits em máquinas modernas)
    let numero = 100;  // Type inference → i32
    
    println!("Positivo: {}", numero_positivo);
    println!("Negativo: {}", numero_negativo);
    println!("Idade: {}", idade);
    println!("Quantidade: {}", quantidade);
}
```

---

## 3️⃣ Tipos Escalares - Floats

### O que é Float?

Números com decimal. Representa conjunto **R** (Reais):
- -3.14, -2.5, 0.0, 1.5, 2.71...

### Dois Tipos de Floats

| Tipo | Bits | Precisão | Uso |
|------|------|----------|-----|
| `f32` | 32 | ~6 dígitos | Quando memória é crítica |
| `f64` | 64 | ~15 dígitos | Padrão, melhor precisão |

**Tipo padrão:** `f64`

### Exemplos

```rust
fn main() {
    // Type inference → f64 por padrão
    let pi = 3.14159;
    let altura = 1.75;
    
    // Tipo explícito
    let temp: f32 = 36.5;
    let preco: f64 = 99.99;
    
    // Notação científica
    let grande = 1.5e10;  // 15,000,000,000
    let pequeno = 2.5e-3; // 0.0025
    
    println!("π ≈ {}", pi);
    println!("Altura: {}m", altura);
    println!("Temperatura: {}°C", temp);
    println!("Preço: R${}", preco);
}
```

### ⚠️ Cuidado com Precisão

```rust
fn main() {
    let x = 0.1 + 0.2;
    println!("{}", x);  // 0.30000000000000004 (não 0.3!)
    
    // Por isso bancos usam valores inteiros (centavos)
    let centavos: i64 = 30;  // 0.30 reais
}
```

---

## 4️⃣ Tipos Escalares - Booleanos

### O que é Boolean?

Tipo que pode ser apenas **true** ou **false**.

```rust
fn main() {
    let verdadeiro = true;
    let falso: bool = false;
    
    println!("Verdadeiro: {}", verdadeiro);
    println!("Falso: {}", falso);
    
    // Resultado de comparações
    let dez_maior_que_cinco = 10 > 5;  // true
    println!("10 > 5? {}", dez_maior_que_cinco);
}
```

---

## 5️⃣ Tipos Escalares - Char

### O que é Char?

Um **único caractere** (letra, número, símbolo, emoji).

**Diferença importante:**
```
'A'       → Char (aspas simples)
"Hello"   → String (aspas duplas)
```

### Exemplos

```rust
fn main() {
    let letra = 'A';
    let numero_char = '5';
    let simbolo = '♣';
    let emoji = '🦀';
    
    println!("Letra: {}", letra);
    println!("Número como char: {}", numero_char);
    println!("Símbolo: {}", simbolo);
    println!("Emoji: {}", emoji);  // Rust suporta Unicode!
}
```

---

## 6️⃣ Tipos Compostos - Tuplas

### O que é Tupla?

Coleção de valores de **tipos diferentes** e **tamanho fixo**.

### Sintaxe

```rust
let tupla: (tipo1, tipo2, tipo3) = (valor1, valor2, valor3);
```

### Exemplo

```rust
fn main() {
    // Tupla com 4 elementos de tipos diferentes
    let dados: (i32, f64, &str, bool) = (42, 3.14, "Rust", true);
    
    // Type inference
    let pessoa = (25, "Alice", 1.75);
    
    println!("Tupla: {:?}", dados);  // {:?} para debug
}
```

**Saída:**
```
Tupla: (42, 3.14, "Rust", true)
```

### Acessando Elementos

```rust
fn main() {
    let pessoa = (25, "Alice", 1.75);
    
    // Acesso por índice
    let idade = pessoa.0;
    let nome = pessoa.1;
    let altura = pessoa.2;
    
    println!("Nome: {}", nome);
    println!("Idade: {}", idade);
    println!("Altura: {}", altura);
}
```

### Destructuring (Desestruturação)

```rust
fn main() {
    let pessoa = (25, "Alice", 1.75);
    
    // Desestruturar em variáveis
    let (idade, nome, altura) = pessoa;
    
    println!("Nome: {}, Idade: {}, Altura: {}", nome, idade, altura);
}
```

---

## 7️⃣ Tipos Compostos - Arrays

### O que é Array?

Coleção de valores do **mesmo tipo** e **tamanho fixo**.

### Sintaxe

```rust
let array: [tipo; tamanho] = [valor1, valor2, ...];
```

### Exemplos

```rust
fn main() {
    // Array de 5 inteiros
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Type inference
    let cores = ["vermelho", "verde", "azul"];
    
    // Todos iguais
    let zeros: [i32; 5] = [0; 5];  // [0, 0, 0, 0, 0]
    
    println!("Números: {:?}", numeros);
    println!("Cores: {:?}", cores);
    println!("Zeros: {:?}", zeros);
}
```

**Saída:**
```
Números: [1, 2, 3, 4, 5]
Cores: ["vermelho", "verde", "azul"]
Zeros: [0, 0, 0, 0, 0]
```

### Acessando Elementos

```rust
fn main() {
    let numeros = [10, 20, 30, 40, 50];
    
    // Acesso por índice (começa em 0)
    println!("Primeiro: {}", numeros[0]);   // 10
    println!("Terceiro: {}", numeros[2]);   // 30
    println!("Último: {}", numeros[4]);     // 50
    
    // Iterar sobre elementos
    for numero in &numeros {
        println!("Número: {}", numero);
    }
}
```

### ⚠️ Array vs String

```rust
fn main() {
    // Array (coleção fixa)
    let array: [i32; 3] = [1, 2, 3];
    
    // String é diferente!
    let texto = "Rust";  // &str (string slice)
    let string = String::from("Rust");  // String (alocada em heap)
}
```

---

## 8️⃣ Exemplo Completo Aula 03

```rust
/*
 * AULA 03 - Tipos Primitivos e Data Types em Rust
 * 
 * Categorias:
 * 1. ESCALARES: Inteiros, Floats, Booleanos, Chars
 * 2. COMPOSTOS: Tuplas, Arrays
 */

fn main() {
    println!("=== TIPOS PRIMITIVOS EM RUST ===\n");
    
    // ===== 1. INTEIROS SIGNED =====
    println!("1. INTEIROS SIGNED (com sinal):");
    let idade: i8 = 25;           // 8 bits: -128 a 127
    let temperatura: i16 = -15;   // 16 bits
    let numero: i32 = 42;         // 32 bits (tipo padrão)
    let grande: i64 = 9_223_372_036_854_775_807;  // 64 bits
    println!("  i8: {}, i16: {}, i32: {}, i64: {}\n", idade, temperatura, numero, grande);
    
    // ===== 2. INTEIROS UNSIGNED =====
    println!("2. INTEIROS UNSIGNED (sem sinal):");
    let u8_max: u8 = 255;         // 8 bits: 0 a 255
    let u16_max: u16 = 65535;     // 16 bits
    let u32_max: u32 = 4_294_967_295;  // 32 bits
    println!("  u8_max: {}, u16_max: {}, u32_max: {}\n", u8_max, u16_max, u32_max);
    
    // ===== 3. FLOATS =====
    println!("3. FLOATS (números decimais):");
    let pi: f32 = 3.14;
    let altura: f64 = 1.75;       // Tipo padrão
    let pequeno = 2.5e-3;         // 0.0025
    println!("  f32: {}, f64: {}, científica: {}\n", pi, altura, pequeno);
    
    // ===== 4. BOOLEANOS =====
    println!("4. BOOLEANOS:");
    let ativo: bool = true;
    let completado = false;
    let maior = 10 > 5;
    println!("  true: {}, false: {}, 10>5: {}\n", ativo, completado, maior);
    
    // ===== 5. CARACTERES =====
    println!("5. CARACTERES (char):");
    let letra = 'A';
    let numero_char = '5';
    let emoji = '🦀';
    println!("  Letra: {}, Número: {}, Emoji: {}\n", letra, numero_char, emoji);
    
    // ===== 6. TUPLAS =====
    println!("6. TUPLAS (múltiplos tipos, tamanho fixo):");
    let pessoa: (i32, &str, f64) = (25, "Alice", 1.75);
    let (age, name, height) = pessoa;  // Desestruturação
    println!("  Pessoa: {} anos, {}, {}m", age, name, height);
    println!("  Acesso direto pessoa.0: {}\n", pessoa.0);
    
    // ===== 7. ARRAYS =====
    println!("7. ARRAYS (mesmo tipo, tamanho fixo):");
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
    let cores = ["vermelho", "verde", "azul"];
    let zeros = [0; 3];  // [0, 0, 0]
    println!("  Números: {:?}", numeros);
    println!("  Cores: {:?}", cores);
    println!("  Zeros: {:?}\n", zeros);
    
    // ===== 8. EXEMPLO PRÁTICO =====
    println!("8. EXEMPLO PRÁTICO - Dados de Desenvolvedor:");
    
    let dev_info: (i32, &str, f64, bool) = (28, "Carlos", 1.80, true);
    let (years, name, height, active) = dev_info;
    
    println!("  Nome: {}", name);
    println!("  Idade: {} anos", years);
    println!("  Altura: {} m", height);
    println!("  Ativo: {}\n", active);
    
    // ===== 9. TABELA DE DADOS =====
    println!("9. ARRAY COM DADOS:");
    let linguagens = ["Rust", "Python", "JavaScript", "C++"];
    println!("  Linguagens aprendidas:");
    for (i, lang) in linguagens.iter().enumerate() {
        println!("    {}. {}", i + 1, lang);
    }
    
    println!("\n=== FIM DA AULA ===");
}
```

**Saída esperada:**
```
=== TIPOS PRIMITIVOS EM RUST ===

1. INTEIROS SIGNED (com sinal):
  i8: 25, i16: -15, i32: 42, i64: 9223372036854775807

2. INTEIROS UNSIGNED (sem sinal):
  u8_max: 255, u16_max: 65535, u32_max: 4294967295

3. FLOATS (números decimais):
  f32: 3.14, f64: 1.75, científica: 0.0025

4. BOOLEANOS:
  true: true, false: false, 10>5: true

5. CARACTERES (char):
  Letra: A, Número: 5, Emoji: 🦀

6. TUPLAS (múltiplos tipos, tamanho fixo):
  Pessoa: 25 anos, Alice, 1.75m
  Acesso direto pessoa.0: 25

7. ARRAYS (mesmo tipo, tamanho fixo):
  Números: [1, 2, 3, 4, 5]
  Cores: ["vermelho", "verde", "azul"]
  Zeros: [0, 0, 0]

8. EXEMPLO PRÁTICO - Dados de Desenvolvedor:
  Nome: Carlos
  Idade: 28 anos
  Altura: 1.8 m
  Ativo: true

9. ARRAY COM DADOS:
  Linguagens aprendidas:
    1. Rust
    2. Python
    3. JavaScript
    4. C++

=== FIM DA AULA ===
```

---

## 9️⃣ Tabela de Resumo - Tipos Escalares

| Categoria | Tipo | Tamanho | Range | Uso |
|-----------|------|---------|-------|-----|
| **Inteiro** | i32 | 4 bytes | -2B a 2B | Padrão |
| | i64 | 8 bytes | Muito grande | Números grandes |
| | u8 | 1 byte | 0-255 | Bytes, índices |
| **Float** | f64 | 8 bytes | IEEE 754 | Padrão |
| | f32 | 4 bytes | IEEE 754 | Pouca memória |
| **Bool** | bool | 1 byte | true/false | Lógica |
| **Char** | char | 4 bytes | Unicode | Um caractere |

---

## 🔟 Resumo Final

### Tipos Escalares (1 valor)

```rust
let numero: i32 = 42;       // Inteiro
let decimal: f64 = 3.14;    // Float
let ativo: bool = true;     // Booleano
let letra: char = 'A';      // Caractere
```

### Tipos Compostos (múltiplos valores)

```rust
let tupla = (42, "Alice", 1.75);      // Tipos diferentes
let array = [1, 2, 3, 4, 5];          // Mesmo tipo
```

---

## 🎓 Exercícios

1. **Básico**: Crie variáveis de todos os 4 tipos escalares e imprima
2. **Intermediário**: Crie uma tupla com seus dados pessoais
3. **Desafio**: Crie um array com notas e calcule a média

---

## 📚 Próxima Aula

Aula 04 cobrirá **Funções e Controle de Fluxo** - if/else, loops, e funções personalizadas!

