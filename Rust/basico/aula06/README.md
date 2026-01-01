# Aula 06: Laços (Loops) em Rust

## Introdução

Laços permitem que um bloco de código seja executado múltiplas vezes. Rust oferece três tipos principais de laços: `loop`, `while` e `for`, cada um com suas características e casos de uso.

---

## 1. LOOP: Loop Infinito Controlado

`loop` é a forma mais simples de criar um laço que continua indefinidamente até que você o interrompa com `break`.

### Conceito Teórico

Um `loop` em Rust é um laço infinito que você controla manualmente. Diferente de `while` ou `for`, não há condição de parada automática.

### Sintaxe Básica

```rust
let mut i = 0;
loop {
    i += 1;
    println!("i = {}", i);
    
    if i == 10 {
        break;  // Interrompe o loop
    }
}
println!("Fim do loop, i = {}", i);
```

### Características Específicas do Rust

#### 1. LOOP Sem Condição Explícita
```rust
// LOOP: sem condição
loop {
    println!("Executado indefinidamente");
    break;
}

// WHILE: com condição
while condicao {
    println!("Executado enquanto condição for true");
}
```

**Comparação:**
- **Java**: Usa `while(true)` para simular loop infinito
  ```java
  while (true) {
      // código
      break;
  }
  ```
- **C/C++**: Usa `while(1)` ou `for(;;)`
  ```cpp
  while (1) {
      // código
  }
  ```
- **Rust**: `loop` é explícito e seguro

#### 2. LOOP Retorna um Valor (Único em Rust!)
```rust
let mut contador = 0;

// loop RETORNA um valor!
let resultado = loop {
    contador += 3;
    
    if contador >= 10 {
        break contador * 2;  // Retorna o valor
    }
};

println!("Resultado = {}", resultado);  // Imprime: 26
```

**Passo a passo:**
1. `contador` começa em 0
2. Incrementa para 3 (3 < 10, continua)
3. Incrementa para 6 (6 < 10, continua)
4. Incrementa para 9 (9 < 10, continua)
5. Incrementa para 12 (12 >= 10, **break**)
6. `break 12 * 2` retorna **24**
7. `resultado = 24`

**Comparação:**
- **Java**: `while` não retorna valor
- **C/C++**: `while` não retorna valor
- **Rust**: **`loop` retorna valor!** (Exclusive feature)

#### 3. BREAK vs CONTINUE
```rust
let mut i = 0;
loop {
    i += 1;
    
    if i == 3 {
        continue;  // Pula para próxima iteração
    }
    
    if i == 5 {
        break;     // Interrompe o loop
    }
    
    println!("i = {}", i);  // 1, 2, 4
}
```

---

## 2. WHILE: Loop Condicional

`while` executa um bloco enquanto uma condição for verdadeira.

### Conceito Teórico

`while` é útil quando você sabe que precisa repetir algo, mas não sabe quantas vezes. A condição é verificada antes de cada iteração.

### Sintaxe Básica

```rust
let mut i = 10;
while i > 0 {
    println!("i = {}", i);
    i -= 1;  // Decremento
}
println!("Decrescimento completo!");
```

### Características Específicas do Rust

#### 1. Condição Deve Ser Booleana
```rust
let mut i = 10;

// CORRETO:
while i > 0 {
    i -= 1;
}

// ERRADO:
while i {  // Erro: i não é bool!
    i -= 1;
}
```
- Rust **exige** que a condição seja explicitamente booleana
- Sem conversão implícita de números

**Comparação:**
- **C/C++**: Permite `while(i)` (0 = false, não-zero = true)
- **Java**: Permite `while(i != 0)` (seguro)
- **Rust**: Exige booleano explícito (mais seguro)

#### 2. WHILE vs FOR
```rust
// WHILE: você controla o incremento
let mut i = 0;
while i < 10 {
    println!("{}", i);
    i += 1;  // Você incrementa manualmente
}

// FOR: o incremento é automático
for i in 0..10 {
    println!("{}", i);
}
```

**Quando usar:**
- **`while`**: Condições complexas, incremento/decremento manual
- **`for`**: Intervalos ou iteração sobre coleções

#### 3. Sem Operadores `++` e `--`
```rust
// ERRADO em Rust:
while i > 0 {
    i--;  // Erro de compilação!
}

// CORRETO:
while i > 0 {
    i -= 1;
}
```

---

## 3. FOR: Loop sobre Coleções e Intervalos

`for` itera sobre uma sequência de valores (ranges, arrays, etc.).

### Conceito Teórico

