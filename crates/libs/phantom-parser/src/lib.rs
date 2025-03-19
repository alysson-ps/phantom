// mod config;
use chumsky::{
    extra::Err,
    input::{Input, Stream, ValueInput},
    prelude::{
        any, choice, just, nested_delimiters, one_of, recursive, skip_then_retry_until, via_parser,
    },
    select,
    span::SimpleSpan,
    IterParser, Parser,
};
// use config::{load_config, validate};
use logos::Logos;
use phantom_core::{rich::RichError, BinaryOp, Expr, Program, Span, Statement, Token, Type};

fn parser<'a, I>() -> impl Parser<'a, I, Program<'a>, Err<RichError<'a, Token<'a>, Span>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    let block_recovery = nested_delimiters(
        Token::LBracket,
        Token::RBracket,
        [
            (Token::LParen, Token::RParen),
            (Token::LBrace, Token::RBrace),
        ],
        |span| Expr::Error {
            span,
            target: "block",
        },
    );

    just(Token::OpenTag)
        .ignore_then(
            statement()
                .recover_with(skip_then_retry_until(
                    block_recovery.ignored().or(any().ignored()),
                    one_of([
                        Token::RBrace,
                        Token::RParen,
                        Token::RBracket,
                        Token::Semicolon,
                    ])
                    .ignored(),
                ))
                .repeated()
                .collect(),
        )
        .then_ignore(just(Token::CloseTag).or_not())
        .map_with(|statements: Vec<Statement<'a>>, e| Program {
            kind: "Program",
            span: e.span(),
            statements,
        })
}

