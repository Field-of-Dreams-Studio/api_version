use proc_macro::TokenStream;

pub(crate) mod helper;
pub(crate) mod versions;
pub(crate) mod loglist;
pub(crate) mod deprecation;

/// The main attribute macro to annotate functions, structs, etc. with versioning information. 
/// It is able to record multiple version entries, each separated by a semicolon. 
/// Where if the last version is marked as deprecated, it will generate a #[deprecated] attribute to warn users. 
/// 
/// # Example 
/// 
/// The minimal example (Which will warn user): 
/// 
/// ```ignore
/// #[ver(deprecated, since = "0.1.0")]
/// pub fn minimal_example() {
///     println!("This is a minimal example");
/// } 
/// ```
/// 
/// The full example with multiple versions: 
/// 
/// (This will not warn user as the latest version is not deprecated. Documentation will be generated for all versions as a log. Only the latest version is shown as highlighted.) 
/// 
/// ```ignore
/// #[ver(
///     update, since = "1.2.0", note = "Added new parameter", date = "2024-03-01", author = "Akari";
///     stable, since = "1.1.0", note = "First stable release", date = "2024-02-01", author = "Akari";
///     unstable, since = "0.1.0", note = "Initial implementation", date = "2024-01-01", author = "Akari"
/// )]
/// pub fn full_example(value: i32, new_param: bool) {
///     println!("Value: {}, New Param: {}", value, new_param);
/// }
/// ``` 
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
