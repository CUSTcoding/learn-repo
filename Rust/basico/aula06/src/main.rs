fn main() {
    // Aula 06: Laços (Loops) em Rust
    // Demonstra: loop, while, for, e loop como expressão
    
    println!("═══════════════════════════════════════════");
    println!("           LAÇOS EM RUST                  ");
    println!("═══════════════════════════════════════════");
    println!();

    // ─── ETAPA 1: LOOP (Loop Infinito) ───
    // `loop` cria um laço infinito que só termina com `break`.
    // Use-o quando não souber quantas iterações serão necessárias.
    println!("ETAPA 1: LOOP (Loop Infinito)");
    println!("─────────────────────────");
    let mut i = 0;
    
    // Nota: Um loop sem `break` causaria loop infinito
    // Exemplo (comentado):
    // loop {
    //     i += 1;
    //     println!("i = {}", i);
    // }  // Isso seria um erro - loop infinito!

    // LOOP CORRIGIDO com condição de parada:
    loop {
        i += 1; // incremento: em Rust moderno, usa-se `+=` em vez de `i++`
        println!("i = {}", i);

        // Quando i atinge 10, interrompe o loop com `break`
        if i == 10 {
            break;
        }
    }
    println!("Fim do loop, i = {}", i);
    println!();

    // ─── ETAPA 2: WHILE (Loop Condicional) ───
    // `while` executa um bloco enquanto a condição for verdadeira.
    // Útil quando a condição de parada depende de uma variável.
    println!("ETAPA 2: WHILE (Loop Condicional)");
    println!("─────────────────────────");
    println!("Contagem regressiva:");

    i = 10;
    // Enquanto `i` for maior que 0, executa o bloco
    while i > 0 {
        println!("i = {}", i);
        i -= 1; // decremento: em Rust moderno, usa-se `-=` em vez de `i--`
    }
    println!();

    // ─── ETAPA 3: FOR (Loop sobre Coleções/Intervalos) ───
    // `for` itera sobre uma sequência (range, array, etc.).
    // `0..10` cria um intervalo de 0 a 9 (10 não está incluído).
    println!("ETAPA 3: FOR (Loop sobre Intervalos)");
    println!("─────────────────────────");
    
    // Itera sobre cada número de 0 a 9
    for i in 0..10 {
        println!("Rust é de mais! {}", i);
    }
    println!();

    // ─── ETAPA 4: LOOP como Expressão (Retorna Valor) ───
    // Em Rust, `loop` pode retornar um valor quando termina com `break`.
    // O valor após `break` é o que o loop retorna e pode ser atribuído a uma variável.
    println!("ETAPA 4: LOOP como Expressão (Retorna Valor)");
    println!("─────────────────────────");
    
    let mut contador = 0;

    // Este `loop` não só executa, mas também retorna um valor.
    // O valor é capturado em `resultado`.
    let resultado = loop {
        contador += 3;

        // Quando a condição é atendida, `break` retorna `contador * 2`
        if contador >= 10 {
            break contador * 2; // o loop retorna o valor da expressão
        }
    };

    println!("Resultado = {}", resultado);
    
    println!();
    println!("═══════════════════════════════════════════");
    println!("          FIM DA AULA 06                   ");
    println!("═══════════════════════════════════════════");
}
