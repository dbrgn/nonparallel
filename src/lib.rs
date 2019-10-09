//! A procedural macro for Rust that allows you to ensure that functions (e.g.
//! unit tests) are not running at the same time.
//!
//! This is achieved by acquiring a mutex at the beginning of the annotated
//! function.
//!
//! Different functions can synchronize on different mutexes. That's why a
//! static mutex reference must be passed to the `nonparallel` annotation.
//!
//! ## Usage
//!
//! ```rust
//! use std::sync::Mutex;
//! use lazy_static::lazy_static;
//! use nonparallel::nonparallel;
//!
//! // Create two locks
//! lazy_static! { static ref MUT_A: Mutex<()> = Mutex::new(()); }
//! lazy_static! { static ref MUT_B: Mutex<()> = Mutex::new(()); }
//!
//! // Mutually exclude parallel runs of functions using those two locks
//!
//! #[nonparallel(MUT_A)]
//! fn function_a1() {
//!     // This will not run in parallel to function_a2
//! }
//!
//! #[nonparallel(MUT_A)]
//! fn function_a2() {
//!     // This will not run in parallel to function_a1
//! }
//!
//! #[nonparallel(MUT_B)]
//! fn function_b() {
//!     // This may run in parallel to function_a*
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse, parse_macro_input, Ident, Stmt, ItemFn};

#[derive(Debug)]
struct Nonparallel {
    ident: Ident,
}

impl Parse for Nonparallel {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident = input.parse::<Ident>()?;
        Ok(Nonparallel { ident })
    }
}

#[proc_macro_attribute]
pub fn nonparallel(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse macro attributes
    let Nonparallel { ident } = parse_macro_input!(attr);

    // Parse function
    let mut function: ItemFn = parse(item).expect("Could not parse ItemFn");

    // Insert locking code
    let quoted = quote! { let guard = #ident.lock(); };
    let stmt: Stmt = parse(quoted.into()).expect("Could not parse quoted statement");
    function.block.stmts.insert(0, stmt);

    // Generate token stream
    TokenStream::from(function.to_token_stream())
}
