// Copyright (c) 2017 Aler Denisov <aler.zampillo@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![crate_type = "proc-macro"]
#![recursion_limit = "300"]

#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

mod boot;
use boot::*;

#[proc_macro_derive(exonum_service, attributes(record, input, output, id))]
pub fn exonum_service(input: TokenStream) -> TokenStream { 
  let input : String = input.to_string();
  let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
  let result = make_service(&ast);

  result.to_string().parse().expect("Couldn't parse string to tokens")
}

#[proc_macro_derive(exonum_record, attributes(key, get, set, size, service, id, transaction, ty, config))]
pub fn exonum_record(input: TokenStream) -> TokenStream {
  let input : String = input.to_string();
  let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
  let result = make_record(&ast);
  result.to_string().parse().expect("Couldn't parse string to tokens")
}

#[proc_macro_derive(exonum_message, attributes(key, size, service, id, api, ty, config))]
pub fn exonum_message(input: TokenStream) -> TokenStream {
    let input : String = input.to_string();
    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
    let result = make_message(&ast);
    result.to_string().parse().expect("Couldn't parse string to tokens")
}

fn make_record(ast: &syn::MacroInput) -> quote::Tokens { 
  let record_schema = Record::from(ast);
  let record_quote   = record_schema.to_definition();
  let schema_quote   = record_schema.to_schema();
  let record_methods = record_schema.to_methods();

  quote! { 
    #record_quote
    #schema_quote
    #record_methods
  }
}

fn make_message(ast: &syn::MacroInput) -> quote::Tokens { 
  let message_schema = Request::from(ast);
  let message_quote = message_schema.to_definition();

  quote! { 
    #message_quote
  }
}

fn make_service(ast: &syn::MacroInput) -> quote::Tokens {
  let service_schema = Service::from(ast);
  let service_id_macroses = service_schema.to_id_macroses();
  let service_quote = service_schema.to_definition();
  let service_api = service_schema.to_api();

  quote! {
    #service_id_macroses
    #service_quote
    #service_api  
  }
}