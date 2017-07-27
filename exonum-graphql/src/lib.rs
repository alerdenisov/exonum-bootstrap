#![crate_type = "proc-macro"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


#[proc_macro_service(data)]
pub fn data(input: TokenStream) -> TokenStream {
    
}