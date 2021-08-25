use crate::parser::{argument, directive, name, selection};
use crate::{create_err, Parser, SyntaxKind, TokenKind};

/// See: https://spec.graphql.org/June2018/#Field
///
/// ```txt
/// Field
///     Alias(opt) Name Arguments(opt) Directives(opt) SelectionSet(opt)
/// ```
pub(crate) fn field(parser: &mut Parser) {
    let guard = parser.start_node(SyntaxKind::FIELD);
    if let Some(TokenKind::Node) = parser.peek() {
        if let Some(TokenKind::Colon) = parser.peek_n(2) {
            name::alias(parser)
        }
        name::name(parser)
    } else {
        parser.push_err(create_err!(
            parser
                .peek_data()
                .unwrap_or_else(|| String::from("no further data")),
            "Expected Field to have a Name, got {}",
            parser
                .peek_data()
                .unwrap_or_else(|| String::from("no further data"))
        ));
    }
    match parser.peek() {
        Some(TokenKind::LParen) => argument::arguments(parser),
        Some(TokenKind::At) => directive::directives(parser),
        Some(TokenKind::LCurly) => selection::selection_set(parser),
        Some(TokenKind::Comma) => {
            guard.finish_node();
            parser.bump(SyntaxKind::COMMA);
            field(parser)
        }
        Some(TokenKind::Node) => {
            guard.finish_node();
            field(parser)
        }
        Some(TokenKind::RCurly) => {
            guard.finish_node();
        }
        _ => guard.finish_node(),
    }
}

/// See: https://spec.graphql.org/June2018/#FieldsDefinition
///
/// ```txt
/// FieldsDefinition
///     { FieldDefinition[list] }
/// ```
pub(crate) fn fields_definition(parser: &mut Parser) {
    let _guard = parser.start_node(SyntaxKind::FIELDS_DEFINITION);
    parser.bump(SyntaxKind::L_CURLY);
}
