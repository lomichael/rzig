extern crate regex;

use regex::Regex;
use regex::Match;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    IDENTIFIER(String), // 'main'
    INTEGER(i32), // '42'
    LBRACKET, // '{'
    LPAREN, // '('
    RBRACKET, // '}' 
    RPAREN, // ')'
    SEMICOLON, // ';'
    KEYWORD_export, // 'export'
    KEYWORD_fn, // 'fn'
    KEYWORD_return, // 'return'
    KEYWORD_returntype(String), // 'i32' (NOT TO SPEC)
}

pub fn tokenize(input: &str) -> Vec<Token> {
    // the regex for matching different tokens
    let lexer_spec = [
        (Regex::new(r"[a-zA-Z_]\w*").unwrap(), "IDENTIFIER"),
        (Regex::new(r"[0-9]+").unwrap(), "INTEGER"),
        (Regex::new(r"\{").unwrap(), "LBRACKET"),
        (Regex::new(r"\(").unwrap(), "LPAREN"),
        (Regex::new(r"\}").unwrap(), "RBRACKET"),
        (Regex::new(r"\)").unwrap(), "RPAREN"),
        (Regex::new(r"\;").unwrap(), "SEMICOLON"),
        (Regex::new(r"export").unwrap(), "KEYWORD_export"),
        (Regex::new(r"fn").unwrap(), "KEYWORD_fn"),
        (Regex::new(r"return").unwrap(), "KEYWORD_return"),
        (Regex::new(r"i32").unwrap(), "KEYWORD_returntype"),
    ];

    // the token list we return
    let mut tokens = Vec::new();

    let mut input = input;
    while !input.is_empty() {
        let mut longest_match: Option<(Match, &str)> = None;
        for (regex, token_name) in lexer_spec.iter() {
            if let Some(regex_match) = regex.find(input) {
                if regex_match.start() == 0 && (longest_match.is_none() || regex_match.end() > longest_match.unwrap().0.end()) {
                    longest_match = Some((regex_match, *token_name));
                }
            }
        }

        // push longest match to tokens list
        match longest_match {
            Some((regex_match, token_name)) => {
                let token = match token_name {
                    "IDENTIFIER" => Token::IDENTIFIER(regex_match.as_str().into()),
                    "INTEGER" => Token::INTEGER(regex_match.as_str().parse::<i32>().unwrap()),
                    "LBRACKET" => Token::LBRACKET,
                    "LPAREN" => Token::LPAREN,
                    "RBRACKET" => Token::RBRACKET,
                    "RPAREN" => Token::RPAREN,
                    "SEMICOLON" => Token::SEMICOLON,
                    "KEYWORD_export" => Token::KEYWORD_export,
                    "KEYWORD_fn" => Token::KEYWORD_fn,
                    "KEYWORD_return" => Token::KEYWORD_return,
                    "KEYWORD_returntype" => Token::KEYWORD_returntype(regex_match.as_str().into()),
                    _ => panic!("Unknown token"),
                };
                tokens.push(token);
                input = &input[regex_match.end()..];
            },
            None => panic!("Invlaid token at position {}", input.chars().next().unwrap()),
        }
    }

    tokens
}
