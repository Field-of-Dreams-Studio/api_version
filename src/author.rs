use core::iter::Peekable;

use proc_macro::{Literal, TokenStream, TokenTree};

use crate::doc_section::{DocSection, ListStyle};
use crate::fields::AttrLiteralFields;
use crate::helper::{expect_end, parse_string_literal_fields};

pub fn generate_author_docs(
    cursor: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<TokenStream, TokenStream> {
    let pairs = parse_string_literal_fields(cursor)?;
    expect_end(cursor, "unexpected token after author fields")?;

    let mut fields: AttrLiteralFields = pairs.try_into()?;
    let name = fields.take_required("name")?;
    let email = fields.take_optional("email");
    let github = fields.take_optional("github");
    let role = fields.take_optional("role");
    fields.reject_rest()?;

    let mut entry = strip_quotes(&name);
    if let Some(email) = email {
        entry.push_str(&format!(" <{}>", strip_quotes(&email)));
    }
    let mut extras = Vec::new();
    if let Some(github) = github {
        extras.push(format!("github: {}", strip_quotes(&github)));
    }
    if let Some(role) = role {
        extras.push(format!("role: {}", strip_quotes(&role)));
    }
    if !extras.is_empty() {
        entry.push_str(&format!(" ({})", extras.join(", ")));
    }

    // Multiple `#[author]` on one item each emit a `## Authors` heading;
    // rustdoc renders both. Single-author is the polished case.
    Ok(DocSection {
        title: "Authors",
        preamble: None,
        style: ListStyle::Bulleted,
        items: vec![entry],
    }
    .render())
}

fn strip_quotes(literal: &Literal) -> String {
    literal.to_string().trim_matches('"').to_string()
}
