use proc_macro::{TokenStream, TokenTree};
use crate::versions::VerLog;
use crate::deprecation::generate_deprecated_attr;

use super::helper::*; 

/// Separate a TokenStream by a given punctuation character. 
/// Returns a vector of TokenStreams, each representing a segment between the punctuation. 
fn seperate_by_punct(
    tokens: TokenStream, 
    punct: char
) -> Vec<TokenStream> {
    let mut result = Vec::new();
    let mut current = TokenStream::new();
    let mut iter = tokens.into_iter();
    while let Some(token) = iter.next() {
        match &token {
            TokenTree::Punct(p) if p.as_char() == punct => {
                result.push(current);
                current = TokenStream::new();
            }
            _ => {
                current.extend(std::iter::once(token));
            }
        }
    }
    result.push(current);
    result
} 

pub fn from_tokens(tokens: TokenStream) -> Result<Vec<VerLog>, TokenStream> {
    let segments = seperate_by_punct(tokens, ';'); 
    let mut verlogs = Vec::new(); 
    for segment in segments {
        let verlog = VerLog::from_tokens(segment)?; 
        verlogs.push(verlog); 
    } 
    Ok(verlogs) 
} 

pub fn into_doc_attrs(verlogs: &[VerLog]) -> TokenStream {
    let mut result = TokenStream::new();

    for (index, verlog) in verlogs.iter().enumerate() {
        if index == 0 {
            // First element is the most recent version - use enhanced formatting
            let doc_attr = verlog.last_doc_attr();
            result.extend(doc_attr);

            // Generate #[deprecated] attribute for the most recent version
            let deprecated_attr = generate_deprecated_attr(verlog);
            result.extend(deprecated_attr);
        } else {
            // Older versions use simple formatting
            let doc_attr = verlog.into_doc_attr();
            result.extend(doc_attr);
        }
    }

    result
} 
