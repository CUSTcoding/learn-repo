# ===============================
# O QUE SÃO VARIÁVEIS?
# ===============================
# Variáveis são espaços na memória usados para armazenar valores.
# Em Python, você não precisa declarar o tipo da variável.

# ===============================
# CRIAÇÃO DE VARIÁVEIS
# ===============================

nome = "Carlos"        # string (texto)
idade = 25             # int (número inteiro)
altura = 1.75          # float (número decimal)
estudante = True       # bool (booleano: True ou False)

# ===============================
# EXIBINDO VARIÁVEIS
# ===============================

print(nome)
print(idade)
print(altura)
print(estudante)

# ===============================
# TIPO DAS VARIÁVEIS
# ===============================

print(type(nome))
print(type(idade))
print(type(altura))
print(type(estudante))

# ===============================
# REATRIBUIÇÃO DE VALORES
# ===============================
# Em Python, o valor de uma variável pode mudar

idade = 26
print("Nova idade:", idade)

# ===============================
# CONVENÇÕES DE NOMES
# ===============================
# Use nomes claros e descritivos
# Use snake_case para variáveis

valor_total = 150.50
quantidade_produtos = 3

# ===============================
# OPERAÇÕES COM VARIÁVEIS
# ===============================

media = valor_total / quantidade_produtos
print("Média por produto:", media)

# ===============================
# CONCATENAÇÃO DE STRINGS
# ===============================

mensagem = "Olá, meu nome é " + nome
print(mensagem)

# ===============================
# F-STRINGS (FORMA MODERNA)
# ===============================

print(f"Meu nome é {nome} e tenho {idade} anos.")


