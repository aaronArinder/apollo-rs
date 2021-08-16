use crate::{format_err, name, named_type, variable_definition, Parser, SyntaxKind, TokenKind};

/// OperationTypeDefinition is used in a SchemaDefinition. Not to be confused
/// with OperationDefinition.
///
/// See: https://spec.graphql.org/June2018/#RootOperationTypeDefinition
///
/// ```txt
/// OperationTypeDefinition
///    OperationType : NamedType
/// ```
pub(crate) fn operation_type_definition(
    parser: &mut Parser,
    is_operation_type: bool,
) -> Result<(), crate::Error> {
    if let Some(TokenKind::Comma) = parser.peek() {
        parser.bump(SyntaxKind::COMMA);
        return operation_type_definition(parser, is_operation_type);
    }

    if let Some(TokenKind::Node) = parser.peek() {
        let guard = parser.start_node(SyntaxKind::OPERATION_TYPE_DEFINITION);
        operation_type(parser)?;
        if let Some(TokenKind::Colon) = parser.peek() {
            parser.bump(SyntaxKind::COLON);
            named_type(parser)?;
            if parser.peek().is_some() {
                guard.finish_node();
                return operation_type_definition(parser, true);
            }
            return Ok(());
        } else {
            return format_err!(
                parser.peek_data().unwrap(),
                "Expected Operation Type to have a Name Type, got {}",
                parser.peek_data().unwrap()
            );
        }
    }

    if is_operation_type {
        Ok(())
    } else {
        return format_err!(
            parser.peek_data().unwrap(),
            "Expected Schema Definition to have an Operation Type, got {}",
            parser.peek_data().unwrap()
        );
    }
}

/// See: https://spec.graphql.org/June2018/#OperationDefinition
///
/// ```txt
/// OperationDefinition
///    OperationType Name VariableDefinitions Directives SelectionSet
///    Selection Set (TODO)
/// ```

pub(crate) fn operation_definition(parser: &mut Parser) -> Result<(), crate::Error> {
    let _guard = parser.start_node(SyntaxKind::OPERATION_DEFINITION);
    operation_type(parser)?;
    if let Some(TokenKind::Node) = parser.peek() {
        name(parser)?;
    }

    if let Some(TokenKind::LParen) = parser.peek() {
        parser.bump(SyntaxKind::L_PAREN);
        match parser.peek() {
            // variable definition
            Some(TokenKind::Dollar) => {
                let _guard = parser.start_node(SyntaxKind::VARIABLE_DEFINITIONS);
                variable_definition(parser, false)?;
            }
            // directive definition
            Some(TokenKind::At) => todo!(),
            // error: expected a vairable definition or a directive name to follow an opening brace
            _ => todo!(),
        }
    }
    // this is a selection set
    if let Some(TokenKind::LCurly) = parser.peek() {}
    Ok(())
}

/// See: https://spec.graphql.org/June2018/#OperationType
///
/// ```txt
/// OperationType : one of
///    query    mutation    subscription
/// ```
pub(crate) fn operation_type(parser: &mut Parser) -> Result<(), crate::Error> {
    if let Some(node) = parser.peek_data() {
        let _guard = parser.start_node(SyntaxKind::OPERATION_TYPE);
        match node.as_str() {
            "query" => parser.bump(SyntaxKind::query_KW),
            "subscription" => parser.bump(SyntaxKind::subscription_KW),
            "mutation" => parser.bump(SyntaxKind::mutation_KW),
            _ => {
                return format_err!(
                    parser
                        .peek_data()
                        .unwrap_or_else(|| String::from("no further data")),
                    "Operation Type must be either 'mutation', 'query' or 'subscription', got {}",
                    parser
                        .peek_data()
                        .unwrap_or_else(|| String::from("no further data"))
                )
            }
        }
    }

    Ok(())
}
