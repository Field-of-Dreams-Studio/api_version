use core::iter::Peekable;

use proc_macro::{Span, TokenStream, TokenTree};

use crate::doc_section::{DocSection, ListStyle};
use crate::helper::{
    expect_end, generate_compile_error, generate_doc_attribute, parse_string_literal_list,
};

pub fn generate_panics_docs(
    cursor: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<TokenStream, TokenStream> {
    if is_no_panic_sentinel(cursor.peek()) {
        cursor.next();
        expect_end(
            cursor,
            "the 'never'/'none' sentinel is exclusive; remove additional arguments",
        )?;
        return Ok(generate_doc_attribute(
            "## Panics\n\nThis function does not panic.",
        ));
    }

    let literals = parse_string_literal_list(cursor)?;
    expect_end(cursor, "unexpected token after panic conditions")?;

    if literals.is_empty() {
        return Err(generate_compile_error(
            Span::call_site(),
            "#[panics(...)] requires at least one condition string, or the 'never'/'none' sentinel",
        ));
    }

    let items = literals
        .into_iter()
        .map(|lit| lit.to_string().trim_matches('"').to_string())
        .collect();

    Ok(DocSection {
        title: "Panics",
        preamble: Some("This function panics when:"),
        style: ListStyle::Numbered,
        items,
    }
    .render())
}

fn is_no_panic_sentinel(token: Option<&TokenTree>) -> bool {
    matches!(
        token,
        Some(TokenTree::Ident(ident))
            if matches!(ident.to_string().as_str(), "never" | "none")
    )
}
