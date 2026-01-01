/*
 * AULA 01 - Primeiros Passos em Rust
 * 
 * Conceitos:
 * 1. Estrutura básica de um programa Rust
 * 2. Usando a macro println!
 * 3. Cargo como gerenciador de projetos
 */

fn main() {
    // ===== EXEMPLOS DE PRINTLN! =====
    
    // 1. Texto simples
    println!("Hello, world!");
    
    // 2. Com placeholders {}
    let nome = "Rustacean";
    println!("Bem-vindo, {}!", nome);
    
    // 3. Múltiplos valores
    let linguagem = "Rust";
    let ano = 2026;
    println!("Aprendendo {} em {}!", linguagem, ano);
    
    // 4. Sem quebra de linha (print!)
    print!("Isso está na mesma linha");
    println!(" que isso!");
}
