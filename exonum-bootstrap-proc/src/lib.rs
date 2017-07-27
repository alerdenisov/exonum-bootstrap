#![feature(box_syntax, test, fmt_internals)]
#![crate_type = "proc-macro"]
#![recursion_limit = "200"]

#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;

#[proc_macro_derive(exonum_record, attributes(key, get, set, size, service, id, transaction, ty))]
pub fn exonum_record(input: TokenStream) -> TokenStream {
    let input : String = input.to_string();
    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
    let result = match ast.body {
        syn::Body::Struct(ref variant) => make_structure(&ast, &variant),
        _ => unimplemented!(),
    };

    result.to_string().parse().expect("Couldn't parse string to tokens")
}

#[proc_macro_derive(exonum_message, attributes(key, size, service, id, api, ty))]
pub fn exonum_message(input: TokenStream) -> TokenStream {
    let input : String = input.to_string();
    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
    let result = match ast.body {
        syn::Body::Struct(ref variant) => make_message(&ast, &variant),
        _ => unimplemented!(),
    };

    result.to_string().parse().expect("Couldn't parse string to tokens")
}

fn make_structure(ast: &syn::MacroInput, variant: &syn::VariantData) -> quote::Tokens {
    let struct_name = record_name(&ast);
    let schema_name  = syn::Ident::new(struct_name.to_string() + &"Schema".to_string());
    let service_name = syn::Ident::new(struct_name.to_string() + &"Service".to_string());
    let api_name = syn::Ident::new(struct_name.to_string() + &"Api".to_string());
    let api_transaction_name = syn::Ident::new(struct_name.to_string() + &"ApiTransactionRequest".to_string());
    let snake_name = to_snake_case(&struct_name.to_string());

    let api_url = format!("/v1/{}", snake_name);
    let api_url_transactions = format!("{}/tx", api_url);



    let (
        service_id,
        record_id
    ) = parse_attributes(&ast);

    let (
        total, // syn::Ident, //total,
        names, //Vec<syn::Ident>, //names,
        types, //Vec<syn::Ident>, //types,
        sizes, //Vec<(syn::Ident, syn::Ident, syn::Ident)>, //sizes
        setters, //Vec<bool>, //setters
        key_name, //syn::Ident, //key_name,
        key_ty, //syn::Ident, //key_ty
    ) = parse_fields(&variant);

    let key_ty_schema = syn::Ident::from(key_ty.to_string().replace("&", ""));

    let mut encoding_fields : Vec<quote::Tokens> = Vec::new();
    let mut setter_tokens : Vec<quote::Tokens> = Vec::new();
    let mut getter_tokens : Vec<quote::Tokens> = Vec::new();

    for index in 0..(names.len()) {
        let name = names[index].clone();
        let ty = types[index].clone();
        let (_, start, end) = sizes[index].clone();
        let encoded = quote! {
            field #name: #ty [#start => #end]
        };

        encoding_fields.push(encoded);

        if setters[index] {
            let setter_name = syn::Ident::from("set_".to_string() + &name.to_string());
            let setter = quote! {
                pub fn #setter_name(&mut self, value : #ty) {
                    Field::write(&value, &mut self.raw, #start, #end);
                }
            };
            setter_tokens.push(setter);
        }
    }

    let fields : Vec<&syn::Field> = match *variant {
        syn::VariantData::Struct(ref fields) => fields,
        _ => unimplemented!()
    }.iter().filter(|f| field_has_attribute(&f, "transaction")).collect();

    let transaction_names : Vec<syn::Ident> = fields.iter()
        .map(|ref field| match field.ident {
            Some(ref ident) => ident.clone(),
            _ => unimplemented!()
        })
        .collect();

    let transaction_types : Vec<syn::Ident> = fields.iter()
        .map(|ref field| field_type_string(field))
        .map(|ref token| syn::Ident::from(token.replace("&", "").clone()))
        .collect();

    let transaction_ids : Vec<syn::Ident> = fields.iter()
        .map(|ref field| field_get_attribute_value(field, "id"))
        .collect();
        
    let mut transaction_request_tokens : Vec<quote::Tokens> = Vec::new();
    let mut transaction_boxing_tokens : Vec<quote::Tokens> = Vec::new();
    let mut transaction_unboxing_tokens : Vec<quote::Tokens> = Vec::new();

    for index in 0..(transaction_names.len()) {
        let name = transaction_names[index].clone();
        let ty = transaction_types[index].clone();
        let id = transaction_ids[index].clone();

        transaction_request_tokens.push(quote! {
            #name ( #ty ),
        });

        transaction_unboxing_tokens.push(quote! {
            #id => Box::new(#ty::from_raw(raw)?),
        });

        transaction_boxing_tokens.push(quote! {
            TransactionRequest::#name(trans) => Box::new(trans),
        })
    }


    quote! {
        encoding_struct! {
            struct #struct_name {
                const SIZE = #total;
                #(
                    encoding_fields
                )*
            }
        }

        // Struct methods
        impl #struct_name {
            #(
                setter_tokens
            )*
        }

        pub struct #schema_name <'schema> {
            view: &'schema mut Fork,
        }

        impl <'schema>  #schema_name <'schema>  {
            pub fn index(&mut self) -> MapIndex<&mut Fork, #key_ty_schema, #struct_name> {
                let prefix = blockchain::gen_prefix(#service_id, #record_id, &());
                MapIndex::new(prefix, self.view)
            }

            pub fn get(&mut self, key: #key_ty) -> Option<#struct_name> {
                self.index().get(key)
            }
        }

        struct #api_name {
            channel: ApiSender<NodeChannel>,
        }

        impl Api for #api_name {
            fn wire(&self, router: &mut Router) {
                #[serde(untagged)]
                #[derive(Clone, Serialize, Deserialize)]
                enum TransactionRequest {
                    #(
                        transaction_request_tokens
                    )*                    
                }

                impl Into<Box<Transaction>> for TransactionRequest {
                    fn into(self) -> Box<Transaction> {
                        match self {
                            #(
                                transaction_boxing_tokens
                            )*
                        }
                    }
                }

                #[derive(Serialize, Deserialize)]
                struct TransactionResponse {
                    tx_hash: Hash,
                }

                let self_ = self.clone();
                let transaction = move |req: &mut Request| -> IronResult<Response> {
                    match req.get::<bodyparser::Struct<TransactionRequest>>() {
                        Ok(Some(transaction)) => {
                            let transaction: Box<Transaction> = transaction.into();
                            let tx_hash = transaction.hash();
                            self_.channel.send(transaction).map_err(|e| ApiError::Events(e))?;
                            let json = TransactionResponse { tx_hash };
                            self_.ok_response(&serde_json::to_value(&json).unwrap())
                        }
                        Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
                        Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
                    }
                };

                let route_post = #api_url_transactions;
                router.post(&route_post, transaction, "transaction");
            }
        }

        struct #service_name;

        impl Service for #service_name {
            fn service_name(&self) -> &'static str { #snake_name }

            fn service_id(&self) -> u16 { #service_id }

            fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
                let trans: Box<Transaction> = match raw.message_type() {
                    #(
                        transaction_unboxing_tokens
                    )*
                    _ => {
                        return Err(encoding::Error::IncorrectMessageType { message_type: raw.message_type() });
                    },
                };
                Ok(trans)
            }

            fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
                let mut router = Router::new();
                let api = #api_name {
                    channel: ctx.node_channel().clone(),
                };
                api.wire(&mut router);
                Some(Box::new(router))
            }
        }
    }
}

