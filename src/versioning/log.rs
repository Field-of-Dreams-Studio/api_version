use core::iter::Peekable;

use proc_macro::{Ident, Literal, TokenStream, TokenTree};

use crate::fields::AttrLiteralFields;
use crate::helper::{
    expect_any_ident, expect_punct_consume, generate_compile_error, parse_string_literal_fields,
};

pub struct VerLog {
    pub ver_type: VerType,
    pub version: Literal,
    pub note: Option<Literal>,
    pub date: Option<Literal>,
}

impl VerLog {
    pub fn new(
        ver_type: VerType,
        version: Literal,
        note: Option<Literal>,
        date: Option<Literal>,
    ) -> Self {
        VerLog {
            ver_type,
            version,
            note,
            date,
        }
    }

    /// Parse one version entry from the cursor.
    ///
    /// Grammar: `status, since = "..." [, note = "..."] [, date = "..."]`.
    /// Authorship is recorded separately via `#[author(...)]`.
    /// The `;`-separated multi-entry form is no longer supported; a `;` at the top
    /// level yields a targeted migration error.
    pub fn from_tokens(
        cursor: &mut Peekable<impl Iterator<Item = TokenTree>>,
    ) -> Result<Self, TokenStream> {
        let ver_type_ident = expect_any_ident(
            cursor,
            "expected status (unstable, stable, update, update_unstable, deprecated)",
        )?;
        let ver_type = VerType::from_ident(ver_type_ident)?;

        expect_punct_consume(cursor, ",", "expected ',' after status")?;

        let pairs = parse_string_literal_fields(cursor)?;

        if let Some(TokenTree::Punct(punct)) = cursor.peek() {
            if punct.as_char() == ';' {
                return Err(generate_compile_error(
                    punct.span(),
                    "the `;`-separated multi-entry form is no longer supported; use one #[ver(...)] and stack additional #[verlog(...)] attributes",
                ));
            }
        }

        if let Some(token) = cursor.next() {
            return Err(generate_compile_error(
                token.span(),
                "unexpected token after version fields",
            ));
        }

        let mut fields: AttrLiteralFields = pairs.try_into()?;
        let version = fields.take_required("since")?;
        let note = fields.take_optional("note");
        let date = fields.take_optional("date");
        fields.reject_rest()?;

        Ok(VerLog::new(ver_type, version, note, date))
    }
}

pub enum VerType {
    Unstable,
    Stable,
    Update,
    UpdateUnstable,
    Deprecated,
}

impl VerType {
    pub fn label(&self) -> &'static str {
        match self {
            VerType::Unstable => "Unstable",
            VerType::Stable => "Stable",
            VerType::Update => "Update",
            VerType::UpdateUnstable => "UpdateUnstable",
            VerType::Deprecated => "Deprecated",
        }
    }

    pub fn from_ident(ident: Ident) -> Result<Self, TokenStream> {
        match ident.to_string().to_lowercase().as_str() {
            "unstable" => Ok(VerType::Unstable),
            "stable" => Ok(VerType::Stable),
            "update" => Ok(VerType::Update),
            "updateunstable" | "update_unstable" => Ok(VerType::UpdateUnstable),
            "deprecated" => Ok(VerType::Deprecated),
            _ => Err(generate_compile_error(
                ident.span(),
                "invalid status. Expected one of: unstable, stable, update, update_unstable, deprecated (case-insensitive)",
            )),
        }
    }
}