fn statement<'a, I>(
) -> impl Parser<'a, I, Statement<'a>, Err<RichError<'a, Token<'a>, Span>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    recursive(|statement| {
        let new_lines = just(Token::Newline)
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|n, e| (n.len(), e.span()))
            .validate(|(total_count, span): (usize, SimpleSpan), _, emitter| {
                if total_count > 2 {
                    let span_start = span.start + 2;

                    emitter.emit(RichError::custom(
                        Span::new(span_start, span.end),
                        "error".to_string(),
                        "Too many consecutive new lines",
                        true,
                    ));
                }
            });

        let val = select! {
            Token::Null => Expr::Value{
                kind: "Value",
                type_: Type::Null,
                value: "null",
            },
            Token::Bool(x) => Expr::Value {
                kind: "Value",
                type_: Type::Bool,
                value: x,
            },
            Token::Number(n) => Expr::Value{
                kind: "Value",
                type_: Type::Num,
                value: n,
            },
            Token::String(s) => Expr::Value{
                kind: "Value",
                type_: Type::Str,
                value: s,
            },
        };

        let base_path = select! { Token::Identifier(name) => name };
        let path = base_path.separated_by(just(Token::Backslash)).collect().labelled("Path");

        let namespace_without_brackets = just(Token::Namespace)
            .ignore_then(path.clone())
            .then_ignore(just(Token::Semicolon))
            .then(
                statement.clone().repeated().collect::<Vec<_>>().try_map(|body, span| {
                    if body.iter().any(|stmt| matches!(stmt, Statement::Namespace { .. })) {
                        Err(RichError::custom(
                            span,
                            "error".to_string(),
                            "Nested namespaces are not allowed in this context.".to_string(),
                            false,
                        ))
                    } else {
                        Ok(body)
                    }
                }),
            )
            .map_with(|(path, body), e| Statement::Namespace {
                kind: "Namespace",
                span: e.span(),
                path,
                with_brackets: false,
                body,
            });

        let namespace_with_brackets = just(Token::Namespace)
            .ignore_then(path.clone())
            .then(
                statement
                    .clone()
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with(|(path, body), e| Statement::Namespace {
                kind: "Namespace",
                span: e.span(),
                path,
                with_brackets: true,
                body,
            });

        let namespace =
            namespace_with_brackets.or(namespace_without_brackets).labelled("Namespace");

        let _value_property = just(Token::Assign).ignore_then(val);
        let property = select! {Token::Visibility(visibility) => visibility}
            .then(select! { Token::Variable(name) => name })
            .then(_value_property.or_not())
            .then_ignore(just(Token::Semicolon))
            .map_with(|((visibility, name), value), e| Statement::Property {
                kind: "Property",
                span: e.span(),
                name,
                value: value,
                visibility,
            });

        let class = just(Token::Final)
            .or_not()
            .then_ignore(just(Token::Class))
            .then(select! { Token::Identifier(name) => name })
            .then(
                statement
                    .clone()
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with(|((is_final, name), body), e| Statement::Class {
                kind: "Class",
                span: e.span(),
                is_final: is_final.is_some(),
                is_abstract: false,
                extends: None,
                name,
                body,
            });

        let _arg = select! {Token::Variable(name) => name}
            .then_ignore(just(Token::Assign).or_not())
            .then(val.or_not());

        let args = _arg
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(just(Token::LParen), just(Token::RParen))
            .map(|names| {
                names
                    .into_iter()
                    .map(|(name, val)| Statement::Parameter {
                        kind: "Parameter",
                        name,
                        typed: None,
                        value: val,
                    })
                    .collect::<Vec<_>>()
            })
            .labelled("Args");

        let function = select! { Token::Visibility(visibility) => visibility }
            .labelled("Visibility")
            .then(just(Token::Static).or_not())
            .then_ignore(just(Token::Function))
            .then(select! { Token::Identifier(name) => name })
            .then(args)
            .then(
                expression()
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with(
                |((((visibility, is_static), name), args), body), e| Statement::Method {
                    kind: "Method",
                    span: e.span(),
                    name,
                    is_static: is_static.is_some(),
                    visibility,
                    args,
                    body,
                },
            );

        choice((class, function, namespace, property)).padded_by(new_lines.repeated())
    })
}

fn expression<'a, I>() -> impl Parser<'a, I, Expr<'a>, Err<RichError<'a, Token<'a>, Span>>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    recursive(|expr| {
        let inline_expr = recursive(|inline_expr| {
            let val = select! {
                Token::Null => Expr::Value{
                    kind: "Value",
                    type_: Type::Null,
                    value: "null",
                },
                Token::Bool(x) => Expr::Value {
                    kind: "Value",
                    type_: Type::Bool,
                    value: x,
                },
                Token::Number(n) => Expr::Value{
                    kind: "Value",
                    type_: Type::Num,
                    value: n,
                },
                Token::String(s) => Expr::Value{
                    kind: "Value",
                    type_: Type::Str,
                    value: s,
                },
            }
            .labelled("value");

            let ident = select! { Token::Identifier(ident) => ident };

            // A list of expressions
            let items =
                expr.clone().separated_by(just(Token::Comma)).allow_trailing().collect::<Vec<_>>();

            let variable = select! {Token::Variable(name) => name}
                .map(|name| Expr::Variable(name))
                .labelled("variable");

            let property = select! {Token::Variable(name) => name}
                .then_ignore(just(Token::Arrow))
                .then(select! {Token::Identifier(name) => name})
                .map(|(_name, property)| Expr::Property(property))
                .labelled("property");

            let assingment = choice((variable.clone(), property.clone()))
                .then_ignore(just(Token::Assign))
                .then(inline_expr.clone())
                .then_ignore(just(Token::Semicolon))
                .map(|(target, val)| Expr::Assignment {
                    target: Box::new(target),
                    value: Box::new(val),
                });

            // .then_ignore(just(Token::Assign))
            // .then(inline_expr.clone())
            // .map(|((_, name), val)| Expr::Assignment {
            //     target: Box::new(Expr::Property(name)),
            //     value: Box::new(val),
            // });

            let array = items
                .clone()
                .map(Expr::List)
                .delimited_by(just(Token::LBracket), just(Token::RBracket));

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(ident.map(Expr::Local))
                .or(assingment)
                .or(property.clone())
                .or(variable)
                .or(array)
                .map_with(|expr, _span| expr)
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr.clone().delimited_by(just(Token::LParen), just(Token::RParen)))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::LParen,
                    Token::RParen,
                    [
                        (Token::LBrace, Token::RBrace),
                        (Token::LBracket, Token::RBracket),
                    ],
                    |span| Expr::Error {
                        span,
                        target: "parenthesised expression",
                    },
                )))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::LBrace,
                    Token::RBrace,
                    [
                        (Token::LParen, Token::RParen),
                        (Token::LBracket, Token::RBracket),
                    ],
                    |span| Expr::Error {
                        span,
                        target: "list aqui",
                    },
                )))
                .boxed();

            // Function calls have very high precedence so we prioritise them
            let call = atom.foldl_with(
                items
                    .delimited_by(just(Token::LParen), just(Token::RParen))
                    .map(|args| args)
                    .repeated(),
                |f, args, e| Expr::Call {
                    kind: "Call",
                    span: e.span(),
                    func: Box::new(f),
                    args: Box::new(args),
                },
            );

            // Product ops (multiply and divide) have equal precedence
            let op =
                just(Token::Asterisk).to(BinaryOp::Mul).or(just(Token::Slash).to(BinaryOp::Div));
            let product =
                call.clone().foldl_with(op.then(call).repeated(), |a, (op, b), e| Expr::Binary {
                    kind: "Binary",
                    span: e.span(),

                    left: Box::new(a),
                    op,
                    right: Box::new(b),
                });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Plus).to(BinaryOp::Add).or(just(Token::Minus).to(BinaryOp::Sub));
            let sum = product.clone().foldl_with(op.then(product).repeated(), |a, (op, b), e| {
                Expr::Binary {
                    kind: "Binary",
                    span: e.span(),

                    left: Box::new(a),
                    op,
                    right: Box::new(b),
                }
            });

            // Comparison ops (equal, not-equal) have equal precedence
            let op = just(Token::Eq).to(BinaryOp::Eq).or(just(Token::NotEq).to(BinaryOp::NotEq));
            let compare =
                sum.clone().foldl_with(op.then(sum).repeated(), |a, (op, b), e| Expr::Binary {
                    kind: "Binary",
                    span: e.span(),

                    left: Box::new(a),
                    op,
                    right: Box::new(b),
                });

            let op = just(Token::Concat).to(BinaryOp::Concat);
            let concat =
                compare.clone().foldl_with(op.then(compare).repeated(), |a, (op, b), e| {
                    Expr::Binary {
                        kind: "Binary",
                        span: e.span(),

                        left: Box::new(a),
                        op,
                        right: Box::new(b),
                    }
                });

            // Logical ops (and, or) have equal precedence
            let op = just(Token::And).to(BinaryOp::And).or(just(Token::Or).to(BinaryOp::Or));
            let logic = concat.clone().foldl_with(op.then(concat).repeated(), |a, (op, b), e| {
                Expr::Binary {
                    kind: "Binary",
                    span: e.span(),

                    left: Box::new(a),
                    op,
                    right: Box::new(b),
                }
            });

            logic.labelled("expression").as_context().boxed()
        });

        let new_lines = just(Token::Newline)
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|n, e| (n.len(), e.span()))
            .validate(|(total_count, span): (usize, SimpleSpan), _, emitter| {
                if total_count > 2 {
                    let span_start = span.start + 2;

                    emitter.emit(RichError::custom(
                        Span::new(span_start, span.end),
                        "error".to_string(),
                        "Too many consecutive new lines",
                        true,
                    ));
                }
            });

        let _return = just(Token::Return)
            .ignore_then(inline_expr.clone().or_not())
            .then_ignore(just(Token::Semicolon))
            .map(|ret_expr| Expr::Return {
                expr: Box::new(ret_expr),
            });

        // let block = choice((
        //     _return,
        //     inline_expr.clone().then_ignore(just(Token::Semicolon)),
        // ))
        // .padded_by(new_line.repeated())
        // .delimited_by(just(Token::LBrace), just(Token::RBrace));

        let block_recovery = nested_delimiters(
            Token::LBracket,
            Token::RBracket,
            [
                (Token::LParen, Token::RParen),
                (Token::LBrace, Token::RBrace),
            ],
            |span| Expr::Error {
                span,
                target: "block",
            },
        );

        let block = expr
            .clone()
            .delimited_by(just(Token::LBrace), just(Token::RBrace))
            .recover_with(via_parser(nested_delimiters(
                Token::LBracket,
                Token::RBracket,
                [
                    (Token::LParen, Token::RParen),
                    (Token::LBrace, Token::RBrace),
                ],
                |span| Expr::Error {
                    span,
                    target: "block",
                },
            )));

        let _if = recursive(|if_| {
            just(Token::If)
                .ignore_then(expr.clone().delimited_by(just(Token::LParen), just(Token::RParen)))
                .then(block.clone())
                .then(just(Token::Else).ignore_then(block.clone().or(if_)).or_not())
                .map_with(|((cond, then), else_), _e| Expr::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    else_: Box::new(else_),
                })
        });

        let block_expr = _if.or(block).or(_return);

        let block_chain = block_expr.clone();

        block_chain
            // TODO: warning in 'Semicolon' expression
            .or(inline_expr.then_ignore(just(Token::Semicolon).or_not()))
            .padded_by(new_lines.repeated())
            .recover_with(skip_then_retry_until(
                block_recovery.ignored().or(any().ignored()),
                one_of([
                    Token::RBrace,
                    Token::RParen,
                    Token::RBracket,
                    Token::Semicolon,
                ])
                .ignored(),
            ))
    })
}

