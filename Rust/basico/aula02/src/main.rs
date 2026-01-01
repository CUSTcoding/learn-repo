/*
 * AULA 02 - Variáveis e Mutabilidade em Rust
 * 
 * Conceitos principais:
 * 1. Declaração de variáveis com let
 * 2. Tipagem automática vs explícita
 * 3. Imutabilidade (padrão do Rust)
 * 4. Mutabilidade com 'mut'
 * 5. Escopo de variáveis
 * 6. RAII (Resource Acquisition Is Initialization)
 */

fn main() {
    println!("=== AULA 02: VARIÁVEIS E MUTABILIDADE ===\n");

    // ===== 1. TIPAGEM AUTOMÁTICA =====
    println!("1. TIPAGEM AUTOMÁTICA (Type Inference):");
    
    let nome = "Custódio";      // &str (automaticamente)
    let idade = 20;             // i32 (automaticamente)
    let altura = 1.75;          // f64 (automaticamente)
    
    println!("  Nome: {} (tipo: &str)", nome);
    println!("  Idade: {} (tipo: i32)", idade);
    println!("  Altura: {} (tipo: f64)\n", altura);
    
    
    // ===== 2. TIPAGEM EXPLÍCITA =====
    println!("2. TIPAGEM EXPLÍCITA (Type Annotation):");
    
    let salario: f64 = 3000.50;
    let funcionarios: i32 = 150;
    let ativo: bool = true;
    
    println!("  Salário: R$ {} (f64)", salario);
    println!("  Funcionários: {} (i32)", funcionarios);
    println!("  Ativo: {} (bool)\n", ativo);
    
    
    // ===== 3. IMUTABILIDADE (Padrão) =====
    println!("3. IMUTABILIDADE (Padrão do Rust):");
    
    let x = 5;
    println!("  x = {} (imutável)", x);
    
    // Descomente a linha abaixo para ver o erro:
    // x = 10;  // ❌ ERRO: cannot assign twice to immutable variable
    
    println!("  ✓ Variáveis são imutáveis por padrão\n");
    
    
    // ===== 4. MUTABILIDADE COM 'mut' =====
    println!("4. MUTABILIDADE COM 'mut':");
    
    let mut contador = 0;
    println!("  contador inicial: {}", contador);
    
    contador = 1;
    println!("  contador após modificação: {}", contador);
    
    contador = contador + 5;
    println!("  contador += 5: {}\n", contador);
    
    
    // ===== 5. ESCOPO DE VARIÁVEIS =====
    println!("5. ESCOPO DE VARIÁVEIS:");
    
    let escopo_externo = "Existo aqui";
    println!("  Fora do bloco: {}", escopo_externo);
    
    {
        let escopo_interno = "Só existo dentro do bloco";
        println!("  Dentro do bloco: {}", escopo_interno);
    }
    // Descomente para ver o erro:
    // println!("  {}", escopo_interno);  // ❌ ERRO: cannot find value
    
    println!("  ✓ Escopo respeitado\n");
    
    
    // ===== 6. SHADOWING (Redeclaração) =====
    println!("6. SHADOWING (Redeclaração):");
    
    let numero = 5;
    println!("  número original: {}", numero);
    
    let numero = numero + 1;  // Shadowing!
    println!("  número após shadowing: {}", numero);
    
    let numero = "agora sou um texto";
    println!("  número após outro shadowing: {} (agora é string)\n", numero);
    
    
    // ===== 7. EXEMPLO PRÁTICO =====
    println!("7. EXEMPLO PRÁTICO - Cálculo de Salário:");
    
    let salario_base: f64 = 2500.00;
    let mut bonus: f64 = 0.0;
    
    println!("  Salário base: R$ {}", salario_base);
    
    bonus = salario_base * 0.10;  // 10% de bônus
    println!("  Bônus: R$ {}", bonus);
    
    let salario_total = salario_base + bonus;
    println!("  Salário total: R$ {}\n", salario_total);
    
    
    println!("=== FIM DA AULA ===");
}