`for` é a forma mais idiomática em Rust para repetir algo um número específico de vezes ou iterar sobre elementos de uma coleção.

### Sintaxe Básica com Ranges

```rust
// Itera de 0 a 9 (10 não está incluído)
for i in 0..10 {
    println!("Rust é de mais! {}", i);
}
```

### Características Específicas do Rust

#### 1. Ranges em Rust

```rust
// 0..10: 0 a 9 (10 não incluído)
for i in 0..10 {
    println!("{}", i);  // 0, 1, 2, ..., 9
}

// 0..=10: 0 a 10 (10 incluído)
for i in 0..=10 {
    println!("{}", i);  // 0, 1, 2, ..., 10
}

// 1..5: 1 a 4
for i in 1..5 {
    println!("{}", i);  // 1, 2, 3, 4
}
```

**Notação:**
- `a..b` — exclusiva no final (a a b-1)
- `a..=b` — inclusiva no final (a a b)

**Comparação:**
- **Java**: Usa `range(0, 10)` ou `IntStream.range(0, 10)`
- **C++**: Usa `for (int i = 0; i < 10; i++)`
- **Rust**: `0..10` é mais limpo e idiomático

#### 2. FOR com Arrays/Vetores
```rust
let numeros = [1, 2, 3, 4, 5];

// Itera sobre elementos
for num in numeros {
    println!("{}", num);
}

// Itera com índice
for (i, num) in numeros.iter().enumerate() {
    println!("numeros[{}] = {}", i, num);
}
```

#### 3. FOR é Mais Seguro que WHILE
```rust
// WHILE: Fácil cometer erros de índice
let mut i = 0;
while i < 10 {
    println!("{}", i);
    i += 1;  // Você pode esquecer este incremento!
}

// FOR: Seguro e automático
for i in 0..10 {
    println!("{}", i);  // Nenhum risco de loop infinito
}
```

#### 4. Variável de Loop é Imutável
```rust
// ERRADO:
for mut i in 0..10 {
    i += 1;  // Modifica a cópia local, não a variável do loop
}

// CORRETO:
for i in 0..10 {
    println!("{}", i);  // Use i como está
}

// Se precisa de mutação, use while:
let mut i = 0;
while i < 10 {
    i += 1;  // Agora pode mutar
}
```

#### 5. FOR com BREAK e CONTINUE
```rust
for i in 0..10 {
    if i == 3 {
        continue;  // Pula para próxima iteração
    }
    
    if i == 7 {
        break;     // Sai do loop
    }
    
    println!("{}", i);  // 0, 1, 2, 4, 5, 6
}
```

---

## Comparação Completa: LOOP vs WHILE vs FOR

### Tabela Comparativa

| Aspecto | LOOP | WHILE | FOR |
|---------|------|-------|-----|
| Condição de parada | Manual (`break`) | Automática | Automática (iterações) |
| Retorna valor | **Sim** | Não | Não |
| Incremento automático | Não | Não | **Sim** |
| Iterações desconhecidas | Bom | **Melhor** | Não ideal |
| Iterações conhecidas | Não ideal | Não ideal | **Melhor** |
| Segurança de tipo | Alta | Alta | **Muito Alta** |
| Risk of infinite loop | **Sim** | Sim (fácil erro) | Não |

---

## Exemplos Comparativos entre Linguagens

### Exemplo 1: Contagem de 0 a 9

**Java:**
```java
for (int i = 0; i < 10; i++) {
    System.out.println(i);
}
```

**C/C++:**
```cpp
for (int i = 0; i < 10; i++) {
    cout << i << endl;
}
```

**Rust:**
```rust
for i in 0..10 {
    println!("{}", i);
}
```

**Vantagem Rust:** Sintaxe mais limpa, sem risco de errar a condição

---

### Exemplo 2: Contagem Regressiva

**Java:**
```java
int i = 10;
while (i > 0) {
    System.out.println(i);
    i--;
}
```

**C/C++:**
```cpp
int i = 10;
while (i > 0) {
    cout << i << endl;
    i--;
}
```

**Rust:**
```rust
let mut i = 10;
while i > 0 {
    println!("{}", i);
    i -= 1;
}
```

**Diferença Rust:** Sem `--`, usa `-=`

---

### Exemplo 3: Loop com Valor de Retorno

**Java:**
```java
int resultado = 0;
for (int i = 0; i < 10; i++) {
    resultado += i;
}
System.out.println(resultado);  // 45
```

