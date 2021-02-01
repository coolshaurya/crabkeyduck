#[derive(Debug, Clone, Copy)]
enum Token<'a> {
    Text(&'a str),
    BoldDelimiter,
    ItalicsDelimiter,
    MonoSpaceDelimiter,
    HighlightDelimiter,
}

fn lex(raw: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut anchor: usize = 0;
    let iter = raw.char_indices().peekable();
    for (i, ch) in iter {
        match ch {
            delim @ '*' | '_' | '`' | '#' => {
                if anchor != i {
                    tokens.push(Token::Text(raw[anchor..i]));
                }
                tokens.push(match delim {
                    '*' => Token::BoldDelimiter,
                    '_' => Token::ItalicsDelimiter,
                    '`' => Token::MonospaceDelimiter,
                    '#' => Token::HighlightDelimiter,
                    _ => unreachable!(),
                });
                if let Some(&x) = iter.peek() {
                    anchor = x.0;
                }
            }
            _other => {
                if iter.peek().is_none() {
                    tokens.push(Token::Text(raw[anchor..]));
                }
            }
        }
    }
    tokens
}
