use core::iter::Peekable;

use proc_macro::{Span, TokenStream, TokenTree};

use crate::doc_section::{DocSection, HeadingLevel, ListStyle};
use crate::helper::{
    expect_end, generate_compile_error, parse_string_literal_list,
};

pub fn generate_safety_docs(
    cursor: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<TokenStream, TokenStream> {
    let literals = parse_string_literal_list(cursor)?;
    expect_end(cursor, "unexpected token after safety conditions")?;

    if literals.is_empty() {
        return Err(generate_compile_error(
            Span::call_site(),
            "#[safety(...)] requires at least one condition string",
        ));
    }

    let items = literals
        .into_iter()
        .map(|lit| lit.to_string().trim_matches('"').to_string())
        .collect();

    Ok(DocSection {
        title: "Safety",
        heading_level: HeadingLevel::H1,
        preamble: Some("The caller must uphold:"),
        style: ListStyle::Numbered,
        items,
    }
    .render())
}
