fn main() {
    // Programa demonstrativo: operadores e operações básicas em Rust
    // Valores de exemplo e observações sobre tipos estão abaixo.

    // Números de exemplo usados em todas as demonstrações abaixo.
    // Tipo: `i8` escolhido por simplicidade (faixa: -128..=127).
    // Note: operações que ultrapassem esse intervalo causariam overflow em builds de release.
    let number1: i8 = 10;
    let number2: i8 = 30;

    // Bloco 1: operações aritméticas básicas
    {
        println!("Todas as operações matemáticas");
        println!("-----------------------------------");

        // Soma: usa o operador `+` para adicionar dois inteiros
        let soma: i8 = number1 + number2;
        println!("A soma dos números {} e {} é {}", number1, number2, soma);
        println!("-----------------------------------");

        // Subtração: `a - b` calcula a diferença entre dois valores
        let subtracao: i8 = number2 - number1;
        println!("A subtração entre {} e {} é {}", number2, number1, subtracao);
        println!("-----------------------------------");

        // Multiplicação: multiplica `number1` por um fator (2 neste exemplo)
        let multiplicacao: i8 = number1 * 2;
        println!("A multiplicação de {} por {} é {}", number1, 2, multiplicacao);
        println!("-----------------------------------");

        // Divisão: divisão inteira entre dois `i8` (descarta parte fracionária)
        let divisao: i8 = number2 / number1; // 30 / 10 = 3
        println!("A divisão entre {} e {} é {}", number2, number1, divisao);
        println!("-----------------------------------");

        // Resto: operador `%` retorna o resto da divisão inteira
        let resto: i8 = number2 % 7; // resto de 30 / 7 = 2
        println!("O resto da divisão entre {} e {} é {}", number2, 7, resto);
        println!("-----------------------------------");

        // Potência: usa o método `.pow()` para elevar ao expoente (u32)
        // `.pow(2)` calcula o quadrado do número
        let potencia: i8 = number1.pow(2);
        println!("A potência de {} elevado a 2 é {}", number1, potencia);
        println!("-----------------------------------");
    }

    println!();

    // Bloco 2: operadores de atribuição compostos
    {
        // Operadores de atribuição compostos demonstram formas abreviadas

        // de atualizar o valor de uma variável mutável (`mut`).
        println!("Operadores de Atribuição");
        println!("-----------------------------------");
        let mut atribuicao: i8 = 15; // valor inicial
        println!("Valor inicial: {}", atribuicao);

        // `+=` adiciona e atribui o resultado à variável
        atribuicao += 5; // agora 20
        println!("Após += 5: {}", atribuicao);

        // `-=` subtrai e atribui
        atribuicao -= 3; // agora 17
        println!("Após -= 3: {}", atribuicao);

        // `*=` multiplica e atribui
        atribuicao *= 2; // agora 34
        println!("Após *= 2: {}", atribuicao);

        // `/=` divide e atribui (divisão inteira no caso de inteiros)
        atribuicao /= 4; // agora 8 (34 / 4 = 8)
        println!("Após /= 4: {}", atribuicao);

        // `%=` aplica o operador resto e atribui
        atribuicao %= 3; // agora 2 (8 % 3 = 2)
        println!("Após %= 3: {}", atribuicao);
        println!("-----------------------------------");
    }

    println!();

    // Bloco 3: operadores de comparação
    {
        // Operadores de comparação retornam `bool` e são usados em condições
        println!("Operadores de Comparação");
        println!("-----------------------------------");
        let a: i8 = 20;
        let b: i8 = 25;

        // Comparações comuns: igualdade, diferença e relações de ordem
        println!("a = {}, b = {}", a, b);
        println!("a == b: {}", a == b); // igualdade
        println!("a != b: {}", a != b); // desigualdade
        println!("a > b: {}", a > b);   // maior que
        println!("a < b: {}", a < b);   // menor que
        println!("a >= b: {}", a >= b); // maior ou igual
        println!("a <= b: {}", a <= b); // menor ou igual
        println!("-----------------------------------");
    }

    println!();

    // Bloco 4: operadores lógicos
    {
        // Operadores lógicos trabalham com valores booleanos (`bool`)
        println!("Operadores Lógicos");
        println!("-----------------------------------");
        let x: bool = true;
        let y: bool = false;

        println!("x = {}, y = {}", x, y);

        // `&&` é o operador lógico E (true somente se ambos forem true)
        println!("x AND y: {}", x && y);

        // `||` é o operador lógico OU (true se pelo menos um for true)
        println!("x OR y: {}", x || y);

        // `!` nega o valor booleano
        println!("NOT x: {}", !x);
        println!("NOT y: {}", !y);
        println!("-----------------------------------");
    }
}