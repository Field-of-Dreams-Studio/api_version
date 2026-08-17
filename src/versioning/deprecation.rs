use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use super::log::{VerLog, VerType};

/// Generate a `#[deprecated]` attribute based on the version log and feature flags.
///
/// Behavior:
/// - `deprecated` status: always emits `#[deprecated(since = "...", note = "...")]`.
/// - `unstable` status:
///   - with the `deprecated_for_unstable` feature: emits `#[deprecated]` with an
///     `[UNSTABLE]` note prefix.
///   - without the feature: returns an empty TokenStream.
pub fn generate_deprecated_attr(verlog: &VerLog) -> TokenStream {
    match verlog.ver_type {
        VerType::Deprecated => build_deprecated_attr(&verlog.version, verlog.note.as_ref(), None),
        VerType::Unstable => {
            #[cfg(feature = "deprecated_for_unstable")]
            {
                build_deprecated_attr(&verlog.version, verlog.note.as_ref(), Some("[UNSTABLE] "))
            }

            #[cfg(not(feature = "deprecated_for_unstable"))]
            {
                TokenStream::new()
            }
        }
        _ => TokenStream::new(),
    }
}

fn build_deprecated_attr(
    version: &Literal,
    note: Option<&Literal>,
    prefix: Option<&str>,
) -> TokenStream {
    let mut tokens = TokenStream::new();

    tokens.extend(vec![
        TokenTree::Punct(Punct::new('#', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Bracket, {
            let mut inner = TokenStream::new();
            inner.extend(vec![TokenTree::Ident(Ident::new(
                "deprecated",
                Span::call_site(),
            ))]);
            inner.extend(vec![TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                build_deprecated_args(version, note, prefix),
            ))]);
            inner
        })),
    ]);

    tokens
}

fn build_deprecated_args(
    version: &Literal,
    note: Option<&Literal>,
    prefix: Option<&str>,
) -> TokenStream {
    let mut args = TokenStream::new();

    args.extend(vec![
        TokenTree::Ident(Ident::new("since", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Literal(version.clone()),
    ]);

    if let Some(note_lit) = note {
        args.extend(vec![
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            TokenTree::Ident(Ident::new("note", Span::call_site())),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        ]);

        if let Some(prefix_str) = prefix {
            let note_content = note_lit.to_string();
            let trimmed = note_content.trim_matches('"');
            let prefixed_note = format!("{prefix_str}{trimmed}");
            args.extend(vec![TokenTree::Literal(Literal::string(&prefixed_note))]);
        } else {
            args.extend(vec![TokenTree::Literal(note_lit.clone())]);
        }
    } else if prefix.is_some() {
        args.extend(vec![
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            TokenTree::Ident(Ident::new("note", Span::call_site())),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Literal(Literal::string(
                prefix.unwrap_or("This API may change without notice."),
            )),
        ]);
    }

    args
}
