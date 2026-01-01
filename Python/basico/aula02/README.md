# Aula 02 - Variáveis 

## Objetivo

Entender como declarar variáveis em Python, os tipos de dados básicos, a diferença entre mutabilidade, convenções de nomeação e escopo de variáveis.

---

## O que são variáveis?

Variáveis são espaços na memória utilizados para armazenar valores que podem ser usados ao longo do programa. Em Python, não é necessário declarar o tipo da variável - o tipo é definido automaticamente no momento da atribuição.

---

## Tópicos

1. **Declaração de variáveis**
   - Sintaxe básica: `nome = valor`
   - Atribuição múltipla: `a, b = 10, 20`
   - Reatribuição de valores

2. **Tipos de dados básicos**
   - Inteiros (`int`)
   - Floats (`float`)
   - Strings (`str`)
   - Booleanos (`bool`)

3. **Mutabilidade**
   - Tipos mutáveis: listas, dicionários, sets
   - Tipos imutáveis: strings, tuplas, números
   - Por que isso importa

4. **Convenções de nomeação**
   - Snake_case para variáveis: `minha_variavel`
   - MAIÚSCULAS para constantes: `PI = 3.14159`
   - Nomes descritivos

5. **Escopo de variáveis**
   - Local (dentro de funções)
   - Global (em todo o programa)
   - Palavra-chave `global`

---

## Exemplos Práticos

```python
# Declaração básica
x = 10           # inteiro
nome = "Alice"   # string
pi = 3.14        # float
ativo = True     # booleano

# Mutabilidade
lista = [1, 2, 3]
lista.append(4)      # lista é mutável - pode ser modificada

tupla = (1, 2, 3)
# tupla[0] = 10      # erro: tupla é imutável

texto = "Hello"
# texto[0] = "J"     # erro: string é imutável

# Convenções
TAXA_JUROS = 0.05    # constante
minha_variavel = 42  # variável
```

---

## Próximas Aulas

- **Aula 01**: Primeiros passos (Hello World)
- **Aula 03**: Tipos de dados mais complexos (listas, dicts)

Execute o script `variables.py` para ver mais exemplos:

```bash
python variables.py
```