fn make_message(ast: &syn::MacroInput, variant: &syn::VariantData) -> quote::Tokens {

    let struct_name = message_name(&ast);

    let fields = match *variant {
        syn::VariantData::Struct(ref fields) => fields,
        _ => unimplemented!()
    };

    let (
        service_id,
        message_id
    ) = parse_attributes(&ast);

    let (
        total, // syn::Ident, //total,
        names, //Vec<syn::Ident>, //names,
        types, //Vec<syn::Ident>, //types,
        sizes, //Vec<(syn::Ident, syn::Ident, syn::Ident)>, //sizes
        setters, //Vec<bool>, //setters
        key_name, //syn::Ident, //key_name,
        key_ty, //syn::Ident, //key_ty
    ) = parse_fields(&variant);

    let mut encoding_fields : Vec<quote::Tokens> = Vec::new();

    for index in 0..(names.len()) {
        let name = names[index].clone();
        let ty = types[index].clone();
        let (_, start, end) = sizes[index].clone();

        encoding_fields.push(quote! {
            field #name: #ty [#start => #end]
        });
    }

    quote! {
        message! {
            struct #struct_name {
                const TYPE = #service_id;
                const ID = #message_id;
                const SIZE = #total;

                #(
                    encoding_fields
                )*
            }
        }

        impl Transaction for #struct_name {
            fn verify(&self) -> bool {
                self.verify_signature(self.#key_name()) && TransactionVerify::verify(self) 
            }

            fn execute(&self, view : &mut Fork) {
                TransactionExecute::execute(self, view);
            }
        }
    }
}


