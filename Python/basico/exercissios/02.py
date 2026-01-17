'''
    Peça para que o usuário digite um número, em seguida exiba em
    tela o número digitado e o tipo de dado.

    Peça para que o usuário digite uma palavra, em seguida exiba em
    tela a palavra digitado e o tipo de dado.

''' 

num = int(input("Digita um numero: "))
print(f"O numero e {num} e o tipo de dado e: ", type(num))

print("\n")

wold = input("Digita um numero: ")
print(f"A palavra digitada e {wold} e o tipo de dado e: ", type(wold))