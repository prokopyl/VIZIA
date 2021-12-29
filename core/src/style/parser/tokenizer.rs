


use chumsky::prelude::*;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};


#[derive(Debug, Clone, Copy)]
pub enum Combinator {
    Universal,
    Parent,
    Ancestor,
    Adjacent,
    Sibling,
}


// Tokenizer

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    AtKeyword(String),
    Hash(String),
    Number(f32),
    Percentage(f32),
    Dimension {
        value: f32,
        unit: String,
    },
    Comma,
    Function(String),


}


// Consume comment
fn comment() -> impl Parser<char, (), Error = Simple<char>> {
    just("/*")
        .then(take_until(just("*/")))
        .ignored()
}

// Consume newline
fn newline() -> impl Parser<char, (), Error = Simple<char>> {
    text::newline()
}

// Consume whitespace
fn whitespace() -> impl Parser<char, (), Error = Simple<char>> {
    text::whitespace()
}

// Consume hex digit
fn hex_digit() -> impl Parser<char, String, Error = Simple<char>> {
    text::digits(16)
}

// Consume escape code
fn escape() -> impl Parser<char, char, Error = Simple<char>> {
    just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('b').to('\x08'))
            .or(just('f').to('\x0C'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')),
    )
}

// Consume ident token
fn ident_token<'a>() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    text::ident().map_with_span(|ident, span| (Token::Ident(ident), span))
}


// Consume <function-token>
fn function_token() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    text::ident().then_ignore(just('(')).map_with_span(|ident, span| (Token::Function(ident), span))
} 

// Consume <at-keyword-token>
fn at_keyword_token() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    just('@').ignore_then(text::ident()).map_with_span(|ident, span| (Token::AtKeyword(ident), span))
}



// Parse a number token
// <number> + <ident> | <number> + '%' | <number>
fn numeric_token() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    
    let frac = just('.').chain(text::digits(10));

    let exp = just('e')
        .or(just('E'))
        .ignore_then(just('+').or(just('-')).or_not())
        .chain(text::digits(10));

    let number = just('-')
        .or_not()
        .chain(text::int(10))
        .chain(frac.or_not().flatten())
        .chain::<char, _, _>(exp.or_not().flatten())
        .collect::<String>()
        .from_str()
        .unwrapped()
        .labelled("number");

    number.map(|num: f32| num).then(text::ident()).map_with_span(|(value, unit): (f32, String), span| (Token::Dimension{value, unit}, span))
        .or(number.then_ignore(just('%')).map_with_span(|value, span| (Token::Percentage(value), span)))
        .or(number.map_with_span(|value, span| (Token::Number(value), span)))
}






// fn hex_digit() -> impl Parser<char, (), Error = Simple<char>> {
//     text::digits(16)
// }


pub enum SelectorToken {
    Id(String),
    Element(String),
    Class(String),
    Pseudoclass(String),
    
}

pub type Span = std::ops::Range<usize>;

// // Parse a selector id
// fn id() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
//     just('#').ignore_then(text::ident()).map_with_span(|ident, span| (Token::Id(ident), span))
// }

// // Parse a selector element
// fn element() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
//     text::ident().map_with_span(|ident, span| (Token::Element(ident), span))
// }

// // Parse a class selector
// fn class() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
//     just('.').ignore_then(text::ident()).map_with_span(|ident, span| (Token::Class(ident), span))
// }

// // Parse a selector pseudoclass
// fn pseudoclass() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
//     just(':').ignore_then(text::ident().map_with_span(|ident, span| (Token::PseudoClass(ident), span)))
// }

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


// // Transform the input into a vector of tokens
// fn tokenize() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    
//     recursive(|value|{

//         // Parser for ancestor combinator
//         //let ancestor = just(' ').repeated().at_least(1).then_ignore(tag().rewind()).map_with_span(|_, span| (Token::Combinator(Combinator::Ancestor), span));
        

//         // combinator().map(|token| vec![token])
//         //     .or(just(',').map_with_span(|_, span| vec![(Token::Comma, span)]))
//         //     .or(tag().chain(ancestor.or_not()))
//         //     .padded()
//         //     .repeated()
//         //     .flatten()
//     })
// }







fn print_errors(input: &str, errs: Vec<Simple<char>>) {
    errs.into_iter().for_each(|e| {
        let msg = format!(
            "{}{}, expected {}",
            if e.found().is_some() {
                "Unexpected token"
            } else {
                "Unexpected end of input"
            },
            if let Some(label) = e.label() {
                format!(" while parsing {}", label)
            } else {
                String::new()
            },
            if e.expected().len() == 0 {
                "something else".to_string()
            } else {
                e.expected()
                    .map(|expected| match expected {
                        Some(expected) => expected.to_string(),
                        None => "end of input".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        );

        let report = Report::build(ReportKind::Error, (), e.span().start)
            .with_code(3)
            .with_message(msg)
            .with_label(
                Label::new(e.span())
                    .with_message(format!(
                        "Unexpected {}",
                        e.found()
                            .map(|c| format!("token {}", c.fg(Color::Red)))
                            .unwrap_or_else(|| "sequence".to_string())
                    ))
                    .with_color(Color::Red),
            );

        let report = match e.reason() {
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report.with_label(
                Label::new(span.clone())
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_color(Color::Yellow),
            ),
            chumsky::error::SimpleReason::Unexpected => report,
            chumsky::error::SimpleReason::Custom(msg) => report.with_label(
                Label::new(e.span())
                    .with_message(format!("{}", msg.fg(Color::Yellow)))
                    .with_color(Color::Yellow),
            ),
        };

        report.finish().print(Source::from(input)).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use chumsky::{Parser, prelude::*};
    use super::{print_errors, comment, ident_token, numeric_token, Token};

    //use super::tokenize;
    const TEST_STRING: &str = r#" + element:hover .button, > :active#other"#;


    // #[test]
    // fn it_works() {
    //     let (output, errs) = tokenize().then_ignore(end()).parse_recovery(TEST_STRING);
    //     println!("Output: {:#?}", output);

    //     print_errors(TEST_STRING, errs);
    // }


    #[test]
    fn test_comment() {
        const TEST: &str = r#"/* comment */"#;
        let output = comment().parse(TEST);
        println!("Output: {:#?}", output);
    }

    #[test]
    fn test_number() {
        let output = numeric_token().parse("0.3px");
        println!("Output: {:#?}", output);

        assert_eq!(numeric_token().parse("0.3").map(|token| token.0), Ok(Token::Number(0.3)));
        assert_eq!(numeric_token().parse("0.3%").map(|token| token.0), Ok(Token::Percentage(0.3)));
        assert_eq!(numeric_token().parse("0.3px").map(|token| token.0), Ok(Token::Dimension{value: 0.3, unit: "px".to_string()}));
    }

    #[test]
    fn test_ident() {
        let input = "testytest";
        let output = ident_token().parse(input);
        println!("Output: {:#?}", output);
    }
}
