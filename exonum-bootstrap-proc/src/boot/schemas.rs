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

use quote;
use syn;

use boot::helpers::*;

#[derive(Clone, Debug)]
pub struct Service {
  pub raw: String,
  pub name: String, 
  pub snake: String,
  pub id: u16,
  pub records: Option<Vec<ServiceLink>>,
  pub inputs: Option<Vec<ServiceLink>>,
  pub outputs: Option<Vec<ServiceLink>>
}

#[derive(Clone, Debug)]
pub struct ServiceLink {
  pub name: String,
  pub ty: String,
  pub snake: String,
  pub id: u16
}

#[derive(Clone, Debug)]
pub struct Record {
    pub name: String,
    pub snake: String,
    pub fields: Option<Vec<Field>>
}

#[derive(Clone, Debug)]
pub struct Request {
    pub name: String,
    pub snake: String,
    pub fields: Option<Vec<Field>>
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub size: u16,
    pub is_key: bool,
    pub is_setter: bool,
    pub modificators: Option<Vec<FieldModificator>>
}

#[derive(Clone, Debug)]
pub enum FieldModificator {
    NameValue(String, String),
    Name(String)
}

// ███████╗███████╗██████╗ ██╗   ██╗██╗ ██████╗███████╗
// ██╔════╝██╔════╝██╔══██╗██║   ██║██║██╔════╝██╔════╝
// ███████╗█████╗  ██████╔╝██║   ██║██║██║     █████╗  
// ╚════██║██╔══╝  ██╔══██╗╚██╗ ██╔╝██║██║     ██╔══╝  
// ███████║███████╗██║  ██║ ╚████╔╝ ██║╚██████╗███████╗
// ╚══════╝╚══════╝╚═╝  ╚═╝  ╚═══╝  ╚═╝ ╚═════╝╚══════╝
                                                    
impl Service {
  pub fn from(ast: &syn::MacroInput) -> Service {
    let raw = (*ast).service_name();
    let name = format!("{}Service", raw);
    let snake = to_snake_case(&name);
    let id = (*ast).service_id();

    let mut next_id = 1u16;
    let records = (*ast).collect_records().iter()
      .map(move |f| ServiceLink::record(f, &mut next_id)).collect::<Vec<ServiceLink>>();

    let mut next_id = 1u16;
    let inputs = (*ast).collect_inputs().iter()
      .map(move |f| ServiceLink::input(f, &mut next_id)).collect::<Vec<ServiceLink>>();

    let mut next_id = 1u16;
    let outputs = (*ast).collect_outputs().iter()
      .map(move |f| ServiceLink::output(f, &mut next_id)).collect::<Vec<ServiceLink>>();

    Service {
      name: name,
      raw: raw,
      snake: snake,
      id: id,
      records: match records.len() { 
        0 => None,
        _ => Some(records)
      },
      inputs: match inputs.len() { 
        0 => None,
        _ => Some(inputs)
      },
      outputs: match outputs.len() { 
        0 => None,
        _ => Some(outputs)
      },
    }
  }

  pub fn to_definition(&self) -> quote::Tokens {
    let name = syn::Ident::from(self.name.clone());
    let api_name = syn::Ident::from(format!("{}Api", self.raw));
    let id = syn::Ident::from_u16(self.id);
    let snake_name = self.snake.clone();
    let boxing_tokens : Vec<quote::Tokens> = match self.inputs {
      Some(ref inputs) => inputs.iter().map(|f| {
        let ty = syn::Ident::from(f.ty.clone());
        let id = syn::Ident::from_u16(f.id);
        
        quote! {
          #id => Box::new(#ty::from_raw(raw)?),
        }
      }).collect(),
      None => Vec::new()
    };

    let input_schema_tokens : Vec<quote::Tokens> = match self.inputs {
      Some(ref inputs) => inputs.iter().map(|f| {
        let ty = syn::Ident::from(f.ty.clone());        
        quote! { #ty::get_schema() }
      }).collect(),
      None => Vec::new()
    };

    quote! {
      #[derive(Default)]
      struct #name;

      impl #name {
        pub fn new() -> #name {
          #name {}
        }
      }

      impl Service for #name {
        fn service_name(&self) -> &'static str { #snake_name }

        fn service_id(&self) -> u16 { #id }

        fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
            let trans: Box<Transaction> = match raw.message_type() {
                #(
                    boxing_tokens
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
              blockchain: ctx.blockchain().clone()
          };
          api.wire(&mut router);
          Some(Box::new(router))
        }
      }
      
      impl ServiceFactory for #name {
        fn make_service(_: &Context) -> Box<Service> {
          Box::new(#name::new())
        }
      }