fn parse_fields(variant: &syn::VariantData) -> (
    syn::Ident, //total,
    Vec<syn::Ident>, //names,
    Vec<syn::Ident>, //types,
    Vec<(syn::Ident, syn::Ident, syn::Ident)>, //sizes
    Vec<bool>, //setters
    syn::Ident, //key_name,
    syn::Ident, //key_ty
) {
    let fields : Vec<&syn::Field> = match *variant {
        syn::VariantData::Struct(ref fields) => fields,
        _ => unimplemented!()
    }.iter().filter(|f| !field_has_attribute(&f, "transaction")).collect();

    let names : Vec<syn::Ident> = fields.iter()
        .map(|ref field| match field.ident {
            Some(ref ident) => ident.clone(),
            _ => unimplemented!()
        })
        .collect();
    

    let key = fields.iter()
        .find(|field| field_has_attribute(&field, "key"))
        .unwrap();

    let key_name : syn::Ident = match key.ident {
        Some(ref ident) => ident.clone(),
        _ => unimplemented!()
    };

    let key_ty : syn::Ident = syn::Ident::from(field_type_string(key));

    let types : Vec<syn::Ident> = fields.iter()
        .map(|ref field| field_type_string(field))
        .map(|ref token| syn::Ident::from(token.clone()))
        .collect();

    let mut total = 0;
    
    let sizes : Vec<(syn::Ident, syn::Ident, syn::Ident)> = fields
        .iter()
        .map(|field| field_size_tuple(field, &mut total)).collect();

    let setters : Vec<bool> = fields.iter()
        .map(|field| field_has_attribute(&field, "set"))
        .collect();

    let total = syn::Ident::from(total.to_string().replace("i32", ""));

    (
        total,
        names,
        types,
        sizes,
        setters,
        key_name,
        key_ty
    )
}

fn parse_attributes(ast: &syn::MacroInput) -> (
    syn::Ident, // service id
    syn::Ident  // struct id (record or message)
) {
    (
        ast.attrs.iter()
            .find(|attr| match attr.value {
                syn::MetaItem::NameValue(ref ident, _) if ident.to_string() == "service" => true,
                _ => false
            })
            .map(|attr| match attr.value {
                syn::MetaItem::NameValue(_, syn::Lit::Str(ref raw, _)) => syn::Ident::from(raw.clone()),
                _ => unimplemented!()
            }).unwrap(),
        ast.attrs.iter()
            .find(|attr| match attr.value {
                syn::MetaItem::NameValue(ref ident, _) if ident.to_string() == "id" => true,
                _ => false
            })
            .map(|attr| match attr.value {
                syn::MetaItem::NameValue(_, syn::Lit::Str(ref raw, _)) => syn::Ident::from(raw.clone()),
                _ => unimplemented!()
            }).unwrap()
    )
}

fn message_name(ast: &syn::MacroInput) -> syn::Ident {
    let name = ast.ident.to_string().replace("__", "Tx");
    return syn::Ident::from(name);
}

fn field_has_attribute(field: &syn::Field, name : &str) -> bool {
    (*field).attrs.iter().any(|attr| is_attribute(&attr, &name))
}

