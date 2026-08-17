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
    verlog.append_optional_fields(&mut doc);

    generate_doc_attribute(&doc)
}
