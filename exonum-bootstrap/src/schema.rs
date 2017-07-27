#[derive(Clone, Serialize)]
pub struct SchemaService {
    pub name: String,
    pub id: u16,
    pub records: Option<Vec<SchemaRecord>>
}

#[derive(Clone, Serialize)]
pub struct SchemaRecord {
    pub name: String,
    pub url: String,
    pub id: u16,
    pub fields: Option<Vec<SchemaField>>,
    pub inputs: Option<Vec<SchemaRequest>>,
    pub outputs: Option<Vec<SchemaRequest>>
}

#[derive(Clone, Serialize)]
pub struct SchemaRequest {
    pub name: String,
    pub id: u16,
    pub size: u8,
    pub request: Option<Vec<SchemaField>>,
    pub response: Option<Vec<SchemaField>>,
}

#[derive(Clone, Serialize)]
pub struct SchemaField {
    pub name: String,
    pub ty: String,
    pub size: u8,
    pub modificators: Option<Vec<FieldModificator>>
}

#[derive(Clone, Serialize)]
pub struct FieldModificator {
    pub name: String,
    pub value: String
}

pub trait SchemaRecordProvider { fn get_schema() -> Option<SchemaRecord>; }
pub trait SchemaServiceProvider { fn get_schema() -> Option<SchemaService>; }
pub trait SchemaRequestProvider { fn get_schema() -> Option<SchemaRequest>; }
pub trait SchemaFieldProvider { fn get_schema() -> Option<SchemaField>; }

// impl SchemaRecordProvider for syn::MacroInput {
//     fn get_schema(&self) -> Option<SchemaRecord> {
//         Some(SchemaRecord {
//             name: self.ident.to_string().replace("__", ""),
//             url: format!("http://localhost:8000/api/services/{}/v1/schema", to_snake_case(self.ident.to_string()),
//             id: get_id(self),
//             fields: self.fields.iter().map(|field| field.get_schema).collect::Vec<SchemaField>::(),
//             inputs: None,
//             outputs: None,
//         })
//     }
// }

// impl SchemaFieldProvider for syn::Field {
//     fn get_schema(&self) -> Option<SchemaField> {
//         Some(
//             SchemaField {
//                 name: self.ident.to_string(),
//                 ty: field_type_string(self),
//                 size: field_size(self),
//                 modificators: field_modificators(self)
//             }
//         )
//     }
// }

// impl SchemaModificatorProvider for syn::MetaItem {
//     fn get_schema(&self) -> Option<SchemaModificator> {
//         match self {
//             syn:NameValue(ref ident, syn::Lit::Str(ref value, _)) => Some( SchemaModificator {
//                 name: ident.to_string(),
//                 value: value.to_string()
//             }),
//             Word(ref ident) => Some( SchemaModificator {
//                 name: ident.to_string(),
//                 value: ident.to_string()
//             }),
//             _ => None
//         }
//     }
// }

// fn field_modificators(field: &syn::Field) -> Option<Vec<SchemaModificator>> {
//     let mut modificators = Vec<SchemaModificator>::new();

//     let configs : Vec<syn::List> = field.attrs.iter().filter(|attr| match attr {
//         syn:List(ref ident, ref nested) if ident.to_string() == "config" => true,
//         _ => false
//     }).collect();

//     for conf in configs {
//         match conf {
//             syn:List(_, ref nested) => {
//                 for n in nested {
//                     modificators.push(n.get_schema());
//                 }
//             },
//             _ => {}
//         };
//     }

//     return modificators;
// }

// fn field_type_string(field: &syn::Field) -> String {
//     match field_get_attribute_value_or_none(field, "ty") {
//         Some(ref ty) => ty.to_string(),
//         None => match *field {
//             syn::Field { ident: _, vis: _, attrs: _, ty: ref ty } => match *ty {
//                 syn::Ty::Rptr(_, ref mubox) => {
//                     match *mubox.as_ref() { 
//                         syn::MutTy { ty: ref ty, mutability: _} => {
//                             let t = match *ty {
//                                 syn::Ty::Path(_, syn::Path { global: _, segments: ref segments}) => segments.as_slice().last().unwrap().ident.to_string(),
//                                 _ => unimplemented!()
//                             };

//                             "&".to_string() + &t
//                         },
//                         _ => unimplemented!()
//                     }
//                 },
//                 syn::Ty::Path(_, syn::Path { global: _, segments: ref segments }) => segments.as_slice().last().unwrap().ident.to_string(),
//                 _ => unimplemented!()
//             },
//             _ => unimplemented!()
//         }
//     }
// }


// fn field_size(field: &syn::Field) -> u8 {
//     let size_attr = (*field).attrs
//         .iter()
//         .map(|attr| &attr.value)
//         .find(|val| match *val {
//             &syn::MetaItem::NameValue(ref ident, _) if ident.to_string() == "size" => true,
//             _ => false
//         })
//         .map(|a| match *a {
//             syn::MetaItem::NameValue(_, ref lit) => lit.clone(),
//             _ => unimplemented!()
//         })
//         .map(|lit| {
//             match lit {
//                 syn::Lit::Str(ref raw, _) => raw.clone(),
//                 _ => unimplemented!()
//             }
//         }).unwrap();
        
//     size_attr.parse()
// }


// fn to_snake_case(s: &str) -> String {
//     let (ch, next, mut acc): (Option<char>, Option<char>, String) =
//         s.chars().fold((None, None, String::new()), |(prev, ch, mut acc), next| {
//             if let Some(ch) = ch {
//                 if let Some(prev) = prev {
//                     if ch.is_uppercase() {
//                         if prev.is_lowercase() || prev.is_numeric() ||
//                             (prev.is_uppercase() && next.is_lowercase())
//                         {
//                             acc.push('_');
//                         }
//                     }
//                 }
//                 acc.extend(ch.to_lowercase());
//             }
//             (ch, Some(next), acc)
//         });
//     if let Some(next) = next {
//         if let Some(ch) = ch {
//             if (ch.is_lowercase() || ch.is_numeric()) && next.is_uppercase() {
//                 acc.push('_');
//             }
//         }
//         acc.extend(next.to_lowercase());
//     }
//     acc
// }