


use chumsky::{prelude::*, text::{ident, TextParser}};
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};


#[derive(Debug, Clone, Copy)]
pub enum Combinator {
    Universal,
    Parent,
    Ancestor,
    Adjacent,
    Sibling,
}

pub type Span = std::ops::Range<usize>;

// (https://drafts.csswg.org/css-syntax/#tokenization)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Ident(String),
    Function(String),
    AtKeyword(String),
    Hash(String),
    IDHash(String),
    String(String),
    BadString(String),
    Url(String),
    BadUrl(String),
    Delim(char),
    Number(String),
    Percentage(String),
    Dimension {
        value: String,
        unit: String,
    },
    Whitespace,
    CDO,
    CDC,
    Colon,
    Semicolon,
    Comma,
    OpenSquareBracket,
    CloseSquareBracket,       
    OpenParenthesis,
    CloseParenthesis,  
    OpenCurlyBracket,
    CloseCurlyBracket,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Ident(s) => write!(f, "{}", s),
            Token::Function(s) => write!(f, "{}", s),
            Token::AtKeyword(s) => write!(f, "{}", s),            
            Token::Hash(s) => write!(f, "{}", s),
            Token::IDHash(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "{}", s),
            Token::BadString(s) => write!(f, "{}", s),
            Token::Url(s) => write!(f, "{}", s),
            Token::BadUrl(s) => write!(f, "{}", s),
            Token::Delim(s) => write!(f, "{}", s),
            Token::Whitespace => write!(f, " "),
            Token::CDO => write!(f, "<!-->"),
            Token::CDC => write!(f, "-->"),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::OpenSquareBracket => write!(f, "["),
            Token::OpenParenthesis => write!(f, "("),
            Token::OpenCurlyBracket => write!(f, "{{"),
            Token::CloseSquareBracket => write!(f, "]"),
            Token::CloseParenthesis => write!(f, ")"),
            Token::CloseCurlyBracket => write!(f, "}}"),
            Token::Number(s) => write!(f, "{}", s),
            Token::Percentage(s) => write!(f, "{}", s),
            Token::Dimension { value, unit } => write!(f, "{}{}", value, unit),
        }
    }
}

pub type TokenSpan = (Token, Span);


// Consume comment (https://drafts.csswg.org/css-syntax/#comment-diagram)
fn comment() -> impl Parser<char, (), Error = Simple<char>> {
    just("/*")
        .then(take_until(just("*/")))
        .padded()
        .ignored()
}

// Consume newline (https://drafts.csswg.org/css-syntax/#newline-diagram)
fn newline() -> impl Parser<char, (), Error = Simple<char>> {
    just('\r')
    .or_not()
    .ignore_then(just('\n'))
    .or(just('\r')) // Carriage return
    .or(just('\x0C')) // Form feed
    .ignored()
}

// Consume whitespace (https://drafts.csswg.org/css-syntax/#whitespace-diagram)
fn whitespace() -> impl Parser<char, (), Error = Simple<char>> {
    just(' ')
        .or(just('\t'))
        .ignored()
        .or(newline())
}

// Consume whitespace token (https://drafts.csswg.org/css-syntax/#whitespace-token-diagram)
fn whitespace_token() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    whitespace().repeated().at_least(1).map_with_span(|_, span| (Token::Whitespace, span))
}




// // Consume whitespace
// fn whitespace() -> impl Parser<char, (), Error = Simple<char>> {
//     text::whitespace()
// }

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

    let ident = filter(|c: &char| c.is_ascii_alphabetic() || *c == '_')
        .map(Some)
        .chain::<char, Vec<_>, _>(
            filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_' || *c == '-').repeated(),
        )
        .collect();

    ident.map_with_span(|ident, span| (Token::Ident(ident), span))
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
//<number> + <ident> | <number> + '%' | <number>
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
        .labelled("number");

    number.then(text::ident()).map_with_span(|(value, unit), span| (Token::Dimension{value, unit}, span))
        .or(number.then_ignore(just('%')).map_with_span(|value, span| (Token::Percentage(value), span)))
        .or(number.map_with_span(|value, span| (Token::Number(value), span)))
}


