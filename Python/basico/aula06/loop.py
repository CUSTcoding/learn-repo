# ===============================
# CONTROLE DE FLUXO
# ===============================

# for -> repetição com quantidade definida
print("Loop for:")
for i in range(3):
    print(i)

# -------------------------------

# while -> repetição com condição
print("Loop while:")
contador = 0

while contador < 3:
    print(contador)
    contador += 1

# ===============================
# CONTROLE DE LOOP
# ===============================

# break -> encerra o loop
print("Break:")
for i in range(5):
    if i == 3:
        break
    print(i)
