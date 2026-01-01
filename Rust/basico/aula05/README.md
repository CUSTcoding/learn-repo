# Aula 05: Estruturas de Controle de Fluxo em Rust

## Introdução

Estruturas de controle de fluxo permitem que o programa tome decisões e execute diferentes blocos de código baseado em condições. Nesta aula, exploraremos `if/else` e `match`, que são fundamentais para lógica condicional em Rust.

---

## 1. Estrutura IF

A estrutura `if` executa um bloco de código apenas se uma condição for verdadeira.

### Conceito Teórico

A instrução `if` é a forma mais básica de controle de fluxo. Ela avalia uma expressão booleana e executa o código correspondente apenas se a condição for `true`.

### Sintaxe Básica

```rust
if idade < 18 {
    println!("Menor de idade");
}
```

### Características Específicas do Rust

#### 1. Condição Deve Ser Booleana
```rust
let idade = 15;

// CORRETO:
if idade < 18 {
    println!("Menor");
}

// ERRADO:
if idade {  // Erro: idade não é bool!
    println!("Menor");
}
```
- Rust **exige** que a condição seja explicitamente booleana
- Não há conversão implícita de números para booleanos

**Comparação:**
- **Java**: Também exige booleano (não converte int para boolean)
- **C/C++**: Permite números (0 = false, não-zero = true) — **perigoso!**
- **Rust**: Seguro, exige booleano explícito

#### 2. IF Sem Parênteses
```rust
// RUST (válido):
if idade < 18 {
    println!("Menor");
}

// C/C++/Java (exige parênteses):
if (idade < 18) {
    println!("Menor");
}
```
- Rust permite (mas não exige) parênteses
- Preferência de estilo: Rust prefere **sem** parênteses

---

## 2. Estrutura IF/ELSE

Fornece um bloco alternativo para quando a condição é falsa.

### Conceito Teórico

`else` permite especificar código que executará quando a condição `if` for falsa.

### Sintaxe

```rust
if idade < 18 {
    println!("Menor de idade");
} else {
    println!("Maior de idade");
}
```

### Características Específicas do Rust

#### 1. Else é Opcional
```rust
let status = if idade < 18 {
    "Menor"
} else {
    "Maior"
};
```
- Você não é obrigado a ter um `else`
- Se não houver `else`, o bloco `if` pode não executar nada

#### 2. IF/ELSE como Expressão (Retorna Valor)
```rust
// Em Rust, if/else RETORNA um valor!
let categoria = if idade < 18 {
    "Menor de idade"
} else {
    "Maior de idade"
};

println!("Categoria: {}", categoria);
```

**Comparação:**
- **Java**: `if/else` é statement (não retorna valor)
  ```java
  // Java requer ternário:
  String categoria = (idade < 18) ? "Menor" : "Maior";
  ```
- **C/C++**: `if/else` é statement (não retorna valor)
  ```cpp
  // C++ requer ternário:
  string categoria = (idade < 18) ? "Menor" : "Maior";
  ```
- **Rust**: `if/else` **é expressão** e retorna valor diretamente!

#### 3. Regra: Sem Ponto-e-Vírgula no Valor Retornado
```rust
// CORRETO (retorna o valor):
let resultado = if condicao {
    42  // sem ; aqui
} else {
    0   // sem ; aqui
};

// ERRADO (retorna nada, resultado será ()):
let resultado = if condicao {
    42;  // ; muda para statement!
} else {
    0;
};
```
- A última expressão sem `;` é o valor retornado
- Com `;`, vira statement e não retorna nada

---

## 3. Estrutura IF/ELSE IF/ELSE

Para múltiplas condições em sequência.

### Conceito Teórico

Quando você tem mais de duas possibilidades, use `else if` para verificar múltiplas condições.

### Sintaxe

```rust
if idade < 18 {
    println!("Menor de idade");
} else if idade == 18 {
    println!("Tem exatamente 18 anos");
} else {
    println!("Maior de idade");
}
```

### Características Específicas do Rust

#### 1. Ordem Importa
```rust
if idade > 65 {
    println!("Idoso");
} else if idade > 18 {
    println!("Adulto");
} else if idade >= 13 {
    println!("Adolescente");
} else {
    println!("Criança");
}
```
- A **primeira** condição verdadeira é executada
- As demais são ignoradas

#### 2. Sem Limite de `else if`
```rust
if x == 1 {
    // ...
} else if x == 2 {
    // ...
} else if x == 3 {
    // ...
} else if x == 4 {
    // ...
} else {
    // ...
}
```
- Você pode ter quantos `else if` precisar
- Porém, com muitas condições, considere usar `match`

---

## 4. MATCH: Pattern Matching (Poderoso em Rust!)

`match` é uma estrutura que compara um valor contra padrões e executa código correspondente.

### Conceito Teórico

`match` é mais poderoso que `switch` em linguagens como Java e C++. Ele funciona com padrões (patterns), não apenas valores literais.

### Sintaxe Básica

```rust
let dia = 3;
let dia_nome = match dia {
    1 => "Domingo",
    2 => "Segunda-feira",
    3 => "Terça-feira",
    4 => "Quarta-feira",
    5 => "Quinta-feira",
    6 => "Sexta-feira",
    7 => "Sábado",
    _ => "Dia inválido",
};

println!("O dia é {}", dia_nome);
```

### Características Específicas do Rust

#### 1. MATCH vs SWITCH em C/C++ e Java

**C/C++:**
```cpp
switch (dia) {
    case 1:
        printf("Domingo\n");
        break;  // OBRIGATÓRIO, senão "fall through"
    case 2:
        printf("Segunda-feira\n");
        break;
    default:
        printf("Dia inválido\n");
}
```

