# Shell Scripting - Aprendizado e Domínio

## Objetivo do Aprendizado

Shell script é a linguagem de automação nativa dos sistemas Unix/Linux. Estou estudando Shell Scripting com o objetivo de:

- **Automação de Tarefas**: Automatizar workflows, deployments e operações do sistema
- **Administração de Sistemas**: Gerenciar servidores, permissões e processos
- **Processamento de Dados**: Manipular texto, logs e arquivos em massa
- **Produtividade de Desenvolvedor**: Criar scripts que economizam horas de trabalho manual
**Foco Principal**: Automação e administração de sistemas Linux

---

## Principais Áreas de Estudo

### 1. Básico (`/basico`)
Fundamentos essenciais de Shell Scripting

**Tópicos**:
- Estrutura de script (shebang, permissões)
- Variáveis e tipos
- Strings e interpolação
- Números e aritmética
- Arrays e associative arrays
- Estruturas de controle (if/else, case, loops)
- Funções e scope
- Pipes e redirects
- Command substitution
- Exit codes e status
- Debugging scripts
- Best practices e style guide

**Aplicação Prática**:
- Scripts básicos de automação
- Backup de arquivos
- Monitoramento de sistema
- Instalação automatizada

---

### 2. Automação (`/automacao`)
Criação de scripts avançados para automação

**Tópicos**:
- Processamento de arquivos
- Scheduling com cron
- Parallel execution
- Error handling e recovery
- Logging e debugging
- Configuration management
- Deployment scripts
- Backup e restore
- Monitoramento e alertas
- Integration com outras ferramentas

**Aplicação Prática**:
- CI/CD pipelines
- Deployments automáticos
- System backups
- Log analysis e parsing
- Infrastructure automation

---

### 3. Comandos Avançados (`/command`)
Dominando ferramentas essenciais do shell

**Tópicos**:
- **Text Processing**:
  - grep, egrep, fgrep (busca)
  - sed (stream editor)
  - awk (processamento estruturado)
  - cut, tr, sort, uniq (manipulação)

- **File Operations**:
  - find, locate, xargs
  - ls, cp, mv, rm avançado
  - Permissions (chmod, chown)
  - Disk usage (du, df)

- **System Monitoring**:
  - ps, top, htop
  - netstat, ss (network)
  - systemctl, service
  - journalctl (logs)

- **Advanced Piping**:
  - Complex pipelines
  - Process substitution
  - Here documents (heredocs)
  - Tee e output redirection

- **Regular Expressions**:
  - Padrões POSIX e extended
  - Metacharacters
  - Capturing groups

**Aplicação Prática**:
- Processamento em massa de dados
- Análise de logs complexos
- Busca e substituição avançada
- Troubleshooting de sistemas
- Performance tuning

---

## Relacionamento com Outras Linguagens

### **Shell → Python** (Escalabilidade)
```
Shell Script (automação simples)
         ↓
    Python (automação complexa)
         ↓
    DevOps tooling completo
```

- **Shell**: Rápido, simples, integrado no SO
- **Python**: Quando lógica fica complexa
- **Ambos**: Frequentemente usados juntos

### **Shell ↔ Docker** (Containerização)
- Dockerfile usa Shell commands
- Scripts Shell dentro de containers
- Orchestration com shell

---

## Motivação Pessoal

Estou estudando Shell Scripting porque:


1. **Automação de Horas**: Scripts podem economizar horas de trabalho manual
2. **Qualquer Job Unix**: Conhecimento útil em praticamente qualquer carreira tech
3. **Base para CI/CD**: GitHub Actions, GitLab CI, Jenkins todos usam Shell
7. **Compatibilidade**: Shell script roda em qualquer Unix/Linux

**Meta**: Aprender Shell scripting para automação profissional, deploy, monitoramento e administração de sistemas, sendo capaz de escrever scripts eficientes em produção.

---

## Estrutura de Pastas

```
Shell/
├── README.md (este arquivo)
├── basico/
│   ├── README.md
│   └── introducao/
│       └── README.md
├── automacao/
│   ├── CI/CD scripts
│   ├── Deployment scripts
│   └── Monitoring scripts
└── command/
    ├── Text processing examples
    ├── File operations
    └── System tools
```

---

## Recursos Úteis

- [GNU Bash Manual](https://www.gnu.org/software/bash/manual/)
- [Shell Parameter Expansion](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html)
- [The Linux Command Line Book](https://linuxcommand.org/)
- [ShellCheck (Script Linter)](https://www.shellcheck.net/)
- [Awesome Bash](https://github.com/awesome-lists/awesome-bash)

