# ===============================
# EXEMPLO 1: IF / ELSE SIMPLES
# ===============================

# Variável idade
idade = 18

# Verifica se a pessoa é maior de idade
if idade >= 18:       # operador relacional >=
    print("Maior de idade")  # executa se a condição for True
else:
    print("Menor de idade")  # executa se a condição for False

# -------------------------------
# EXEMPLO 2: IF / ELIF / ELSE COM MAIS CONDIÇÕES
# -------------------------------

# Variável nota
nota = 7

if nota >= 9:          # verifica nota maior ou igual a 9
    print("Excelente")
elif nota >= 7:        # verifica nota entre 7 e 8
    print("Aprovado")
else:                  # executa se nota < 7
    print("Reprovado")

# -------------------------------
# EXEMPLO 3: CONDIÇÃO COM OPERADORES LÓGICOS
# -------------------------------

# Variáveis para exemplo lógico
idade = 20
tem_carteira = True

# Verifica duas condições ao mesmo tempo
if idade >= 18 and tem_carteira:  # operador lógico 'and'
    print("Pode dirigir")          # executa se ambas forem True
else:
    print("Não pode dirigir")      # executa se pelo menos uma for False

# -------------------------------
# EXEMPLO 4: CONDIÇÃO COM OPERADORES OR E NOT
# -------------------------------

choveu = False
tem_guarda_chuva = True

if choveu or tem_guarda_chuva:    # operador lógico 'or'
    print("Pode sair sem se molhar")
else:
    print("Fique em casa")

if not choveu:                     # operador lógico 'not'
    print("Hoje está seco")
else:
    print("Está chovendo")