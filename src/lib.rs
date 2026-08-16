use proc_macro::TokenStream;

pub(crate) mod helper;
pub(crate) mod fields;
pub(crate) mod versioning;

pub(crate) mod doc_section;
pub(crate) mod safety;
pub(crate) mod panics;
pub(crate) mod author;

/// Record the current version of an API item.
///
/// Exactly one `#[ver(...)]` per item. Renders a highlighted version heading;
/// when the status is `deprecated` (or `unstable` under the
/// `deprecated_for_unstable` feature), also emits a `#[deprecated]` attribute.
///
/// Grammar: `status, since = "…" [, note = "…"] [, date = "…"] [, author = "…"]`.
///
/// Historical version entries live on stacked `#[verlog(...)]` attributes.
///
/// ```ignore
/// #[ver(stable, since = "1.1.0", note = "Stabilised", author = "Akari")]
/// #[verlog(unstable, since = "0.1.0", note = "Prototype", author = "Redstone")]
/// pub fn my_api() { }
/// ```
///
/// The `;`-separated multi-entry form (`#[ver(a; b; c)]`) is no longer accepted.
#[proc_macro_attribute]
pub fn ver(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    let mut output = TokenStream::new();
    match versioning::generate_ver_docs(&mut cursor) {
        Ok(docs) => output.extend(docs),
        Err(err) => return err,
    }
    output.extend(item);
    output
}

/// Record a historical version entry, rendered as a plain (non-highlighted) log line.
///
/// Zero or more `#[verlog(...)]` per item; stack in the order they should appear in
/// the rendered docs (most-recent-first is the convention). Never emits
/// `#[deprecated]` — that role belongs solely to `#[ver]`.
///
/// Same field shape as `#[ver]`.
///
/// ```ignore
/// #[ver(stable, since = "1.1.0", author = "Akari")]
/// #[verlog(unstable, since = "0.1.0", author = "Redstone")]
/// pub fn my_api() { }
/// ```
#[proc_macro_attribute]
pub fn verlog(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut cursor = attr.into_iter().peekable();
    let mut output = TokenStream::new();
    match versioning::generate_verlog_docs(&mut cursor) {
        Ok(docs) => output.extend(docs),
        Err(err) => return err,
    }
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
