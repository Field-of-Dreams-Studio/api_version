use proc_macro::TokenStream;

pub(crate) mod helper;
pub(crate) mod versions;
pub(crate) mod loglist;
pub(crate) mod deprecation;

#[proc_macro_attribute]
pub fn ver(attr: TokenStream, item: TokenStream) -> TokenStream {
    let verlogs = match loglist::from_tokens(attr) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let mut output = TokenStream::new();

    // Prepend doc attributes and #[deprecated] (for most recent version only)
    output.extend(loglist::into_doc_attrs(&verlogs));

    // Append the original item
    output.extend(item);

    output
} 
