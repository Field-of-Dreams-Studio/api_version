use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::versions::{VerLog, VerType};

/// Generate a #[deprecated] attribute based on the version log and feature flags.
///
/// Behavior:
/// - `deprecated` status: Always emits #[deprecated(since = "...", note = "...")]
/// - `unstable` status:
///   - With `deprecated_for_unstable` feature: Emits #[deprecated] with "[UNSTABLE]" prefix
///   - Without feature: Returns empty TokenStream (no warning)
pub fn generate_deprecated_attr(verlog: &VerLog) -> TokenStream {
    match verlog.ver_type {
        VerType::Deprecated => {
            // Always emit deprecated for deprecated items
            build_deprecated_attr(&verlog.version, verlog.note.as_ref(), None)
        }
        VerType::Unstable => {
            // Only emit if feature flag is enabled
            #[cfg(feature = "deprecated_for_unstable")]
            {
                build_deprecated_attr(&verlog.version, verlog.note.as_ref(), Some("[UNSTABLE] "))
            }

            #[cfg(not(feature = "deprecated_for_unstable"))]
            {
                TokenStream::new() // No warning when feature is disabled
            }
        }
        _ => {
            // Other status types don't emit deprecated warnings
            TokenStream::new()
        }
    }
}

/// Build a #[deprecated(since = "version", note = "message")] attribute.
///
/// Parameters:
/// - `version`: The version literal (e.g., "0.1.0")
/// - `note`: Optional custom note
/// - `prefix`: Optional prefix to prepend to the note (e.g., "[UNSTABLE] ")
fn build_deprecated_attr(
    version: &Literal,
    note: Option<&Literal>,
    prefix: Option<&str>,
) -> TokenStream {
    let mut tokens = TokenStream::new();

    // #[deprecated(
    tokens.extend(vec![
        TokenTree::Punct(Punct::new('#', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Bracket,
            {
                let mut inner = TokenStream::new();
                inner.extend(vec![TokenTree::Ident(Ident::new("deprecated", Span::call_site()))]);

                // Build the argument group: (since = "...", note = "...")
                inner.extend(vec![TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    build_deprecated_args(version, note, prefix),
                ))]);

                inner
            },
        )),
    ]);

    tokens
}

/// Build the arguments for the deprecated attribute: since = "...", note = "..."
fn build_deprecated_args(
    version: &Literal,
    note: Option<&Literal>,
    prefix: Option<&str>,
) -> TokenStream {
    let mut args = TokenStream::new();

    // since = "version"
    args.extend(vec![
        TokenTree::Ident(Ident::new("since", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Literal(version.clone()),
    ]);

    // note = "message" (if provided)
    if let Some(note_lit) = note {
        args.extend(vec![
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            TokenTree::Ident(Ident::new("note", Span::call_site())),
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        ]);

        // If there's a prefix, prepend it to the note
        if let Some(prefix_str) = prefix {
            let note_content = note_lit.to_string();
            // Remove quotes, prepend prefix, re-quote
            let trimmed = note_content.trim_matches('"');
            let prefixed_note = format!("{}{}", prefix_str, trimmed);
            args.extend(vec![TokenTree::Literal(Literal::string(&prefixed_note))]);
        } else {
            args.extend(vec![TokenTree::Literal(note_lit.clone())]);
        }
    } else if prefix.is_some() {
        // If there's a prefix but no custom note, create a default message
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