      impl SchemaServiceProvider for #name {
        fn get_schema() -> SchemaService {
          SchemaService {
            name: stringify!(#name).to_string(),
            id: #id,
            records: None,
            inputs: Some(vec![
              #(input_schema_tokens),*
            ]),
            outputs: None
          }
        }
      }
    }
  }

  pub fn to_api(&self) -> quote::Tokens {
    let name = syn::Ident::from(self.name.clone());
    let api_name = syn::Ident::from(format!("{}Api", self.raw));
    let inputs_request_tokens: Vec<quote::Tokens> = match self.inputs {
      Some(ref inputs) => inputs.iter().map(|f| {
        let tx_name = syn::Ident::from(f.snake.clone());
        let tx_ty   = syn::Ident::from(f.ty.clone());
        quote! { #tx_name ( #tx_ty ) }
      }).collect(),
      None => Vec::new()
    };

    let inputs_boxing_tokens: Vec<quote::Tokens> = match self.inputs {
      Some(ref inputs) => inputs.iter().map(|f| {
        let tx_name = syn::Ident::from(f.snake.clone());
        quote! { TransactionRequest::#tx_name(trans) => Box::new(trans) }
      }).collect(),
      None => Vec::new()
    };

    
    let api_url = "/v1";
    let api_url_transactions = format!("{}/tx", api_url);
    let api_url_schema = format!("{}/schema", api_url);

    quote! {
      #[derive(Clone)]
      struct #api_name {
        channel: ApiSender<NodeChannel>,
        blockchain: Blockchain
      }

      impl #api_name {
        fn get_tx(&self, tx_hash: &Hash) -> Option<TxInfo> {
            let explorer = Explorer::new(&self.blockchain);
            explorer.tx_info(tx_hash).unwrap()
        }
      }

      impl Api for #api_name {
        fn wire(&self, router: &mut Router) {
          #[serde(untagged)]
          #[derive(Clone, Serialize, Deserialize)]
          enum TransactionRequest {
            #(inputs_request_tokens),*                    
          }

          impl Into<Box<Transaction>> for TransactionRequest {
            fn into(self) -> Box<Transaction> {
              match self {
                #(inputs_boxing_tokens),*
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

          let self_ = self.clone();
          let schema_handler = move |req: &mut Request| -> IronResult<Response> {
            let schema = #name::get_schema();
            self_.ok_response(&serde_json::to_value(schema).unwrap())
          };

          // let self_ = self.clone();
          // let trx_explorer = move |req: &mut Request| -> IronResult<Response> {
          //   let tx_hash = req.extensions.get::<Router>().unwrap().find("hash").unwrap_or("/").to_string();
          //   let tx_hash = Hash::from_hex(tx_hash).unwrap();
          //   let explorer = Explorer::new(&self_.blockchain);
            
          //   let result = self_.get_tx(&tx_hash).unwrap();
          //   self_.ok_response(&serde_json::to_value(result).unwrap())
          // };

          // let self_ = self.clone();
          // let schema_handler = move |red: &mut Request| -> IronResult<Response> {
          //   self_.ok_response(&serde_json::to_value(&self_.schema).unwrap())
          // };

          let route_post = #api_url_transactions;
          router.post(&route_post, transaction, "transaction");

          let schema_route = "/v1/schema";
          router.get(&schema_route, schema_handler, "schema");
        }
      }
    }
  }

  pub fn to_id_macroses(&self) -> quote::Tokens {
    let id = syn::Ident::from_u16(self.id);
    let records_service_ids : Vec<quote::Tokens> = match self.records {
      Some(ref r) => r.iter().map(|f| {
        let macros_name = syn::Ident::from(format!("{}_service_id", f.snake));
        quote! {
          macro_rules! #macros_name {
            () => { #id }
          }
        }
      }).collect(),
      None => Vec::new()
    };

    let records_ids : Vec<quote::Tokens> = match self.records {
      Some(ref r) => r.iter().map(|f| {
        let macros_name = syn::Ident::from(format!("{}_id", f.snake));
        let record_id = syn::Ident::from_u16(f.id);
        quote! {
          macro_rules! #macros_name {
            () => { #record_id }
          }
        }
      }).collect(),
      None => Vec::new()
    };

    let inputs_service_ids : Vec<quote::Tokens> = match self.inputs {
      Some(ref r) => r.iter().map(|f| {
        let macros_name = syn::Ident::from(format!("{}_service_id", f.snake));
        quote! {
          macro_rules! #macros_name {
            () => { #id }
          }
        }
      }).collect(),
      None => Vec::new()
    };

    let inputs_ids : Vec<quote::Tokens> = match self.inputs {
      Some(ref r) => r.iter().map(|f| {
        let macros_name = syn::Ident::from(format!("{}_id", f.snake));
        let record_id = syn::Ident::from_u16(f.id);
        quote! {
          macro_rules! #macros_name {
            () => { #record_id }
          }
        }
      }).collect(),
      None => Vec::new()
    };

    quote! {
      #(
        records_service_ids
      )*

      #(
        inputs_service_ids
      )*

      #(
        records_ids
      )*

      #(
        inputs_ids
      )*
    }
  }
}

