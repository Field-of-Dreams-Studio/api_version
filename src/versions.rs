use proc_macro::{Ident, Literal, Span, TokenStream};
use super::helper::*; 

pub struct VerLog {
    pub ver_type: VerType, 
    pub version: Literal, 
    pub note: Option<Literal>,
    pub date: Option<Literal>, 
    pub author: Option<Literal>, 
} 

impl VerLog { 
    pub fn new(
        ver_type: VerType, 
        version: Literal, 
        note: Option<Literal>, 
        date: Option<Literal>,
        author: Option<Literal>,
    ) -> Self {
        VerLog {
            ver_type,
            version,
            note,
            date,
            author,
        }
    } 

    /// Parse a VerLog from a TokenStream.
    /// The expected format is:
    /// status, since = "version", [note = "..."], [date = "..."], [author = "..."]
    pub fn from_tokens(tokens: TokenStream) -> Result<Self, TokenStream> {
        let mut tokens = into_peekable_iter(tokens);

        // Parse status (e.g., unstable, deprecated)
        let ver_type_ident = expect_any_ident(&mut tokens, "Expected status (unstable, stable, deprecated, etc.)")?;
        let ver_type = VerType::from_ident(ver_type_ident)?;

        // Consume comma after status
        expect_punct_consume(&mut tokens, ",", "Expected ',' after status")?;

        // Parse since = "version"
        expect_ident_consume(&mut tokens, "since", "Expected 'since' field")?;
        expect_punct_consume(&mut tokens, "=", "Expected '=' after 'since'")?;
        let version = expect_literal_consume(&mut tokens, "Expected version literal after 'since ='")?;

        // Parse optional fields (note, date, author)
        let mut note = None;
        let mut date = None;
        let mut author = None;

        while match_punct_consume(&mut tokens, ",") {
            if match_ident_consume(&mut tokens, "note") {
                expect_punct_consume(&mut tokens, "=", "Expected '=' after 'note'")?;
                note = Some(expect_literal_consume(&mut tokens, "Expected note literal")?);
            } else if match_ident_consume(&mut tokens, "date") {
                expect_punct_consume(&mut tokens, "=", "Expected '=' after 'date'")?;
                date = Some(expect_literal_consume(&mut tokens, "Expected date literal")?);
            } else if match_ident_consume(&mut tokens, "author") {
                expect_punct_consume(&mut tokens, "=", "Expected '=' after 'author'")?;
                author = Some(expect_literal_consume(&mut tokens, "Expected author literal")?);
            } else {
                return Err(generate_compile_error(
                    Span::call_site(),
                    "Expected one of: note, date, author"
                ));
            }
        }

        Ok(VerLog::new(ver_type, version, note, date, author))
    } 

    pub fn into_doc_attr(&self) -> TokenStream {
        let mut doc_string = format!("Version: {}, **{}**\n\n", self.version.to_string().trim_matches('"'), self.ver_type.to_string());
        if let Some(note) = &self.note {
            doc_string.push_str(&format!("Note: {}\n\n", note.to_string().trim_matches('"')));
        }
        if let Some(date) = &self.date {
            doc_string.push_str(&format!("Date: {}\n\n", date.to_string().trim_matches('"')));
        }
        if let Some(author) = &self.author {
            doc_string.push_str(&format!("Author: {}\n\n", author.to_string().trim_matches('"')));
        }
        generate_doc_attribute(&doc_string)
    } 

    pub fn last_doc_attr(&self) -> TokenStream {
        let mut doc_string = String::new(); 
        match self.ver_type { 
            VerType::Unstable => {
                doc_string.push_str(&format!("### Unstable Version: {}\n\n", self.version.to_string().trim_matches('"')));
            },
            VerType::Stable => {
                doc_string.push_str(&format!("### Stable Version: {}\n\n", self.version.to_string().trim_matches('"')));
            },
            VerType::Update => {
                doc_string.push_str(&format!("### Updated Version: {}\n\n", self.version.to_string().trim_matches('"')));
            },
            VerType::UpdateUnstable => {
                doc_string.push_str(&format!("### Unstable Modified Version: {}\n\n", self.version.to_string().trim_matches('"')));
            },
            VerType::Deprecated => {
                doc_string.push_str(&format!("### Deprecated Version: {}\n\n", self.version.to_string().trim_matches('"')));
            }, 
        }
        if let Some(note) = &self.note {
            doc_string.push_str(&format!("Note: {}\n\n", note.to_string().trim_matches('"')));
        }
        if let Some(date) = &self.date {
            doc_string.push_str(&format!("Date: {}\n\n", date.to_string().trim_matches('"')));
        }
        if let Some(author) = &self.author {
            doc_string.push_str(&format!("Author: {}\n\n", author.to_string().trim_matches('"')));
        }
        generate_doc_attribute(&doc_string)
    }  
}

pub enum VerType {
    Unstable,
    Stable, 
    Update, 
    UpdateUnstable, 
    Deprecated 
} 

impl VerType { 
    fn to_string(&self) -> String {
        match self {
            VerType::Unstable => "Unstable".to_string(),
            VerType::Stable => "Stable".to_string(),
            VerType::Update => "Update".to_string(),
            VerType::UpdateUnstable => "UpdateUnstable".to_string(),
            VerType::Deprecated => "Deprecated".to_string() 
        }
    } 

    fn from_ident(ident: Ident) -> Result<Self, TokenStream> {
        // Convert to lowercase for case-insensitive matching
        match ident.to_string().to_lowercase().as_str() {
            "unstable" => Ok(VerType::Unstable),
            "stable" => Ok(VerType::Stable),
            "update" => Ok(VerType::Update),
            "updateunstable" | "update_unstable" => Ok(VerType::UpdateUnstable),
            "deprecated" => Ok(VerType::Deprecated),
            _ => Err(generate_compile_error(
                ident.span(),
                "Invalid status. Expected one of: unstable, stable, update, update_unstable, deprecated (case-insensitive)"
            )),
        }
    } 
} 
