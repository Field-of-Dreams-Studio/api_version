use core::iter::Peekable;

use proc_macro::{TokenStream, TokenTree};

use crate::helper::generate_doc_attribute;

use super::log::VerLog;

/// Parse and render a `#[verlog(...)]` attribute.
///
/// Emits a plain, non-highlighted historical entry. Never emits `#[deprecated]`
/// — that role belongs solely to `#[ver]`.
pub fn generate_verlog_docs(
    cursor: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<TokenStream, TokenStream> {
    let verlog = VerLog::from_tokens(cursor)?;
    Ok(render_historical(&verlog))
}

fn render_historical(verlog: &VerLog) -> TokenStream {
    let version = verlog.version.to_string();
    let version = version.trim_matches('"');
    let ver_type_label = verlog.ver_type.label();

    let mut doc = format!("Version: {version}, **{ver_type_label}**\n\n");
    append_optional(&mut doc, "Note", verlog.note.as_ref());
    append_optional(&mut doc, "Date", verlog.date.as_ref());
    append_optional(&mut doc, "Author", verlog.author.as_ref());

    generate_doc_attribute(&doc)
}

fn append_optional(doc: &mut String, label: &str, literal: Option<&proc_macro::Literal>) {
    if let Some(lit) = literal {
        let value = lit.to_string();
        let value = value.trim_matches('"');
        doc.push_str(&format!("{label}: {value}\n\n"));
    }
}