**Java:**
```java
switch (dia) {
    case 1:
        System.out.println("Domingo");
        break;
    case 2:
        System.out.println("Segunda-feira");
        break;
    default:
        System.out.println("Dia inválido");
}
```

**Rust:**
```rust
match dia {
    1 => "Domingo",
    2 => "Segunda-feira",
    _ => "Dia inválido",
}
```

**Diferenças:**

| Aspecto | Rust | C/C++ | Java |
|---------|------|-------|------|
| `break` Obrigatório | **Não** | **Sim** | **Sim** |
| Fall-through | **Não** | Padrão | Padrão |
| Retorna Valor | **Sim** | Não | Não |
| `default` | `_` | `default:` | `default:` |
| Verificação de Cobertura | **Sim** (compilador avisa) | Não | Não |

#### 2. Verificação Exaustiva (Compilador Garante Cobertura)
```rust
let numero = 5;

// ERRO: compilador reclama!
let resultado = match numero {
    1 => "Um",
    2 => "Dois",
    // Faltam 3, 4, 5, ... e outros números!
    // Erro: pattern `3_i32..=i32::MAX` not covered
};

// CORRETO:
let resultado = match numero {
    1 => "Um",
    2 => "Dois",
    _ => "Outro",  // Cobre todos os casos restantes
};
```

**Comparação:**
- **C/C++**: Sem verificação; `switch` sem `default` compila normalmente
- **Java**: Sem verificação; `switch` sem `default` compila normalmente
- **Rust**: **Compilador força** você a cobrir todos os casos!

#### 3. MATCH como Expressão (Retorna Valor)
```rust
let dia = 3;
// match RETORNA um valor!
let dia_nome = match dia {
    1 => "Domingo",
    2 => "Segunda-feira",
    3 => "Terça-feira",
    _ => "Inválido",
};

println!("{}", dia_nome);  // Imprime: Terça-feira
```

#### 4. Wildcard `_`
```rust
match numero {
    1 => println!("Um"),
    2 => println!("Dois"),
    _ => println!("Outro"),  // Captura QUALQUER outro valor
}
```
- `_` significa "qualquer outro padrão"
- Deve ser o último padrão (geralmente)

#### 5. Padrões Avançados (Rust-específico)

```rust
match numero {
    1..=5 => println!("Um a cinco"),     // Range
    6 | 7 | 8 => println!("Seis a oito"), // OU
    n if n > 10 => println!("Maior que 10"), // Guard
    _ => println!("Outro"),
}
```

---

## Comparação Detalhada: IF vs MATCH

### Quando Usar IF/ELSE

```rust
let idade = 25;

if idade < 13 {
    println!("Criança");
} else if idade < 18 {
    println!("Adolescente");
} else if idade < 65 {
    println!("Adulto");
} else {
    println!("Idoso");
}
```

**Melhor para:**
- Condições baseadas em comparações
- Intervalos de valores
- Lógica booleana complexa

### Quando Usar MATCH

```rust
let dia = 3;

match dia {
    1 => println!("Domingo"),
    2 => println!("Segunda"),
    3 => println!("Terça"),
    _ => println!("Outro"),
}
```

**Melhor para:**
- Valores discretos/enums
- Padrões específicos
- Quando você quer força exhaustiva do compilador

---

## Resumo Comparativo

### Tabela de Diferenças

| Aspecto | Rust | Java | C/C++ |
|---------|------|------|-------|
| IF retorna valor | **Sim** | Não (usa ternário) | Não |
| Condição booleana | Exigido | Exigido | Convertido implicitamente |
| MATCH/SWITCH break | Não (automático) | Obrigatório | Obrigatório |
| Cobertura exaustiva | **Compilador verifica** | Não | Não |
| Fall-through | Não | Padrão | Padrão |
| MATCH valor | **Sim** | Não | Não |

---

## Exemplo Prático Completo

```rust
fn main() {
    // IF simples
    let idade = 15;
    if idade < 18 {
        println!("Menor de idade");
    }

    // IF/ELSE
    let idade1 = 18;
    if idade1 < 18 {
        println!("Menor");
    } else {
        println!("Maior");
    }

    // IF/ELSE IF/ELSE
    let idade2 = 18;
    if idade2 < 18 {
        println!("Menor");
    } else if idade2 == 18 {
        println!("Tem 18");
    } else {
        println!("Maior");
    }

    // IF como expressão
    let idade3 = 20;
    let categoria = if idade3 < 18 {
        "Menor"
    } else {
        "Maior"
    };
    println!("Categoria: {}", categoria);

    // MATCH
    let dia = 3;
    let dia_nome = match dia {
        1 => "Domingo",
        2 => "Segunda",
        3 => "Terça",
        4 => "Quarta",
        5 => "Quinta",
        6 => "Sexta",
        7 => "Sábado",
        _ => "Inválido",
    };
    println!("Dia: {}", dia_nome);

    // MATCH como expressão
    let resultado = match dia {
        1 | 7 => 10,  // Domingo ou Sábado
        2..=5 => 5,   // Segunda a Quinta
        6 => 8,       // Sexta
        _ => 0,
    };
    println!("Pontos: {}", resultado);
}
```

---

## Conclusão

Rust oferece estruturas de controle superiores:

1. **IF/ELSE como expressões** — retorna valores diretamente
2. **MATCH com verificação exaustiva** — compilador garante cobertura
3. **Sem fall-through** — código mais seguro
4. **Pattern matching avançado** — mais poderoso que `switch`
5. **Segurança de tipo** — sem conversões implícitas perigosas

Essas características tornam Rust mais seguro e expressivo que C/C++ e Java para lógica condicional.
