#[derive(Debug, Clone, Copy, PartialEq)]
enum Token<'a> {
    Text(&'a str),
    BoldDelimiter,
    ItalicsDelimiter,
    MonospaceDelimiter,
    HighlightDelimiter,
}

fn lex(raw: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut anchor: usize = 0;
    let mut iter = raw.char_indices().peekable();
    while let Some((i, ch)) = iter.next() {
        match ch {
            '*' | '_' | '`' | '#' => {
                if anchor != i {
                    tokens.push(Token::Text(&raw[anchor..i]));
                }
                tokens.push(match ch {
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
                    tokens.push(Token::Text(&raw[anchor..]));
                }
            }
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_works() {
        let text = "I am _really_ excited.";
        let expected = vec![
            Token::Text("I am "),
            Token::ItalicsDelimiter,
            Token::Text("really"),
            Token::ItalicsDelimiter,
            Token::Text(" excited."),
        ];
        assert_eq!(lex(text), expected);
    }
}
