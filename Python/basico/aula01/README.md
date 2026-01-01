# Aula 01 - Primeiros Passos em Python

## Objetivos da Aula

Nesta aula você aprenderá a instalar o Python, configurar um ambiente virtual e executar seu primeiro script.

**Tópicos abordados:**
- Instalação do Python
- Criação de ambiente virtual (`venv`)
- Primeiro script: `Hello, World!`
- Execução de scripts e uso do interpretador interativo

---

## 1. Instalação

Linux (Debian/Ubuntu):

```bash
sudo apt update
sudo apt install python3 python3-venv python3-pip
```

macOS (Homebrew):

```bash
brew install python
```

Windows:

- Baixe o instalador em https://www.python.org/downloads/ e marque "Add Python to PATH".

---

## 2. Ambiente Virtual

```bash
python3 -m venv venv
source venv/bin/activate  # Linux/macOS
venv\Scripts\activate    # Windows
pip install --upgrade pip
```

---

## 3. Primeiro Script

Arquivo: `hello.py`

```python
print("Hello, world!")
```

Executar:

```bash
python hello.py
```

Saída esperada:

```
Hello, world!
```

---

## 4. Interpretador Interativo

```bash
python
>>> print('Olá')
```

---

## Exercícios

1. Crie um script que peça o nome do usuário e imprima uma saudação.
2. Crie um script que leia dois números e imprima a soma.

