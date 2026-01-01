/*
 * AULA 03 - Tipos Primitivos e Data Types em Rust
 * 
 * Tipos de Dados Primitivos:
 * ├─ ESCALARES (armazenam 1 valor):
 * │  ├─ Inteiros: i8, i16, i32, i64, i128, isize (signed)
 * │  │            u8, u16, u32, u64, u128, usize (unsigned)
 * │  ├─ Floats: f32, f64
 * │  ├─ Booleanos: bool
 * │  └─ Caracteres: char
 * │
 * └─ COMPOSTOS (armazenam múltiplos valores):
 *    ├─ Tuplas: (tipo1, tipo2, ...)
 *    └─ Arrays: [tipo; tamanho]
 */

fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║      AULA 03: TIPOS PRIMITIVOS EM RUST                ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
    
    // ===== 1. INTEIROS SIGNED (Com Sinal) =====
    println!("▓▓▓ 1. INTEIROS SIGNED (podem ser negativos) ▓▓▓");
    
    let byte: i8 = 127;                          // -128 a 127
    let short: i16 = -32768;                     // -32,768 a 32,767
    let numero: i32 = -42;                       // Tipo padrão
    let grande: i64 = 9_223_372_036_854_775_807; // Muito grande
    
    println!("  i8  (byte):     {} bits → {}", 8, byte);
    println!("  i16 (short):    {} bits → {}", 16, short);
    println!("  i32 (padrão):   {} bits → {}", 32, numero);
    println!("  i64 (grande):   {} bits → {}\n", 64, grande);
    
    
    // ===== 2. INTEIROS UNSIGNED (Sem Sinal) =====
    println!("▓▓▓ 2. INTEIROS UNSIGNED (apenas positivos) ▓▓▓");
    
    let byte_pos: u8 = 255;                      // 0 a 255
    let short_pos: u16 = 65535;                  // 0 a 65,535
    let numero_pos: u32 = 1_000_000;             // Underscore para legibilidade
    let grande_pos: u64 = 18_446_744_073_709_551_615; // Máximo u64
    
    println!("  u8  (máx):      {} bits → {}", 8, byte_pos);
    println!("  u16 (máx):      {} bits → {}", 16, short_pos);
    println!("  u32 (grande):   {} bits → {}", 32, numero_pos);
    println!("  u64 (máximo):   {} bits → {}\n", 64, grande_pos);
    
    
    // ===== 3. FLOATS (Números Decimais) =====
    println!("▓▓▓ 3. FLOATS (números decimais) ▓▓▓");
    
    let pi_f32: f32 = 3.14159;
    let pi_f64: f64 = 3.14159265358979;          // Mais preciso (padrão)
    let altura = 1.75;                           // Type inference → f64
    let velocidade: f32 = 9.8;                   // Aceleração
    let notacao_cientifica = 1.5e10;             // 15 bilhões
    
    println!("  f32: {}        (menos preciso)", pi_f32);
    println!("  f64: {} (mais preciso)", pi_f64);
    println!("  altura: {}m      (type inference)", altura);
    println!("  velocidade: {} m/s", velocidade);
    println!("  científica: {}\n", notacao_cientifica);
    
    
    // ===== 4. BOOLEANOS =====
    println!("▓▓▓ 4. BOOLEANOS (verdadeiro ou falso) ▓▓▓");
    
    let verdadeiro: bool = true;
    let falso = false;                           // Type inference
    let maior_que = 10 > 5;
    let eh_igual = 42 == 42;
    let nao_igual = 10 != 5;
    
    println!("  true:              {}", verdadeiro);
    println!("  false:             {}", falso);
    println!("  10 > 5:            {}", maior_que);
    println!("  42 == 42:          {}", eh_igual);
    println!("  10 != 5:           {}\n", nao_igual);
    
    
    // ===== 5. CARACTERES (Char) =====
    println!("▓▓▓ 5. CARACTERES (um caractere Unicode) ▓▓▓");
    
    let letra = 'A';
    let numero_char = '5';
    let espaco = ' ';
    let simbolo = '♠';
    let emoji = '🦀';  // Rust suporta Unicode completo!
    
    println!("  Letra: '{}'", letra);
    println!("  Número: '{}'", numero_char);
    println!("  Espaço: '{}'", espaco);
    println!("  Símbolo: '{}'", simbolo);
    println!("  Emoji: '{}'\n", emoji);
    
    
    // ===== 6. TUPLAS =====
    println!("▓▓▓ 6. TUPLAS (múltiplos tipos, tamanho fixo) ▓▓▓");
    
    // Tupla com 4 elementos de tipos diferentes
    let pessoa: (i32, &str, f64, bool) = (25, "Alice", 1.75, true);
    
    println!("  Tupla: {:?}", pessoa);
    println!("  Acesso por índice:");
    println!("    pessoa.0 = {} (idade)", pessoa.0);
    println!("    pessoa.1 = {} (nome)", pessoa.1);
    println!("    pessoa.2 = {}m (altura)", pessoa.2);
    println!("    pessoa.3 = {} (ativo)", pessoa.3);
    
    // Desestruturação
    let (idade, nome, altura, ativo) = pessoa;
    println!("\n  Após desestruturação:");
    println!("    {} tem {} anos, {}m e ativo: {}\n", nome, idade, altura, ativo);
    
    
    // ===== 7. ARRAYS =====
    println!("▓▓▓ 7. ARRAYS (mesmo tipo, tamanho fixo) ▓▓▓");
    
    // Array com 5 números
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
    println!("  Array numeros: {:?}", numeros);
    println!("  Tamanho: {} elementos", numeros.len());
    println!("  Primeiro: {}, Último: {}", numeros[0], numeros[4]);
    
    // Array de strings
    let cores = ["vermelho", "verde", "azul"];
    println!("\n  Array cores: {:?}", cores);
    
    // Array com valores repetidos
    let zeros = [0; 5];
    println!("  Array zeros: {:?}", zeros);
    
    // Iteração sobre array
    println!("\n  Iterando sobre números:");
    for (index, &numero) in numeros.iter().enumerate() {
        println!("    índice {} → valor {}", index, numero);
    }
    
    println!();
    
    
    // ===== 8. EXEMPLO PRÁTICO: CADASTRO DE DESENVOLVEDOR =====
    println!("▓▓▓ 8. EXEMPLO PRÁTICO: CADASTRO DE DESENVOLVEDOR ▓▓▓\n");
    
    // Dados do desenvolvedor em diferentes tipos
    let nome = "Carlos";
    let idade: u8 = 28;
    let salario: f64 = 5500.50;
    let altura: f32 = 1.80;
    let ativo = true;
    let nivel_experiencia: i32 = 5;
    
    println!("  ╔════════════════════════════════════════╗");
    println!("  ║  CADASTRO DE DESENVOLVEDOR             ║");
    println!("  ╠════════════════════════════════════════╣");
    println!("  ║ Nome:              {:<20} ║", nome);
    println!("  ║ Idade:             {:<20} ║", format!("{} anos", idade));
    println!("  ║ Altura:            {:<20} ║", format!("{} m", altura));
    println!("  ║ Salário:           {:<20} ║", format!("R$ {}", salario));
    println!("  ║ Anos experiência:  {:<20} ║", format!("{} anos", nivel_experiencia));
    println!("  ║ Ativo:             {:<20} ║", ativo);
    println!("  ╚════════════════════════════════════════╝\n");
    
    
    // ===== 9. RESUMO DE TAMANHOS =====
    println!("▓▓▓ 9. RESUMO DE TAMANHOS EM BYTES ▓▓▓\n");
    
    println!("  Escalares:");
    println!("    bool:  {} bytes", std::mem::size_of::<bool>());
    println!("    i8:    {} bytes", std::mem::size_of::<i8>());
    println!("    i32:   {} bytes", std::mem::size_of::<i32>());
    println!("    i64:   {} bytes", std::mem::size_of::<i64>());
    println!("    f32:   {} bytes", std::mem::size_of::<f32>());
    println!("    f64:   {} bytes", std::mem::size_of::<f64>());
    println!("    char:  {} bytes", std::mem::size_of::<char>());
    
    println!("\n  Compostos:");
    println!("    (i32, &str):    {} bytes", std::mem::size_of::<(i32, &str)>());
    println!("    [i32; 5]:       {} bytes", std::mem::size_of::<[i32; 5]>());
    
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                   FIM DA AULA 03                       ║");
    println!("╚════════════════════════════════════════════════════════╝");
}
