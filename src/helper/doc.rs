use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// Generate a `#[doc = "..."]` attribute from a string.
///
/// Example: `generate_doc_attribute("Hello")` produces `#[doc = "Hello"]`.
pub fn generate_doc_attribute(doc_string: &str) -> TokenStream {
    let mut tokens = TokenStream::new();

    let hash = TokenTree::Punct(Punct::new('#', Spacing::Alone));

    let mut inner = TokenStream::new();
    inner.extend(vec![
        TokenTree::Ident(Ident::new("doc", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Literal(Literal::string(doc_string)),
    ]);

    let group = TokenTree::Group(Group::new(Delimiter::Bracket, inner));

    tokens.extend(vec![hash, group]);
    tokens
}

/// Insert `docs` into `item` after any leading `#[...]` attributes and before
/// the actual item body.
///
/// This lets an attribute macro place its generated `#[doc = "..."]` output
/// AFTER the user's `///` doc comments (which desugar to `#[doc]` attributes),
/// so the rendered rustdoc shows the user's description first and the
/// macro-generated structured sections (Safety, Panics, Version, etc.) after —
/// matching the standard Rust doc layout used by `std`.
///
/// Without this helper, a macro doing `output = [my_docs, item]` places its
/// docs BEFORE the user's `///`, inverting the natural reading order.
pub fn insert_docs_before_body(item: TokenStream, docs: TokenStream) -> TokenStream {
    let mut cursor = item.into_iter().peekable();
    let mut leading = TokenStream::new();

    // Consume every leading `#[...]` attribute pair (`#` then a bracketed group).
    // Stop as soon as the next token is not an outer-attribute marker.
    while matches!(cursor.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '#') {
        let hash = cursor.next().expect("just peeked `#`");
        match cursor.peek() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
                let group = cursor.next().expect("just peeked attribute group");
                leading.extend(std::iter::once(hash));
                leading.extend(std::iter::once(group));
            }
            _ => {
                // Malformed: `#` not followed by `[...]`. Preserve `#` and stop.
                leading.extend(std::iter::once(hash));
                break;
            }
        }
    }

    let mut out = leading;
    out.extend(docs);
    out.extend(cursor);
    out
}
