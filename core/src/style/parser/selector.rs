use chumsky::prelude::*;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};


use super::Token;

use super::tokenizer::*;

pub type Spanned<T> = (T, Span);


pub enum Combinator {
    Universal,
}

pub enum Component {
    Combinator(Combinator),
}



fn universal() -> impl Parser<Token, Spanned<String>, Error = Simple<Token>> {
    filter_map(|span, token| match token {
        Token::Delim('*') => Ok(("*".to_string(), span)),
        _=> Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
}

// Parse a selector id
fn id() -> impl Parser<Token, Spanned<String>, Error = Simple<Token>> {

    // let ident = filter_map(|span, tok| match tok {
    //     Token::Ident(ident) => Ok(ident.clone()),
    //     _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    // })
    // .labelled("identifier");

    let hash = filter_map(|span, token| match token {
        Token::Hash(ident) => Ok(ident.clone()),
        _=> Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
    .labelled("hash");

    hash.map_with_span(|ident, span| (ident, span) )
}

// Parse a selector element
fn element() -> impl Parser<Token, Spanned<String>, Error = Simple<Token>> {
    let ident = filter_map(|span, token| match token {
        Token::Ident(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
    .labelled("identifier");

    ident.map_with_span(|ident, span| (ident, span) )
}

// Parse a class selector
fn class() -> impl Parser<Token, Spanned<String>, Error = Simple<Token>> {

    let ident = filter_map(|span, token| match token {
        Token::Ident(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
    .labelled("identifier");

    just(Token::Delim('.')).ignore_then(ident).map_with_span(|ident, span| (ident, span) )
}

// Parse a selector pseudoclass
fn pseudoclass() -> impl Parser<Token, Spanned<String>, Error = Simple<Token>> {

    let ident = filter_map(|span, token| match token {
        Token::Ident(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
    .labelled("identifier");

    just(Token::Colon).ignore_then(ident).map_with_span(|ident, span| (ident, span) )

}

fn selector() -> impl Parser<Token, Spanned<String>, Error = Simple<Token>> {
    universal()
        .or(id())
        .or(class())
        .or(pseudoclass())
}

// fn pseudoelement() -> impl Parser<char, (Token, Span), Error = Simple<char>> {

// }

// // Parse the universal selector
// fn universal() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
//     just('*').map_with_span(|ident, span| (Token::Ident(ident.to_string()), span))
// }

// // Parse a selector tag (universal | id | element | class | pseudoclass)
// fn tag() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
//     universal()
//         .or(id())
//         .or(element())
//         .or(class())
//         .or(pseudoclass())
// }

// // Parse a selector combinator (> | + | ~)
// fn combinator() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
//     just('>').padded().map_with_span(|_, span| (Token::Combinator(Combinator::Parent), span))
//         .or(just('+').padded().map_with_span(|_, span| (Token::Combinator(Combinator::Adjacent), span)))
//         .or(just('~').padded().map_with_span(|_, span| (Token::Combinator(Combinator::Sibling), span)))
// }


#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn test_id() {
        const TEST_STRING: &str = r#"
            .button
        "#;

        if let Ok(tokens) = tokenize().parse(TEST_STRING) {
            println!("{:?}", tokens);
            let output = selector().parse(tokens.into_iter().map(|(token, _)| token).collect::<Vec<Token>>());
            println!("{:?}", output);
        }
    }


    #[test]
    fn test_class() {
        let tokens = vec![Token::Delim('.'), Token::Ident("class_name".to_string())];
        let output = class().parse(tokens);
        println!("{:?}", output);
    }

}