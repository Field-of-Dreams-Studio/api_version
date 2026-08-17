use core::iter::Peekable;

use proc_macro::{Literal, TokenStream, TokenTree};

use crate::doc_section::{DocSection, HeadingLevel, ListStyle};
use crate::fields::AttrLiteralFields;
use crate::helper::{expect_end, generate_compile_error, parse_string_literal_fields};

// TODO: merge sibling `#[cite]` attributes into a single `# Reference`
// section. Same sibling-blindness problem as `#[author]` — each invocation
// currently emits its own heading. Machinery for absorbing sibling attributes
// lives in `crate::helper::attrs` (`parse_outer_attr_bodies` +
// `match_outer_attr_list`); see the corresponding TODO in `src/author.rs`
// for the intended `(attr, item) -> (docs, rewritten_item)` migration.
pub fn generate_cite_docs(
    cursor: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<TokenStream, TokenStream> {
    let pairs = parse_string_literal_fields(cursor)?;
    expect_end(cursor, "unexpected token after cite fields")?;

    let mut fields: AttrLiteralFields = pairs.try_into()?;
    let work = fields.take_required("work")?;
    let volume = fields.take_optional("volume");
    let edition = fields.take_optional("edition");
    let chapter = fields.take_optional("chapter");
    let section = fields.take_optional("section");
    let listing = fields.take_optional("listing");
    let figure = fields.take_optional("figure");
    let page = fields.take_optional("page");
    let caption = fields.take_optional("caption");
    let doi = fields.take_optional("doi");
    let url = fields.take_optional("url");
    fields.reject_rest()?;

    if let Some(doi) = &doi {
        validate_doi(doi)?;
    }

    let mut entry = format!("**{}**", strip_quotes(&work));
    if let Some(volume) = volume {
        entry.push_str(&format!(", Vol. {}", strip_quotes(&volume)));
    }
    if let Some(edition) = edition {
        entry.push_str(&format!(", {}", strip_quotes(&edition)));
    }
    if let Some(chapter) = chapter {
        entry.push_str(&format!(", Chapter {}", strip_quotes(&chapter)));
    }
    if let Some(section) = section {
        entry.push_str(&format!(", {}", strip_quotes(&section)));
    }
    if let Some(listing) = listing {
        entry.push_str(&format!(", Listing {}", strip_quotes(&listing)));
    }
    if let Some(figure) = figure {
        entry.push_str(&format!(", Figure {}", strip_quotes(&figure)));
    }
    if let Some(page) = page {
        entry.push_str(&format!(" (page {})", strip_quotes(&page)));
    }
    if let Some(caption) = caption {
        entry.push_str(&format!(". \"{}\"", strip_quotes(&caption)));
    }
    if let Some(doi) = doi {
        let doi = strip_quotes(&doi);
        entry.push_str(&format!(". [doi:{doi}](https://doi.org/{doi})"));
    }
    if let Some(url) = url {
        entry.push_str(&format!(". <{}>", strip_quotes(&url)));
    }

    // Multiple `#[cite]` on one item each emit a `# Reference` heading;
    // rustdoc renders them separately. See the top-of-file TODO on merging.
    Ok(DocSection {
        title: "Reference",
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

/// Reject `doi` values that don't fit the standard DOI shape.
///
/// A DOI begins with the registrant prefix `10.` followed by a numeric
/// registrant code, then `/`, then an opaque suffix. We enforce only the two
/// pieces the spec calls out (§3): the `10.` prefix and the presence of `/`.
/// Stronger checks (numeric registrant, suffix character set) are deliberately
/// left out — user-facing metadata errors should be about typos, not conformance.
fn validate_doi(literal: &Literal) -> Result<(), TokenStream> {
    let value = strip_quotes(literal);
    if !value.starts_with("10.") || !value.contains('/') {
        return Err(generate_compile_error(
            literal.span(),
            &format!(
                "invalid DOI `{value}`: expected format `10.NNNN/...` \
                 (must start with `10.` and contain `/`)"
            ),
        ));
    }
    Ok(())
}
