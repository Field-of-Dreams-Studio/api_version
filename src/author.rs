use core::iter::Peekable;

use proc_macro::{Literal, TokenStream, TokenTree};

use crate::doc_section::{DocSection, HeadingLevel, ListStyle};
use crate::fields::AttrLiteralFields;
use crate::helper::{expect_end, parse_string_literal_fields};

// TODO: merge sibling `#[author]` attributes into a single `# Authors` section.
//
// Current behaviour: each `#[author(...)]` invocation emits its own
// `# Authors` heading + one bullet. When multiple `#[author]`s stack on one
// item, rustdoc renders repeated `# Authors` headings — one per attribute.
//
// Intended behaviour: the outermost (top-most) `#[author]` should scan the
// `item` TokenStream passed to it, extract any subsequent `#[author(...)]`
// attributes on the same item, remove them from `item`, and render one
// combined `# Authors` section with a bullet per author. Inner `#[author]`
// attributes, once absorbed, must not run — either by stripping them from
// `item` (so rustc never sees them as attribute macros) or by making them
// no-ops when they detect they've been absorbed.
//
// Machinery for this is already available in `crate::helper::attrs`:
//   - `parse_outer_attr_bodies(cursor)` — collect all leading outer-attribute
//     bodies from a token stream.
//   - `match_outer_attr_list(body, "author")` — filter for `#[author(...)]`
//     specifically and return the inner arg tokens.
//
// The macro signature would change to take `item` as well:
//   pub fn generate_author_docs(attr, item) -> (TokenStream /* docs */,
//                                               TokenStream /* rewritten item */)
// and lib.rs would forward `attr` and `item` and re-emit the rewritten item.
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

    // Multiple `#[author]` on one item each emit a `# Authors` heading;
    // rustdoc renders both. Single-author is the polished case.
    Ok(DocSection {
        title: "Authors",
        heading_level: HeadingLevel::H1,
        preamble: None,
        style: ListStyle::Bulleted,
        items: vec![entry],
    }
    .render())
}

fn strip_quotes(literal: &Literal) -> String {
    literal.to_string().trim_matches('"').to_string()
}
