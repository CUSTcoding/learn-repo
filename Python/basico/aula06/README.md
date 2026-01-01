# Controle de Fluxo em Python – Loops e Controle de Loop

Este guia explica como controlar o fluxo de execução de programas em Python usando **loops (`for` e `while`)** e mecanismos de **controle de loop** como `break`.  
Cada exemplo inclui explicações detalhadas sobre **o que acontece em cada linha**.

---

## 1. Loop `for` – Repetição com quantidade definida

```python
print("Loop for:")
for i in range(3):
    print(i)
Explicação linha a linha:

for i in range(3):

Cria um loop que se repete 3 vezes (valores 0, 1 e 2).

range(3) gera a sequência 0, 1, 2.

i assume cada valor da sequência a cada iteração.

print(i)

Exibe o valor atual de i em cada repetição.

Uso: quando sabemos quantas vezes queremos repetir algo.

2. Loop while – Repetição com condição
python
Copiar código
print("Loop while:")
contador = 0

while contador < 3:
    print(contador)
    contador += 1
Explicação linha a linha:

contador = 0 → inicializa a variável de controle.

while contador < 3: → o loop continua enquanto a condição for verdadeira.

print(contador) → exibe o valor atual de contador.

contador += 1 → incrementa contador para eventualmente terminar o loop.

Uso: quando não sabemos exatamente quantas vezes o loop deve se repetir, mas temos uma condição.

3. Controle de Loop: break
python
Copiar código
print("Break:")
for i in range(5):
    if i == 3:
        break
    print(i)
Explicação linha a linha:

# Aula 06 - Loops

## Objetivo

Dominar estruturas de repetição em Python: `for`, `while`, controle de loop (`break`, `continue`), `enumerate`, `zip`, compreensões e generators.

---

## Tópicos

1. **Loop `for`**
   - Iteração sobre sequências
   - `range()`
   - `enumerate()` e `zip()`

2. **Loop `while`**
   - Repetição baseada em condição
   - Evitar loops infinitos

3. **Controle de loop**
   - `break` e `continue`

4. **Loops aninhados**
   - Iteração sobre estruturas 2D

5. **Compreensões**
   - List, dict e set comprehensions

6. **Iteradores e generators**
   - Expressões geradoras e `yield`

---

## Exemplos Práticos

```python
# Loop for com range
for i in range(5):
    print(i)

# Enumerate
frutas = ["maçã", "banana", "laranja"]
for indice, fruta in enumerate(frutas):
    print(indice, fruta)

# Zip
nomes = ["Alice", "Bob"]
idades = [25, 30]
for nome, idade in zip(nomes, idades):
    print(f"{nome}: {idade} anos")

# While
contador = 0
while contador < 3:
    print(contador)
    contador += 1

# Break / Continue
for i in range(5):
    if i == 3:
        break
    if i % 2 == 0:
        continue
    print(i)

# List comprehension
pares = [x for x in range(10) if x % 2 == 0]
print(pares)

# Generator expression
gen = (x*x for x in range(5))
for v in gen:
    print(v)
```

---

## Scripts

Este diretório contém `loop.py` com exemplos de loops. Execute:

```bash
python loop.py
```

**Última atualização**: Dezembro 2025

