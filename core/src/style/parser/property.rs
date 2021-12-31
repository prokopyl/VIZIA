
use chumsky::{prelude::*, chain::Chain};
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

use super::{Token, Spanned};


// fn hex_color() -> impl Parser<Token, Spanned<crate::Color>, Error = Simple<Token>> {
//     let hash = filter_map(|span, token| match token {
//         Token::Hash(value) => Ok(value.clone()),
//         _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
//     })
//     .labelled("identifier");
// }

fn rgb_color() -> impl Parser<Token, Spanned<crate::Color>, Error = Simple<Token>> {
    let number = filter_map(|span, token| match token {
        Token::Number(value) => value.parse::<u8>().or_else(move |_| Err(Simple::expected_input_found(span, Vec::new(), Some(Token::Number(value.clone()))))),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
    .labelled("identifier");

    let rgb = just(Token::Function("rgb".to_string()))
        .ignore_then(number)
        .then_ignore(just(Token::Comma))
        .then(number)
        .then_ignore(just(Token::Comma))
        .then(number)
        .map_with_span(|((red, green), blue), span| (crate::Color::rgb(red, green, blue), span));

    rgb
}



#[cfg(test)]
mod tests {
    
    use crate::style::parser::tokenizer::tokenize;

    use super::*;
    
    #[test]
    fn test_color() {
        const TEST_STRING: &str = r#"
            .button:hover {
                background-color: rgb(255, 0, 0);
            }
        "#;

        if let Ok(tokens) = tokenize().parse(TEST_STRING) {
            println!("{:?}", tokens);
            let output = rgb_color().parse(tokens.into_iter().map(|(token, _)| token).collect::<Vec<Token>>());
            println!("{:?}", output);
        }
    }

}
