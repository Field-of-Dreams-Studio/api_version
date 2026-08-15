use proc_macro::TokenStream;

pub(crate) mod helper;
pub(crate) mod fields;
pub(crate) mod versions;
pub(crate) mod loglist;
pub(crate) mod deprecation;

pub(crate) mod doc_section;
pub(crate) mod safety;
pub(crate) mod panics;
pub(crate) mod author;

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

/// Document the preconditions a caller must uphold when invoking an `unsafe fn`.
///
/// Accepts a comma-separated list of string literals; each is rendered as one entry
/// in a numbered list under `## Safety`.
///
/// ```ignore
/// #[safety(
///     "fd is a valid open file descriptor",
///     "buf points to at least count readable bytes"
/// )]
/// pub unsafe fn raw_write(fd: i32, buf: *const u8, count: usize) -> isize { ... }
/// ```
#[proc_macro_attribute]
pub fn safety(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    let mut output = TokenStream::new();
    match safety::generate_safety_docs(&mut cursor) {
        Ok(docs) => output.extend(docs),
        Err(err) => return err,
    }
    output.extend(item);
    output
}

/// Document when a function panics.
///
/// Accepts a comma-separated list of string literals rendered under `## Panics`, or
/// the bare sentinel `never` / `none` to document that the function does not panic.
///
/// ```ignore
/// #[panics("when index >= self.len()", "when the lock is poisoned")]
/// pub fn get(&self, index: usize) -> &T { ... }
///
/// #[panics(never)]
/// pub fn try_get(&self) -> Option<&T> { ... }
/// ```
#[proc_macro_attribute]
pub fn panics(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    let mut output = TokenStream::new();
    match panics::generate_panics_docs(&mut cursor) {
        Ok(docs) => output.extend(docs),
        Err(err) => return err,
    }
    output.extend(item);
    output
}

/// Record the author(s) of a function or type under `## Authors`.
///
/// Each `#[author(...)]` records one entry. Fields: `name` (required),
/// `email`, `github`, `role` (optional).
///
/// ```ignore
/// #[author(name = "Redstone", email = "redstone@example.com")]
/// #[author(name = "Akari", github = "akari", role = "maintainer")]
/// pub fn my_api() { }
/// ```
#[proc_macro_attribute]
pub fn author(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    let mut output = TokenStream::new();
    match author::generate_author_docs(&mut cursor) {
        Ok(docs) => output.extend(docs),
        Err(err) => return err,
    }
    output.extend(item);
    output
}
