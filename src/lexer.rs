#[derive(Debug, Clone, Copy)]
enum Token<'a> {
    Text(&'a str),
    BoldDelimiter,
    ItalicsDelimiter,
    MonoSpaceDelimiter,
    HighlightDelimiter,
}

fn lex(text: &str) -> Vec<Token> {
    todo!()
}