fn is_attribute(attr: &syn::Attribute, name : &str) -> bool {
    match attr.value {
        syn::MetaItem::NameValue(ref ident, _) if ident.to_string() == name => true,
        syn::MetaItem::Word(ref ident) if ident.to_string() == name => true,
        _ => false
    }
}

fn field_get_attribute_value(field: &syn::Field, name : &str) -> syn::Ident {
    field_get_attribute_value_or_none(field, name).unwrap()
}

fn field_get_attribute_value_or_none(field: &syn::Field, name : &str) -> Option<syn::Ident> {
    (*field).attrs.iter()
        .find(|attr| is_attribute(&attr, &name))
        .map(|attr| get_attribute_value(&attr))
}

fn get_attribute_value(attr: &syn::Attribute) -> syn::Ident {
    match attr.value {
        syn::MetaItem::NameValue(_, syn::Lit::Str(ref raw, _)) => syn::Ident::from(raw.clone()),
        _ => unimplemented!()
    }
}

fn field_type_string(field: &syn::Field) -> String {
    match field_get_attribute_value_or_none(field, "ty") {
        Some(ref ty) => ty.to_string(),
        None => match *field {
            syn::Field { ident: _, vis: _, attrs: _, ty: ref ty } => match *ty {
                syn::Ty::Rptr(_, ref mubox) => {
                    match *mubox.as_ref() { 
                        syn::MutTy { ty: ref ty, mutability: _} => {
                            let t = match *ty {
                                syn::Ty::Path(_, syn::Path { global: _, segments: ref segments}) => segments.as_slice().last().unwrap().ident.to_string(),
                                _ => unimplemented!()
                            };

                            "&".to_string() + &t
                        },
                        _ => unimplemented!()
                    }
                },
                syn::Ty::Path(_, syn::Path { global: _, segments: ref segments }) => segments.as_slice().last().unwrap().ident.to_string(),
                _ => unimplemented!()
            },
            _ => unimplemented!()
        }
    }
}

fn field_size_tuple(field: &syn::Field, total: &mut i32) -> (syn::Ident, syn::Ident, syn::Ident) {
    let size_attr = (*field).attrs
        .iter()
        .map(|attr| &attr.value)
        .find(|val| match *val {
            &syn::MetaItem::NameValue(ref ident, _) if ident.to_string() == "size" => true,
            _ => false
        })
        .map(|a| match *a {
            syn::MetaItem::NameValue(_, ref lit) => lit.clone(),
            _ => unimplemented!()
        })
        .map(|lit| {
            match lit {
                syn::Lit::Str(ref raw, _) => raw.clone(),
                _ => unimplemented!()
            }
        }).unwrap();

    let size = size_attr.parse::<i32>().unwrap();
    let start = *total;
    let end = start + size;

    *total = end;

    (
        syn::Ident::from(size_attr),
        syn::Ident::from(start.to_string().replace("i32", "")),
        syn::Ident::from(end.to_string().replace("i32", ""))
    )
}

fn record_name(ast: &syn::MacroInput) -> syn::Ident {
    let name = ast.ident.to_string().replace("__", "");
    return syn::Ident::from(name);
    // let token = quote! { #name Record };
    // let result = token.to_string().replace(" ", "");
    // syn::Ident::from(result)
}

fn to_snake_case(s: &str) -> String {
    let (ch, next, mut acc): (Option<char>, Option<char>, String) =
        s.chars().fold((None, None, String::new()), |(prev, ch, mut acc), next| {
            if let Some(ch) = ch {
                if let Some(prev) = prev {
                    if ch.is_uppercase() {
                        if prev.is_lowercase() || prev.is_numeric() ||
                            (prev.is_uppercase() && next.is_lowercase())
                        {
                            acc.push('_');
                        }
                    }
                }
                acc.extend(ch.to_lowercase());
            }
            (ch, Some(next), acc)
        });
    if let Some(next) = next {
        if let Some(ch) = ch {
            if (ch.is_lowercase() || ch.is_numeric()) && next.is_uppercase() {
                acc.push('_');
            }
        }
        acc.extend(next.to_lowercase());
    }
    acc
}