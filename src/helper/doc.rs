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