fn cdo_token() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    just("<!--").map_with_span(|_, span| (Token::CDO, span))
}

fn cdc_token() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    just("-->").map_with_span(|_, span| (Token::CDC, span))
}

fn string_token() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    just('"')
        .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape()).repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map_with_span(|string, span| (Token::String(string), span))
        .labelled("string")
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

fn hash_or_id() -> impl Parser<char, (Token, Span), Error = Simple<char>> {
    just('#').ignore_then(
        text::ident().map_with_span(|ident, span| (Token::IDHash(ident), span))
            .or(filter(|c: &char| c.is_alphanumeric()).repeated().collect().map_with_span(|ident, span| (Token::Hash(ident), span)))
    )
}


// // Transform the input into a vector of tokens
pub fn tokenize() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    
    recursive(|value|{

        // Parser for ancestor combinator
        //let ancestor = just(' ').repeated().at_least(1).then_ignore(tag().rewind()).map_with_span(|_, span| (Token::Combinator(Combinator::Ancestor), span));
        

        // combinator().map(|token| vec![token])
        //     .or(just(',').map_with_span(|_, span| vec![(Token::Comma, span)]))
        //     .or(tag().chain(ancestor.or_not()))
        //     .padded()
        //     .repeated()
        //     .flatten()

        hash_or_id()
            .or(whitespace_token())
            .or(just('(').map_with_span(|_, span| (Token::OpenParenthesis, span)))
            .or(just(')').map_with_span(|_, span| (Token::CloseParenthesis, span)))
            .or(just('[').map_with_span(|_, span| (Token::OpenSquareBracket, span)))
            .or(just(']').map_with_span(|_, span| (Token::CloseSquareBracket, span)))
            .or(just('{').map_with_span(|_, span| (Token::OpenCurlyBracket, span)))
            .or(just('}').map_with_span(|_, span| (Token::CloseCurlyBracket, span)))
            .or(just(',').map_with_span(|_, span| (Token::Comma, span)))
            .or(just(':').map_with_span(|_, span| (Token::Colon, span)))
            .or(just(';').map_with_span(|_, span| (Token::Semicolon, span)))
            .or(cdo_token())
            .or(cdc_token())
            .or(numeric_token())
            .or(function_token())
            .or(ident_token())
            .or(any().map_with_span(|delim, span| (Token::Delim(delim), span)))
            //.padded_by(comment().repeated())
            //.map_with_span(|token, span| (token, span))
            //.padded()
            .repeated()

    })
}







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
    use super::{print_errors, comment, ident_token, Token};

    use super::*;
    const TEST_STRING: &str = r#"
        .button:hover .checkbox {
            background-color: rgb(255, 0, 0);
        }
    "#;

    //const TEST_STRING: &str = r#" #name"#;


    #[test]
    fn test_tokenize() {
        let (output, errs) = tokenize().then_ignore(end()).parse_recovery(TEST_STRING);
        println!("Output: {:#?}", output);

        print_errors(TEST_STRING, errs);
    }

    #[test]
    fn test_whitespace_token() {
        const TEST: &str = r#"   "#;
        let (output, errs) = text::whitespace().map_with_span(|_, span| (Token::Whitespace, span)).then_ignore(end()).parse_recovery(TEST);
        println!("Output: {:#?}", output);

        print_errors(TEST, errs);
    }


    #[test]
    fn test_comment() {
        const TEST: &str = r#"/* comment */"#;
        let output = comment().parse(TEST);
        println!("Output: {:#?}", output);
    }

    // #[test]
    // fn test_number() {
    //     let output = numeric_token().parse("0.3px");
    //     println!("Output: {:#?}", output);

    //     assert_eq!(numeric_token().parse("0.3").map(|token| token.0), Ok(Token::Number(0.3)));
    //     assert_eq!(numeric_token().parse("0.3%").map(|token| token.0), Ok(Token::Percentage(0.3)));
    //     assert_eq!(numeric_token().parse("0.3px").map(|token| token.0), Ok(Token::Dimension{value: 0.3, unit: "px".to_string()}));
    // }

    #[test]
    fn test_ident() {
        let input = "testytest";
        let output = ident_token().parse(input);
        println!("Output: {:#?}", output);
    }
}
