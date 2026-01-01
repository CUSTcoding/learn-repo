# Aula 04: Operadores em Rust

## Introdução

Operadores são símbolos que realizam operações sobre valores e variáveis. Nesta aula, exploraremos os principais tipos de operadores em Rust e como eles diferem de linguagens como Java, C e C++.

---

## 1. Operadores Aritméticos

Os operadores aritméticos realizam cálculos matemáticos básicos.

### Conceito Teórico

Operadores aritméticos são fundamentais em qualquer linguagem de programação. Eles permitem realizar cálculos como adição, subtração, multiplicação, divisão e obtenção do resto.

### Operadores Disponíveis

| Operador | Nome | Exemplo | Resultado |
|----------|------|---------|-----------|
| `+` | Adição | `10 + 30` | `40` |
| `-` | Subtração | `30 - 10` | `20` |
| `*` | Multiplicação | `10 * 2` | `20` |
| `/` | Divisão (inteira) | `30 / 10` | `3` |
| `%` | Resto (módulo) | `30 % 7` | `2` |

### Características Específicas do Rust

#### 1. Divisão Inteira
```rust
let resultado: i8 = 30 / 10;  // resultado = 3
```
- Em Rust, divisão entre inteiros **sempre resulta em inteiro**.
- Não há arredondamento automático; a parte decimal é truncada.

**Comparação com outras linguagens:**
- **Java**: `30 / 10` também resulta em inteiro (3)
- **C/C++**: `30 / 10` também resulta em inteiro (3)
- **Diferença**: Em Rust, você **não pode** fazer `30 / 10.0` diretamente; tipos devem ser compatíveis

#### 2. Operador de Potência
```rust
let potencia: i8 = 10.pow(2);  // resultado = 100
```
- Rust **não possui** operador `**` como Python ou `^` como alguns outros
- Use o método `.pow(expoente)` onde o expoente é `u32`
- Isso pode causar **overflow** se o resultado exceder o tipo

**Comparação:**
- **Java**: `Math.pow(10, 2)` retorna `double`
- **C/C++**: `pow(10, 2)` do `<cmath>`, retorna `double`
- **Rust**: `.pow(2)` é específico para o tipo numérico

#### 3. Tipos Numéricos e Overflow
```rust
let numero: i8 = 10;  // Faixa: -128..=127
```
- Rust força você a escolher o tipo explicitamente
- Em **debug mode**, overflow causa `panic!`
- Em **release mode**, overflow ocorre silenciosamente (wrapping)

**Comparação:**
- **Java**: Usa tipos como `byte` (-128..127), `int` (32-bit), com overflow silencioso
- **C/C++**: Comportamento de overflow é indefinido (undefined behavior)
- **Rust**: Explícito e seguro por padrão

---

## 2. Operadores de Atribuição

Operadores de atribuição combinam uma operação com atribuição para forma abreviada.

### Conceito Teórico

Esses operadores reduzem a verbosidade ao atualizar variáveis com base em seu valor atual.

### Operadores Disponíveis

| Operador | Equivalente | Exemplo | Descrição |
|----------|------------|---------|-----------|
| `=` | Atribuição | `x = 5` | Atribui valor |
| `+=` | `x = x + 5` | `x += 5` | Adiciona e atribui |
| `-=` | `x = x - 3` | `x -= 3` | Subtrai e atribui |
| `*=` | `x = x * 2` | `x *= 2` | Multiplica e atribui |
| `/=` | `x = x / 4` | `x /= 4` | Divide e atribui |
| `%=` | `x = x % 3` | `x %= 3` | Resto e atribui |

### Características Específicas do Rust

#### 1. Mutabilidade Obrigatória
```rust
let mut x: i8 = 15;  // DEVE ser `mut` para modificar
x += 5;  // Agora válido
```
- Variáveis em Rust são **imutáveis por padrão**
- Você **deve** declarar com `mut` para modificar
- Isso previne alterações acidentais

**Comparação:**
- **Java**: Variáveis são mutáveis por padrão, use `final` se quiser imutável
- **C/C++**: Variáveis são mutáveis por padrão, use `const` para imutável
- **Rust**: Imutável por padrão, use `mut` para mutável (**oposto!**)

#### 2. Sem Operadores `++` e `--`
```rust
// ERRADO em Rust:
x++;  // Erro de compilação!
x--;  // Erro de compilação!

// CORRETO:
x += 1;
x -= 1;
```
- Rust **remove** os operadores `++` e `--`
- Força código mais explícito e clara intenção
- Evita bugs como `x++ * ++y` com efeitos colaterais

**Comparação:**
- **Java**: Suporta `++` e `--`
- **C/C++**: Suporta `++` e `--` (com comportamento diferente para pré/pós-incremento)
- **Rust**: Não existe, força clareza

---

## 3. Operadores de Comparação

Operadores de comparação retornam um valor booleano (`true` ou `false`).

### Conceito Teórico

Comparações são usadas em condições para tomar decisões no código.

### Operadores Disponíveis

| Operador | Nome | Exemplo | Resultado (a=20, b=25) |
|----------|------|---------|------------------------|
| `==` | Igualdade | `a == b` | `false` |
| `!=` | Desigualdade | `a != b` | `true` |
| `>` | Maior que | `a > b` | `false` |
| `<` | Menor que | `a < b` | `true` |
| `>=` | Maior ou igual | `a >= b` | `false` |
| `<=` | Menor ou igual | `a <= b` | `true` |

### Características Específicas do Rust

#### 1. Type Safety em Comparações
```rust
let a: i8 = 20;
let b: i8 = 25;
println!("{}", a == b);  // OK: ambos i8

let c: i32 = 20;
println!("{}", a == c);  // ERRO: tipos diferentes!
```
- Rust **não permite** comparar tipos diferentes
- Você deve fazer conversão explícita

