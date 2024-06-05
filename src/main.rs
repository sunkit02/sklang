use sklang::lexer::Lexer;

fn main() {
    let source = "fn math() -> int { var a: int = 3.14159; print(a <= 1); if (a & 1 > 0 && a != 2 || false) { return a; } else { return 1117; } }";
    let mut lexer = Lexer::new(source);
    lexer.tokenize().iter().for_each(|t| println!("{t:?}"));
}
