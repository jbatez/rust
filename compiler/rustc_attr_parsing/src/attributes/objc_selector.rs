use rustc_ast::token::{Delimiter, Token, TokenKind};
use rustc_ast::tokenstream::TokenTree;
use rustc_ast::{AttrArgs, Attribute, DelimArgs};
use rustc_attr_data_structures::AttributeKind;
use rustc_errors::{Applicability, DiagCtxtHandle};
use rustc_parse::parser::token_descr;
use rustc_span::{Symbol, sym};

pub(crate) fn parse_attr_objc_selector(
    attr: &Attribute,
    dcx: DiagCtxtHandle<'_>,
) -> Option<AttributeKind> {
    // Make sure the attribute matches the form `#[rustc_objc_selector(...)]`.
    assert!(attr.has_name(sym::rustc_objc_selector));
    let AttrArgs::Delimited(DelimArgs { ref dspan, delim: Delimiter::Parenthesis, ref tokens }) =
        attr.get_normal_item().args
    else {
        dcx.struct_span_err(attr.span, "malformed `rustc_objc_selector` attribute input")
            .with_span_suggestion(
                attr.span,
                "must be of the form",
                "#[rustc_objc_selector(methname)]",
                Applicability::HasPlaceholders,
            )
            .emit();
        return None;
    };

    // Parse the method name inside the parentheses.
    // An Objective-C selector method name can be either:
    // - a single identifier or
    // - a series of `:`, each optionally preceded by an identifier.
    let mut methname = String::new();
    let mut token_iter = tokens.iter();
    loop {
        let mut token_tree = token_iter.next();

        // Check if the current token tree is an identifier.
        // If it is, append it to the method name and get the next token tree.
        let (may_terminate, expect_ident) = match token_tree {
            Some(TokenTree::Token(Token { kind: TokenKind::Ident(ident, _), span }, _)) => {
                // The method name may terminate if it's just a single identifier.
                // Every other identifier requires a colon immediately after.
                let may_terminate = methname.is_empty();
                if !may_terminate && token_iter.peek().is_none() {
                    // Since the CloseParen is usually inside a macro invocation,
                    // we point the error at the identifier instead.
                    let msg = "expected `:` after this identifier";
                    dcx.struct_span_err(*span, msg).with_span_label(*span, msg).emit();
                    return None;
                }

                methname.push_str(ident.as_str());
                token_tree = token_iter.next();

                // Don't expect an identifier immediately after an identifier.
                let expect_ident = false;
                (may_terminate, expect_ident)
            }
            _ => {
                let may_terminate = !methname.is_empty();
                let expect_ident = true;
                (may_terminate, expect_ident)
            }
        };

        // Determine the next token kind and span.
        let token = match token_tree {
            Some(TokenTree::Token(token, _)) => *token,
            Some(TokenTree::Delimited(dspan, _, delim, _)) => {
                Token::new(delim.as_open_token_kind(), dspan.open)
            }
            None => Token::new(TokenKind::CloseParen, dspan.close),
        };

        // Append colons to the method name, terminate, or emit an error.
        match token.kind {
            TokenKind::Colon => {
                methname.push(':');
            }
            TokenKind::PathSep => {
                methname.push_str("::");
            }
            TokenKind::CloseParen if may_terminate => {
                let methname = Symbol::intern(&methname);
                return Some(AttributeKind::ObjcSelector(methname));
            }
            _ => {
                let label = match expect_ident {
                    true => "expected identifier or `:`",
                    false => "expected `:`",
                };
                let found = token_descr(&token);
                let msg = format!("{label}, found {found}");
                dcx.struct_span_err(token.span, msg).with_span_label(token.span, label).emit();
                return None;
            }
        }
    }
}
