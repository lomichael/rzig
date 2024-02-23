extern crate regex;

use log::debug;
use regex::Regex;
use regex::Match;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KEYWORDexport, // 'export'
    KEYWORDfn, // 'fn'
	KEYWORDpub, // 'pub'
    KEYWORDreturn, // 'return'
    KEYWORDreturntype(String), // 'u8' or 'i32' (NOT TO SPEC)
	// MATCH IDENTIFIER AFTER KEYWORDS
    IDENTIFIER(String), // 'main'
    INTEGER(i32), // '42'
    LBRACKET, // '{'
    LPAREN, // '('
    RBRACKET, // '}' 
    RPAREN, // ')'
    SEMICOLON, // ';'
}

pub fn tokenize(input: &str) -> Vec<Token> {
    // the regex for matching different tokens
    let lexer_spec = [
        (Regex::new(r"\bexport\b").unwrap(), "KEYWORDexport"),
        (Regex::new(r"\bfn\b").unwrap(), "KEYWORDfn"),
        (Regex::new(r"\bpub\b").unwrap(), "KEYWORDpub"),
        (Regex::new(r"\breturn\b").unwrap(), "KEYWORDreturn"),
        (Regex::new(r"\bu8|i32\b").unwrap(), "KEYWORDreturntype"),
		// MATCH IDENTIFIER AFTER KEYWORDS
        (Regex::new(r"[a-zA-Z_]\w*").unwrap(), "IDENTIFIER"),
        (Regex::new(r"[0-9]+").unwrap(), "INTEGER"),
        (Regex::new(r"\{").unwrap(), "LBRACKET"),
        (Regex::new(r"\(").unwrap(), "LPAREN"),
        (Regex::new(r"\}").unwrap(), "RBRACKET"),
        (Regex::new(r"\)").unwrap(), "RPAREN"),
        (Regex::new(r"\;").unwrap(), "SEMICOLON"),
    ];

    // the token list we return
    let mut tokens = Vec::new();

    let mut input = input;
    while !input.is_empty() {
		input = input.trim_start();

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
                    "KEYWORDexport" => Token::KEYWORDexport,
                    "KEYWORDfn" => Token::KEYWORDfn,
                    "KEYWORDpub" => Token::KEYWORDpub,
                    "KEYWORDreturn" => Token::KEYWORDreturn,
                    "KEYWORDreturntype" => Token::KEYWORDreturntype(regex_match.as_str().into()),
                    _ => panic!("Unknown token"),
                };
                tokens.push(token);
                input = &input[regex_match.end()..];
            },
            None => {
				if !input.is_empty() {
					panic!("Invalid token at position {}", input.chars().next().unwrap());
				}
				break;
			},
        }
    }
	
	debug!("Tokenized input: {:?}", tokens);
    tokens
}
