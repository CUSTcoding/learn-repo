
# Aula 05 - Condições

## Objetivo

Entender e aplicar estruturas condicionais em Python: `if`, `elif`, `else`, operadores relacionais, lógicos e operadores de associação/identidade.

---

## Tópicos

1. **Estrutura básica**
   - `if`, `elif`, `else`
   - Blocos e indentação

2. **Operadores relacionais**
   - `==`, `!=`, `>`, `<`, `>=`, `<=`

3. **Operadores lógicos**
   - `and`, `or`, `not`

4. **Operadores de associação**
   - `in`, `not in`

5. **Operadores de identidade**
   - `is`, `is not`

6. **Expressões condicionais (ternary)**
   - `a if cond else b`

---

## Exemplos Práticos

```python
# If / else básico
idade = 18
if idade >= 18:
    print("Maior de idade")
else:
    print("Menor de idade")

# If / elif / else
nota = 7
if nota >= 9:
    print("Excelente")
elif nota >= 7:
    print("Aprovado")
else:
    print("Reprovado")

# Operadores lógicos
idade = 20
tem_carteira = True
if idade >= 18 and tem_carteira:
    print("Pode dirigir")

# Associação
frutas = ["maçã", "banana"]
if "banana" in frutas:
    print("Tem banana")

# Identidade
a = [1, 2, 3]
b = a
c = [1, 2, 3]
print(a is b)      # True
print(a is not c)  # True

# Operador ternário
mensagem = "Maior" if idade >= 18 else "Menor"
print(mensagem)
```

---

## Scripts

Este diretório contém `control_flow.py` com exemplos de condicionais. Execute:

```bash
python control_flow.py
```

**Última atualização**: Dezembro 2025
```


