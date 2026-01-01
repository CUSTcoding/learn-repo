"""
Arquivo: operadores_python.py
Assunto: Operadores em Python
Descrição:
Exemplos simples dos principais operadores em Python.
"""

# ===============================
# OPERADORES ARITMÉTICOS
# ===============================

a = 10
b = 3

print(a + b)   # soma
print(a - b)   # subtração
print(a * b)   # multiplicação
print(a / b)   # divisão
print(a // b)  # divisão inteira
print(a % b)   # resto
print(a ** b)  # potência

# ===============================
# OPERADORES RELACIONAIS (COMPARAÇÃO)
# ===============================

print(a == b)  # igual
print(a != b)  # diferente
print(a > b)   # maior
print(a < b)   # menor
print(a >= b)  # maior ou igual
print(a <= b)  # menor ou igual

# ===============================
# OPERADORES LÓGICOS
# ===============================

x = True
y = False

print(x and y)  # AND
print(x or y)   # OR
print(not x)    # NOT

# ===============================
# OPERADORES DE ATRIBUIÇÃO
# ===============================

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

# ===============================
# OPERADORES DE ASSOCIAÇÃO
# ===============================

lista = [1, 2, 3]

print(2 in lista)
print(5 not in lista)

# ===============================
# OPERADORES DE IDENTIDADE
# ===============================

d = 10
e = 10

print(d is e)
print(d is not e)

# ===============================
# FIM DO ARQUIVO
# ===============================
