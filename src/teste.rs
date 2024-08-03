use chumsky;

fn main() {
    let input = "let x = 5;"; // Exemplo de entrada
    let result = chumsky::parser(input);

    println!("AST: {:?}", result.ast);
    for error in result.parse_errors {
        println!("Parse Error: {:?}", error);
    }
}