// ██████╗ ███████╗ ██████╗ ██████╗ ██████╗ ██████╗ 
// ██╔══██╗██╔════╝██╔════╝██╔═══██╗██╔══██╗██╔══██╗
// ██████╔╝█████╗  ██║     ██║   ██║██████╔╝██║  ██║
// ██╔══██╗██╔══╝  ██║     ██║   ██║██╔══██╗██║  ██║
// ██║  ██║███████╗╚██████╗╚██████╔╝██║  ██║██████╔╝
// ╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═════╝ 
                                                 
impl Record {
  pub fn from(ast: &syn::MacroInput) -> Record {
    let name = (*ast).record_name();
    let snake = to_snake_case(&name);
    let fields = (*ast).collect_fields();
    let fields = fields.iter().map(|field| Field::from(&field));
    
    let fields = match fields.len() {
      0 => None,
      _ => Some(fields.collect::<Vec<Field>>())
    };

    Record {
        name: name,
        snake: snake,
        fields: fields
    }
  }

  pub fn to_definition(&self) -> quote::Tokens {
    let name = syn::Ident::from(self.name.clone());
    let total = self.fields.clone().unwrap().iter().map(|field| field.size).sum();
    let total = syn::Ident::from_u16(total);

    let snake = to_snake_case(&self.name);

    let fields : Vec<quote::Tokens> = self.fields_tokens();

    quote! {
      encoding_struct! {
        struct #name {
          const SIZE = #total;

          #(
              fields
          )*
        }
      }
    }
  }

  pub fn to_schema(&self) -> quote::Tokens {
    let schema_name = syn::Ident::from(self.name.clone() + "Schema");
    let record_name = syn::Ident::from(self.name.clone());

    let key_field = self.get_key();

    let key_ty = syn::Ident::from(key_field.ty.clone());
    let key_ty_schema = syn::Ident::from(key_field.ty.clone().replace("&", ""));

    let id = syn::Ident::from(format!("{}_id!()", self.snake));

    let service_id = self.service_id();
    

    quote! {
      pub struct #schema_name <'schema> {
        view: &'schema mut Fork,
      }

      impl <'schema>  #schema_name <'schema>  {
        pub fn index(&mut self) -> MapIndex<&mut Fork, #key_ty_schema, #record_name> {
          let prefix = blockchain::gen_prefix(#service_id, #id, &());
          MapIndex::new(prefix, self.view)
        }        

        pub fn get(&mut self, key: #key_ty) -> Option<#record_name> {
          self.index().get(key)
        }
      }
    }
  }

  pub fn to_methods(&self) -> quote::Tokens {
    let record_name = syn::Ident::from(self.name.clone());
    let setter_tokens = self.fields_setters();
    
    quote! {
      impl #record_name {
        #(
          setter_tokens
        )*
      }
    }
  }
  
  pub fn get_key(&self) -> Field {
    let fields = self.fields.clone().unwrap();

    let field = fields.iter()
      .find(|ref f| f.has_attr("key"))
      .unwrap();
    
    field.clone()
  }

  pub fn fields_setters(&self) -> Vec<quote::Tokens> {
    // let mut result : Vec<quote::Tokens> = Vec::new();
    let mut next = 0u16;

    match self.fields {
      None => Vec::new(),
      Some(ref fields) => {
        fields.iter()
        .map(move |ref f| {
          let start = syn::Ident::from_u16(next);
          let end   = next + f.size;
          next = end;
          let end = syn::Ident::from_u16(end);

          let ty = syn::Ident::from(f.ty.clone());
          let snake = to_snake_case(&f.name);
          let setter_name = syn::Ident::from("set_".to_string() + &snake);

          (start, end, ty, setter_name, f.has_attr("setter"))
        })
        .filter(|r| r.4)
        .map(|r| {
          let start = r.0.clone();
          let end = r.1.clone();
          let ty = r.2.clone();
          let setter_name = r.3.clone();

          quote! {
            pub fn #setter_name(&mut self, value : #ty) {
                Field::write(&value, &mut self.raw, #start, #end);
            }
          }
        })
        .collect()
      },
    }
  }

  pub fn fields_tokens(&self) -> Vec<quote::Tokens> {
    let mut next : u16 = 0;

    self.fields.clone().unwrap().iter().map(|field| {
      let name  = syn::Ident::from(field.name.clone());
      let ty    = syn::Ident::from(field.ty.clone());
      let start = syn::Ident::from_u16(next);
      let end   = next + field.size;
      
      next = end;
      let end = syn::Ident::from_u16(end);
      quote! {
        field #name: #ty [#start => #end]
      }
    }).collect()
  }

  pub fn service_id(&self) -> quote::Tokens {
    let request = format!("{}_service_id!()", to_snake_case(&self.name));
    let request = syn::Ident::from(request);
    quote! {
      #request
    }
  }
}