#[derive(Debug)]
pub struct ParserResult<'a> {
    pub tokens: Vec<(Token<'a>, Span)>,
    pub ast: Option<Program<'a>>,
    pub parse_errors: Vec<RichError<'a, Token<'a>>>,
}

pub fn parse<'a>(source: &'a str, config_path: &'a str) -> ParserResult<'a> {
    let config = match phantom_config::load(&config_path) {
        Ok(config) => config,
        Err(err) => {
            panic!("{}", err)
        }
    };

    let lexer = Token::lexer(source);

    let token_iter = lexer
        .clone()
        .spanned()
        .filter(|(token, _span)| !matches!(token, Ok(Token::Whitespace | Token::Tab(_))))
        .map(|(tok, span)| match tok {
            // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
            // to work with
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });

    let tokens: Vec<_> = lexer
        .clone()
        .spanned()
        .map(|(tok, span)| match tok {
            // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
            // to work with
            Ok(tok) => (tok, SimpleSpan::from(span)),
            Err(()) => (Token::Error, span.into()),
        })
        .collect();

    let token_stream = Stream::from_iter(token_iter).map((0..source.len()).into(), |(t, s)| (t, s));

    let (result, mut errs) = parser()
        // .validate(|program, _e, emitter| {
        //     let lexer_ref = &mut Box::new(tokens.clone());
        //     let statements_ref = Box::new(program.statements.clone());
        //     validate(&source, lexer_ref, statements_ref, &config, emitter);
        //     program
        // })
        .parse(token_stream)
        .into_output_errors();

    // let (result, errs) = parser().parse(token_stream).into_output_errors();

    if let Some(ref program) = result {
        let tokens_ref = Box::new(tokens.clone());
        let program_ref = Box::new(program.clone());
        let err_ref = &mut Box::new(errs.clone());

        phantom_linter::validate(&source, &tokens_ref, &program_ref, &config, err_ref);
    }

    // dbg!(&result);
    // dbg!(&errs);

    ParserResult {
        tokens,
        ast: result,
        parse_errors: errs,
    }
}
