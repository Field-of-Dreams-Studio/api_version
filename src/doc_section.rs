use proc_macro::TokenStream;

use crate::helper::generate_doc_attribute;

pub enum ListStyle {
    Numbered,
    Bulleted,
}

#[allow(dead_code)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
}

/// A rustdoc section rendered as a titled list, e.g.
///
/// ```text
/// # Safety
/// The caller must uphold:
///
/// 1. first condition
/// 2. second condition
/// ```
///
/// Emitted as a single `#[doc = "..."]` attribute.
pub struct DocSection<'a> {
    pub title: &'a str,
    pub heading_level: HeadingLevel,
    pub preamble: Option<&'a str>,
    pub style: ListStyle,
    pub items: Vec<String>,
}

impl DocSection<'_> {
    pub fn render(&self) -> TokenStream {
        let marker = match self.heading_level {
            HeadingLevel::H1 => "#",
            HeadingLevel::H2 => "##",
            HeadingLevel::H3 => "###",
        };
        let mut doc = format!("{marker} {}\n", self.title);
        if let Some(preamble) = self.preamble {
            doc.push_str(preamble);
            doc.push_str("\n\n");
        } else {
            doc.push('\n');
        }
        for (index, item) in self.items.iter().enumerate() {
            match self.style {
                ListStyle::Numbered => doc.push_str(&format!("{}. {}\n", index + 1, item)),
                ListStyle::Bulleted => doc.push_str(&format!("- {}\n", item)),
            }
        }
        generate_doc_attribute(&doc)
    }
}