// ██████╗ ███████╗ ██████╗ ██╗   ██╗███████╗███████╗████████╗
// ██╔══██╗██╔════╝██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
// ██████╔╝█████╗  ██║   ██║██║   ██║█████╗  ███████╗   ██║   
// ██╔══██╗██╔══╝  ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║   
// ██║  ██║███████╗╚██████╔╝╚██████╔╝███████╗███████║   ██║   
// ╚═╝  ╚═╝╚══════╝ ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝   
                                                           

impl Request {
  pub fn from(ast: &syn::MacroInput) -> Request {
    let name = (*ast).message_name();
    let snake = to_snake_case(&name);
    let fields = (*ast).collect_fields();
    let fields = fields.iter().map(|field| Field::from(&field));
    
    let fields = match fields.len() {
      0 => None,
      _ => Some(fields.collect::<Vec<Field>>())
    };

    Request {
        name: name,
        snake: snake,
        fields: fields
    }
  }

  pub fn to_definition(&self) -> quote::Tokens {
    let name = syn::Ident::from(self.name.clone());
    let snake = to_snake_case(&self.name);
    let total = self.fields.clone().unwrap().iter().map(|field| field.size).sum();
    let total = syn::Ident::from_u16(total);
    let fields : Vec<quote::Tokens> = self.fields_tokens();
    let service_id = self.service_id();

    let id = format!("{}_id!()", &snake);
    let id = syn::Ident::from(id);

    let key_name = match self.fields {
      Some(ref fields) => fields.iter().find(|f| f.is_key).map(|f| syn::Ident::from(f.name.clone())).unwrap(),
      None => panic!("Record should have a PublicKey field")
    };

    let mut next : u16 = 0;

    let field_schema_tokens : Vec<quote::Tokens> = match self.fields {
      Some(ref fields) => fields.iter().map(|field| {
        let name  = syn::Ident::from(field.name.clone());
        let ty    = syn::Ident::from(field.ty.clone());
        let start = syn::Ident::from_u16(next);
        let end   = next + field.size;
        
        next = end;
        let end = syn::Ident::from_u16(end);
        let size = syn::Ident::from_u16(field.size);

        let mod_tokens : Option<Vec<quote::Tokens>> = match field.modificators {
          Some(ref mods) => Some(mods.iter().map(|m| match *m {
            FieldModificator::Name(ref n) => quote! { FieldModificator { name: #n.to_string(), value: None } },
            FieldModificator::NameValue(ref n, ref v) => quote! { FieldModificator { name: #n.to_string(), value: Some(#v.to_string()) } },
          }).collect()),
          None => None,
        };

        let mod_tokens : quote::Tokens = match mod_tokens {
          Some(ref tokens) => quote! { Some(vec![#(tokens),*]) },
          None => quote! (None)
        };

        quote! {
          SchemaField {
              name: stringify!(#name).to_string(),
              ty: stringify!(#ty).to_string(),
              size: #size,
              from: #start,
              to: #end,
              modificators: #mod_tokens
          }
        }
      }).collect(),
      None => panic!("Record should have a PublicKey field")
    };

    quote! {
      message! {
        struct #name {
          const TYPE = #service_id;
          const ID = #id;
          const SIZE = #total;

          #(
            fields
          )*
        }
      }

      impl Transaction for #name {
        fn verify(&self) -> bool {
            self.verify_signature(self.#key_name()) && TransactionMethods::verify(self) 
        }

        fn execute(&self, view : &mut Fork) {
            TransactionMethods::execute(self, view);
        }
      }

      impl SchemaRequestProvider for #name {
        fn get_schema() -> SchemaRequest {
          SchemaRequest {
            name: stringify!(#name).to_string(),
            id: #id,
            size: #total,
            request: Some(vec![
              #( field_schema_tokens ),*
            ]),
            response: None,
          }
        }
      }
    }
  }

  pub fn fields_tokens(&self) -> Vec<quote::Tokens> {
    let mut next : u16 = 0;

    self.fields.clone().unwrap().iter().map(|field| {
      let name  = syn::Ident::from(field.name.clone());
      let ty    = syn::Ident::from(field.ty.clone());
      let start = syn::Ident::from_u16(next);
      let end   = next + field.size;
      
      next = end;
      let end = syn::Ident::from_u16(end);
      quote! {
        field #name: #ty [#start => #end]
      }
    }).collect()
  }

  pub fn service_id(&self) -> quote::Tokens {
    let request = format!("{}_service_id!()", to_snake_case(&self.name));
    let request = syn::Ident::from(request);
    quote! {
      #request
    }
  }
}

// ███████╗██╗███████╗██╗     ██████╗ 
// ██╔════╝██║██╔════╝██║     ██╔══██╗
// █████╗  ██║█████╗  ██║     ██║  ██║
// ██╔══╝  ██║██╔══╝  ██║     ██║  ██║
// ██║     ██║███████╗███████╗██████╔╝
// ╚═╝     ╚═╝╚══════╝╚══════╝╚═════╝ 
                                   
impl ServiceLink {
  pub fn record(field: &syn::Field, next_id: &mut u16) -> ServiceLink {
    let name = (*field).get_name();
    let ty = (*field).get_type().replace("__", "");
    let id = match (*field).get_attr_value_safe("record") {
      Ok(ref value) => value.parse::<u16>().unwrap(),
      Err(_) => next_id.clone()
    };

    *next_id = id + 1;

    ServiceLink {
      name: name,
      snake: to_snake_case(&ty),
      ty: ty,
      id: id
    }
  }

  pub fn input(field: &syn::Field, next_id: &mut u16) -> ServiceLink {
    let name = (*field).get_name();
    let ty = (*field).get_type().replace("__", "Tx");
    let id = match (*field).get_attr_value_safe("input") {
      Ok(ref value) => value.parse::<u16>().unwrap(),
      Err(_) => next_id.clone()
    };

    *next_id = id + 1;

    ServiceLink {
      name: name,
      snake: to_snake_case(&ty),
      ty: ty,
      id: id
    }
  }

  pub fn output(field: &syn::Field, next_id: &mut u16) -> ServiceLink {
    let name = (*field).get_name();
    let ty = (*field).get_type().replace("__", "Get");
    let id = match (*field).get_attr_value_safe("output") {
      Ok(ref value) => value.parse::<u16>().unwrap(),
      Err(_) => next_id.clone()
    };

    *next_id = id + 1;

    ServiceLink {
      name: name,
      snake: to_snake_case(&ty),
      ty: ty,
      id: id
    }
  }
}

impl Field {
  pub fn from(field: &syn::Field) -> Field {
    let name = (*field).get_name();
    let is_key = (*field).is_key();
    let has_setter = (*field).has_setter();
    let ty = (*field).get_type();
    let size = (*field).get_size();

    let mods = (*field).collect_mods();
    
    let mut mods : Vec<FieldModificator> = match mods { 
      None => Vec::new(),
      Some(mods) => (*mods).iter().map(|attr| FieldModificator::from(attr)).collect()
    };

    if is_key || has_setter {
      if is_key {
        mods.push(FieldModificator::Name("key".to_string()))
      }

      if has_setter {
        mods.push(FieldModificator::Name("setter".to_string()))
      }
    };

    let mods = match mods.len() {
      0 => None,
      _ => Some(mods)
    };

    Field {
      name: name,
      ty: ty,
      is_key: is_key,
      is_setter: has_setter,
      size: size,
      modificators: mods // TODO: collect config attrs
    }
  }

  pub fn has_attr(&self, name : &str) -> bool {
    if self.modificators.is_none() {
      return false
    }

    match self.modificators {
      None => false,
      Some(ref mods) => mods.iter().any(|ref m| m.is_name(name))
    }
  }
}

impl FieldModificator {
  pub fn from(meta : &syn::MetaItem) -> FieldModificator {
    match *meta {
      syn::MetaItem::NameValue(ref name, syn::Lit::Str(ref value, _)) => FieldModificator::NameValue(
        name.to_string(),
        value.to_string()
      ),
      syn::MetaItem::Word(ref name) => FieldModificator::Name(name.to_string()),
      _ => panic!("Should be word or name value")
    }
  }

  pub fn is_name(&self, key : &str) -> bool {
    match *self {
      FieldModificator::NameValue(ref name, _) if name == key => true,
      FieldModificator::Name(ref name) if name == key => true,
      _ => false
    }
  }
}