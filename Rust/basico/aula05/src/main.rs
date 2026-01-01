fn main(){
    // Aula 05: Estruturas de Controle de Fluxo em Rust
    // Demonstra: if, if/else, if/else if/else, if como expressão, e match
    
    println!("═══════════════════════════════════════════");
    println!("     ESTRUTURAS DE CONTROLE DE FLUXO      ");
    println!("═══════════════════════════════════════════");
    println!();

    // ─── ETAPA 1: IF básico ───
    // O `if` é usado para executar um bloco de código se a condição for verdadeira.
    // Estrutura: if <condição> { <bloco_de_código> }
    println!("ETAPA 1: IF básico");
    println!("─────────────────────────");
    let idade = 15;
    
    // Verifica se a idade é menor que 18
    if idade < 18 {
        println!("Menor de idade");
    }
    println!();

    // ─── ETAPA 2: IF/ELSE ───
    // O `else` fornece um bloco alternativo para quando a condição é falsa.
    // Estrutura: if <condição> { ... } else { ... }
    println!("ETAPA 2: IF/ELSE");
    println!("─────────────────────────");
    {
        let idade1 = 18;
        
        // Se idade < 18, executa o primeiro bloco; caso contrário, o segundo.
        if idade1 < 18 {
            println!("Menor de idade");
        } else {
            println!("Maior de idade");
        }
    }
    println!();

    // ─── ETAPA 3: IF/ELSE IF/ELSE ───
    // Permite verificar múltiplas condições em sequência.
    // Estrutura: if <cond1> { ... } else if <cond2> { ... } else { ... }
    println!("ETAPA 3: IF/ELSE IF/ELSE");
    println!("─────────────────────────");
    {
        let idade2 = 18;

        // Verifica se é menor (< 18), depois se tem exatamente 18, depois o resto.
        if idade2 < 18 {
            println!("Menor de idade");
        } else if idade2 == 18 {
            println!("Tem exatamente 18 anos");
        } else {
            println!("Maior de idade");
        }
    }
    println!();

    // ─── ETAPA 4: IF como Expressão ───
    // Em Rust, `if` retorna um valor, permitindo atribuição direta a uma variável.
    // Importante: o último valor de cada bloco é retornado (sem ponto-e-vírgula).
    println!("ETAPA 4: IF como Expressão");
    println!("─────────────────────────");
    {
        let idade3 = 20;

        // O `if` avalia a condição e retorna "Menor de idade" ou "Maior de idade".
        // Esse valor é atribuído à variável `categoria`.
        let categoria = if idade3 < 18 {
            "Menor de idade"
        } else {
            "Maior de idade"
        };

        println!("Categoria: {}", categoria);
    }
    println!();

    // ─── ETAPA 5: MATCH ───
    // `match` é uma estrutura de controle poderosa que testa um valor contra padrões.
    // Estrutura: match <valor> { <padrão> => <ação>, ... }
    println!("ETAPA 5: MATCH");
    println!("─────────────────────────");
    {
        let dia = 3;
        
        // Testa o valor de `dia` contra cada padrão (1-7).
        // `=>` separa o padrão da ação a executar.
        // `_` é um padrão "coringa" que captura qualquer valor não mencionado.
        let dia_nome = match dia {
            1 => "Domingo",
            2 => "Segunda-feira",
            3 => "Terça-feira",      // Este padrão será executado (dia == 3)
            4 => "Quarta-feira",
            5 => "Quinta-feira",
            6 => "Sexta-feira",
            7 => "Sábado",
            _ => "Dia inválido",     // "wildcard" para qualquer outro valor
        };

        println!("O dia é {}", dia_nome);
    }
    
    println!();
    println!("═══════════════════════════════════════════");
    println!("          FIM DA AULA 05                   ");
    println!("═══════════════════════════════════════════");
}