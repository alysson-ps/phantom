use phantom_language_server::chumsky::parser;

#[test]
fn test_parser() {
    let source = std::fs::read_to_string("examples/sem_erros.php").unwrap();
    let result = parser(&source);

    dbg!(result);

    // println!("{:?}", result);
}