**Comparação:**
- **Java**: Permite comparação entre `int` e `long` (conversão implícita)
- **C/C++**: Permite comparação entre tipos numéricos (conversão automática)
- **Rust**: Exige tipos idênticos (segurança de tipo)

#### 2. Sem Operador de Atribuição em Comparação
```rust
// ERRADO:
if x = 5 {  // Erro! Não compila
}

// CORRETO:
if x == 5 {  // OK
}
```
- Rust não permite `=` em contextos booleanos
- Previne um erro comum onde `if (x = 5)` era intendido como `if (x == 5)`

**Comparação:**
- **Java**: Também previne este erro (em versões modernas)
- **C/C++**: Permite `if (x = 5)` (comum fonte de bugs!)

---

## 4. Operadores Lógicos

Operadores lógicos combinam condições booleanas.

### Conceito Teórico

Operadores lógicos permitem criar expressões complexas com múltiplas condições.

### Operadores Disponíveis

| Operador | Nome | Exemplo | Significado |
|----------|------|---------|-------------|
| `&&` | E (AND) | `x && y` | Verdadeiro se **ambos** forem true |
| `\|\|` | OU (OR) | `x \|\| y` | Verdadeiro se **pelo menos um** for true |
| `!` | NÃO (NOT) | `!x` | Inverte o valor booleano |

### Tabela Verdade

#### AND (`&&`)
| x | y | x && y |
|---|---|--------|
| true | true | true |
| true | false | false |
| false | true | false |
| false | false | false |

#### OR (`||`)
| x | y | x \|\| y |
|---|---|----------|
| true | true | true |
| true | false | true |
| false | true | true |
| false | false | false |

#### NOT (`!`)
| x | !x |
|---|-----|
| true | false |
| false | true |

### Características Específicas do Rust

#### 1. Avaliação de Curto-Circuito
```rust
let x = true;
let y = false;

// Na expressão abaixo, `y` nunca é avaliado
if x || y {
    println!("Entra aqui");
}

// Na expressão abaixo, `y` nunca é avaliado
if x && expensive_function() {
    // expensive_function() não é chamada se x for false
}
```
- `&&` retorna `false` assim que encontra um `false`
- `||` retorna `true` assim que encontra um `true`
- Economiza processamento

**Comparação:**
- **Java**: Também usa curto-circuito com `&&` e `||`
- **C/C++**: Também usa curto-circuito
- **Rust**: Idêntico aos outros

#### 2. Operadores Bitwise (Bônus)
```rust
let a = 5;      // 0101 em binário
let b = 3;      // 0011 em binário

println!("{}", a & b);  // 0001 = 1 (AND bitwise)
println!("{}", a | b);  // 0111 = 7 (OR bitwise)
println!("{}", a ^ b);  // 0110 = 6 (XOR bitwise)
println!("{}", !a);     // inverte todos os bits
println!("{}", a << 1); // shift left (multiplica por 2)
println!("{}", a >> 1); // shift right (divide por 2)
```

---

## Resumo Comparativo

### Tabela de Diferenças

| Aspecto | Rust | Java | C/C++ |
|---------|------|------|-------|
| Tipos Numéricos | Deve especificar (i8, i32, etc) | int, long, float, double | int, long, float, double |
| Overflow | Panic em debug, wrap em release | Overflow silencioso | Undefined behavior |
| Mutabilidade | Imutável por padrão | Mutável por padrão | Mutável por padrão |
| `++` e `--` | **Não existem** | Existem | Existem |
| Comparação | Requer tipos idênticos | Conversão implícita | Conversão implícita |
| Divisão | Inteira para inteiros | Inteira para inteiros | Inteira para inteiros |
| Potência | `.pow()` | `Math.pow()` | `pow()` |

---

## Exemplo Prático Completo

```rust
fn main() {
    // Operações aritméticas
    let num1: i8 = 10;
    let num2: i8 = 30;
    
    let soma = num1 + num2;          // 40
    let subtracao = num2 - num1;     // 20
    let multiplicacao = num1 * 2;    // 20
    let divisao = num2 / num1;       // 3
    let resto = num2 % 7;            // 2
    let potencia = num1.pow(2);      // 100

    println!("Soma: {}", soma);
    println!("Subtração: {}", subtracao);
    println!("Multiplicação: {}", multiplicacao);
    println!("Divisão: {}", divisao);
    println!("Resto: {}", resto);
    println!("Potência: {}", potencia);

    // Operadores de atribuição
    let mut contador: i8 = 0;
    contador += 5;   // contador = 5
    contador -= 2;   // contador = 3
    contador *= 2;   // contador = 6
    contador /= 2;   // contador = 3
    contador %= 2;   // contador = 1

    // Comparações
    let a = 20;
    let b = 25;
    
    println!("a > b: {}", a > b);     // false
    println!("a == b: {}", a == b);   // false
    println!("a <= b: {}", a <= b);   // true

    // Operadores lógicos
    let x = true;
    let y = false;
    
    println!("x && y: {}", x && y);   // false
    println!("x || y: {}", x || y);   // true
    println!("!x: {}", !x);           // false
}
```

---

## Conclusão

Rust oferece uma abordagem mais segura e explícita aos operadores:

1. **Tipos explícitos** — segurança de tipo garantida
2. **Mutabilidade controlada** — imutável por padrão
3. **Sem `++` e `--`** — código mais claro
4. **Overflow controlado** — não é undefined behavior
5. **Comparações seguras** — tipos devem corresponder

Essas características fazem Rust mais seguro que C/C++ e mais explícito que Java, prevenindo classes inteiras de bugs.
