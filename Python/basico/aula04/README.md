# Operadores em Python – Guia Completo

Este arquivo contém exemplos dos principais **operadores em Python**, incluindo:
- Aritméticos  
- Relacionais (comparação)  
- Lógicos  
- Atribuição  
- Associação  
- Identidade  

O objetivo deste README é **explicar a lógica de cada operador**, mostrar exemplos práticos com `print()` e esclarecer para que serve cada tipo.

---

## 1. Cabeçalho do Script

```python
"""
Arquivo: operadores_python.py
Assunto: Operadores em Python
Descrição:
Exemplos simples dos principais operadores em Python.
"""
Explicação:

Comentário multilinha (""" ... """) serve como documentação do arquivo.

Indica o nome do arquivo, assunto e descrição.

Boa prática em projetos maiores para organização.

2. Operadores Aritméticos
python
Copiar código
a = 10
b = 3

print(a + b)   # soma
print(a - b)   # subtração
print(a * b)   # multiplicação
print(a / b)   # divisão
print(a // b)  # divisão inteira
print(a % b)   # resto
print(a ** b)  # potência
Explicação linha a linha:

a + b → soma de 10 + 3 = 13

a - b → subtração de 10 - 3 = 7

a * b → multiplicação de 10 * 3 = 30

a / b → divisão normal (3.3333...)

a // b → divisão inteira (3)

a % b → resto da divisão (1)

a ** b → potência (10³ = 1000)

Uso: operações matemáticas básicas em programas.

3. Operadores Relacionais (Comparação)
python
Copiar código
print(a == b)  # igual
print(a != b)  # diferente
print(a > b)   # maior
print(a < b)   # menor
print(a >= b)  # maior ou igual
print(a <= b)  # menor ou igual
Explicação:

== → verifica se os valores são iguais

!= → verifica se os valores são diferentes

> → verifica se o primeiro valor é maior

< → verifica se o primeiro valor é menor

>= → maior ou igual

<= → menor ou igual

Uso: tomada de decisões e condições (if, while).

4. Operadores Lógicos
python
Copiar código
x = True
y = False

print(x and y)  # AND
print(x or y)   # OR
print(not x)    # NOT
Explicação:

and → verdadeiro se ambos os valores forem True

or → verdadeiro se pelo menos um valor for True

not → inverte o valor lógico

Uso: combinar condições em decisões, validar múltiplos critérios.

5. Operadores de Atribuição
python
Copiar código
c = 5
print(c)

c += 2
print(c)

c -= 1
print(c)

c *= 3
print(c)

c /= 2
print(c)
Explicação:

= → atribui valor à variável

+= → incrementa o valor

-= → decrementa o valor

*= → multiplica o valor

/= → divide o valor

Uso: atualizar valores de variáveis de forma prática e legível.

6. Operadores de Associação
python
Copiar código
lista = [1, 2, 3]

print(2 in lista)
print(5 not in lista)
Explicação:

in → verifica se o elemento existe dentro de uma coleção

not in → verifica se o elemento não está na coleção

Uso: verificar presença de elementos em listas, tuplas ou strings.

7. Operadores de Identidade
python
Copiar código
d = 10
e = 10

print(d is e)
print(d is not e)
Explicação:

is → verifica se duas variáveis apontam para o mesmo objeto na memória

is not → verifica se são objetos diferentes

Uso: comparar objetos em memória, não apenas valores.

8. Resumo dos Operadores
Categoria	Operadores	Descrição
Aritméticos	+, -, *, /, //, %, **	Operações matemáticas
Relacionais	==, !=, >, <, >=, <=	Comparação de valores
Lógicos	and, or, not	Combinação de condições
Atribuição	=, +=, -=, *=, /=	Alterar valores de variáveis
Associação	in, not in	Verificar presença de elementos em coleções
Identidade	is, is not	Comparar se objetos são o mesmo na memória