**C/C++:**
```cpp
int resultado = 0;
for (int i = 0; i < 10; i++) {
    resultado += i;
}
cout << resultado << endl;  // 45
```

**Rust:**
```rust
let resultado = (0..10).sum();
// Ou com loop:
let resultado = loop {
    // lógica
    break valor;
};
```

**Vantagem Rust:** Pode usar loops como expressões (retorna valor)

---

## Padrões Idiomáticos em Rust

### 1. Preferir FOR a WHILE
```rust
// Evite:
let mut i = 0;
while i < 10 {
    println!("{}", i);
    i += 1;
}

// Prefira:
for i in 0..10 {
    println!("{}", i);
}
```

### 2. Usar LOOP quando Precisa de BREAK com Valor
```rust
let resultado = loop {
    if condicao {
        break valor;
    }
};
```

### 3. Usar WHILE para Condições Complexas
```rust
let mut i = 0;
while i < 100 && nao_encontrou {
    i += 1;
}
```

---

## Resumo Comparativo

### Tabela de Diferenças

| Aspecto | Rust | Java | C/C++ |
|---------|------|------|-------|
| FOR sintaxe | `for i in range` | `for (int i = 0; ...)` | `for (int i = 0; ...)` |
| `++` e `--` | **Não existem** | Existem | Existem |
| LOOP retorna valor | **Sim** | Não | Não |
| FOR com ranges | **Simples** | Verboso | Verboso |
| Segurança | **Alta** | Alta | Baixa |
| Erro comum | Raro | Fácil (++/--) | Fácil (off-by-one) |

---

## Exemplo Prático Completo

```rust
fn main() {
    println!("=== DEMONSTRAÇÃO DE LAÇOS ===\n");

    // LOOP com break
    println!("1. LOOP com Break:");
    let mut i = 0;
    loop {
        i += 1;
        println!("i = {}", i);
        if i == 5 {
            break;
        }
    }
    println!("Fim do loop\n");

    // LOOP retornando valor
    println!("2. LOOP Retornando Valor:");
    let mut contador = 0;
    let resultado = loop {
        contador += 3;
        if contador >= 10 {
            break contador * 2;
        }
    };
    println!("Resultado = {}\n", resultado);

    // WHILE com contagem regressiva
    println!("3. WHILE com Contagem Regressiva:");
    let mut countdown = 5;
    while countdown > 0 {
        println!("Contagem: {}", countdown);
        countdown -= 1;
    }
    println!("Lançamento!\n");

    // FOR com range
    println!("4. FOR com Range:");
    for i in 0..5 {
        println!("Iteração: {}", i);
    }
    println!();

    // FOR com range inclusivo
    println!("5. FOR com Range Inclusivo:");
    for i in 1..=3 {
        println!("Número: {}", i);
    }
    println!();

    // FOR com break e continue
    println!("6. FOR com BREAK e CONTINUE:");
    for i in 0..10 {
        if i == 3 {
            continue;  // Pula
        }
        if i == 7 {
            break;     // Sai
        }
        println!("i = {}", i);
    }
}
```

**Saída esperada:**
```
=== DEMONSTRAÇÃO DE LAÇOS ===

1. LOOP com Break:
i = 1
i = 2
i = 3
i = 4
i = 5
Fim do loop

2. LOOP Retornando Valor:
Resultado = 26

3. WHILE com Contagem Regressiva:
Contagem: 5
Contagem: 4
Contagem: 3
Contagem: 2
Contagem: 1
Lançamento!

4. FOR com Range:
Iteração: 0
Iteração: 1
Iteração: 2
Iteração: 3
Iteração: 4

5. FOR com Range Inclusivo:
Número: 1
Número: 2
Número: 3

6. FOR com BREAK e CONTINUE:
i = 0
i = 1
i = 2
i = 4
i = 5
i = 6
```

---

## Conclusão

Rust oferece três tipos de laços, cada um com seu propósito:

1. **LOOP** — para controle total e valores de retorno
2. **WHILE** — para condições complexas
3. **FOR** — para iteração sobre ranges e coleções (recomendado)

Características que tornam Rust superior:

1. **FOR é idiomático** — Rust encoraja `for` em vez de `while`
2. **Sem `++` e `--`** — código mais explícito
3. **LOOP retorna valor** — expressões são poderosas em Rust
4. **Segurança** — compilador previne off-by-one errors
5. **Clareza** — ranges como `0..10` são mais legíveis que `for (int i = 0; i < 10; i++)`

Isso torna Rust mais seguro e expressivo para trabalhar com laços comparado a Java e especialmente C/C++.
