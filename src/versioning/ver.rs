use core::iter::Peekable;

use proc_macro::{TokenStream, TokenTree};

use crate::helper::generate_doc_attribute;

use super::deprecation::generate_deprecated_attr;
use super::log::{VerLog, VerType};

/// Parse and render a `#[ver(...)]` attribute.
///
/// Emits a highlighted version heading plus, when applicable, a `#[deprecated]`
/// attribute (for `deprecated` status, or `unstable` under the
/// `deprecated_for_unstable` feature).
pub fn generate_ver_docs(
    cursor: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<TokenStream, TokenStream> {
    let verlog = VerLog::from_tokens(cursor)?;

    let mut output = TokenStream::new();
    output.extend(render_current(&verlog));
    output.extend(generate_deprecated_attr(&verlog));
    Ok(output)
}

fn render_current(verlog: &VerLog) -> TokenStream {
    let version = verlog.version.to_string();
    let version = version.trim_matches('"');

    let heading = match verlog.ver_type {
        VerType::Unstable => format!("### Unstable Version: {version}"),
        VerType::Stable => format!("### Stable Version: {version}"),
        VerType::Update => format!("### Updated Version: {version}"),
        VerType::UpdateUnstable => format!("### Unstable Modified Version: {version}"),
        VerType::Deprecated => format!("### Deprecated Version: {version}"),
    };

    let mut doc = format!("{heading}\n\n");
    append_optional(&mut doc, "Note", verlog.note.as_ref());
    append_optional(&mut doc, "Date", verlog.date.as_ref());

    generate_doc_attribute(&doc)
}

fn append_optional(doc: &mut String, label: &str, literal: Option<&proc_macro::Literal>) {
    if let Some(lit) = literal {
        let value = lit.to_string();
        let value = value.trim_matches('"');
        doc.push_str(&format!("{label}: {value}\n\n"));
    }
}
