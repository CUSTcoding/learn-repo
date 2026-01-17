# Pandas - Biblioteca de Análise de Dados

## Objetivo

Este módulo apresenta a biblioteca Pandas, uma das ferramentas mais poderosas para análise e manipulação de dados em Python. Aprenderemos desde os primeiros passos até técnicas avançadas de data wrangling.

---

## Estrutura do Módulo

### 1. Introdução ao Pandas
**Arquivo:** `aula01.ipynb`

Tópicos:
- História e propósito do Pandas
- Instalação e configuração
- Ambiente virtual e dependências
- Visão geral das estruturas de dados (Series e DataFrame)

### 2. Carregamento e Exploração de Dados
**Arquivo:** `aula02.ipynb`

Tópicos:
- Importação do Pandas
- Carregamento de dados CSV com `pd.read_csv()`
- Parâmetros importantes: `sep`, `encoding`, `header`, `usecols`
- Funcionalidades de display: `head()`, `tail()`, `info()`, `describe()`
- Exploração básica de dados

### 3. Manipulação de Dados
**Arquivo:** `aula03.ipynb` *(em desenvolvimento)*

Tópicos:
- Seleção de colunas e linhas
- Filtragem de dados
- Criação e modificação de colunas
- Operações com datas
- GroupBy e agregações

### 4. Limpeza e Preparação de Dados
**Arquivo:** `aula04.ipynb` *(em desenvolvimento)*

Tópicos:
- Tratamento de valores ausentes
- Remoção de duplicatas
- Conversão de tipos de dados
- Renomeação de colunas
- Merge e concatenação de DataFrames

### 5. Análise e Visualização
**Arquivo:** `aula05.ipynb` *(em desenvolvimento)*

Tópicos:
- Estatísticas descritivas avançadas
- Pivot tables
- Integração com Matplotlib/Seaborn
- Exportação de dados

---

## Fluxo de Aprendizado

1. Comece com `aula01.ipynb` para entender o que é Pandas e como instalar
2. Prossiga para `aula02.ipynb` para aprender carregamento e exploração básica
3. Continue com as aulas subsequentes para aprofundar conhecimentos
4. Pratique com os dados em `data/` - datasets sobre uso de idiomas em Moçambique

---

## Como executar os notebooks

Criar e ativar ambiente virtual:

```bash
python -m venv venv
source venv/bin/activate  # Linux/macOS
venv\Scripts\activate     # Windows
pip install -r requirements.txt
```

Executar um notebook:

```bash
jupyter notebook aula01.ipynb
```

Ou usar VS Code com extensão Python/Jupyter.

---

## Dados de Exemplo

A pasta `data/` contém datasets reais sobre:
- Uso de idiomas nas províncias de Moçambique
- Indicadores de ciência e tecnologia

Estes dados são usados nos exemplos práticos das aulas.

---

## Recursos Úteis

- [Documentação Oficial do Pandas](https://pandas.pydata.org/docs/)
- [Pandas Cheat Sheet](https://pandas.pydata.org/Pandas_Cheat_Sheet.pdf)
- [Kaggle - Pandas Tutorials](https://www.kaggle.com/learn/pandas)
- [Real Python - Pandas Tutorials](https://realpython.com/search?q=pandas)

---

**Última atualização**: Janeiro 2026