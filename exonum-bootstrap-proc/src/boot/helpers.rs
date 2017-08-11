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

use syn;

// ███████╗ ██████╗██╗  ██╗███████╗███╗   ███╗ █████╗     ██████╗ ██╗   ██╗██╗██╗     ██████╗ ███████╗██████╗ ███████╗
// ██╔════╝██╔════╝██║  ██║██╔════╝████╗ ████║██╔══██╗    ██╔══██╗██║   ██║██║██║     ██╔══██╗██╔════╝██╔══██╗██╔════╝
// ███████╗██║     ███████║█████╗  ██╔████╔██║███████║    ██████╔╝██║   ██║██║██║     ██║  ██║█████╗  ██████╔╝███████╗
// ╚════██║██║     ██╔══██║██╔══╝  ██║╚██╔╝██║██╔══██║    ██╔══██╗██║   ██║██║██║     ██║  ██║██╔══╝  ██╔══██╗╚════██║
// ███████║╚██████╗██║  ██║███████╗██║ ╚═╝ ██║██║  ██║    ██████╔╝╚██████╔╝██║███████╗██████╔╝███████╗██║  ██║███████║
// ╚══════╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚═╝  ╚═╝    ╚═════╝  ╚═════╝ ╚═╝╚══════╝╚═════╝ ╚══════╝╚═╝  ╚═╝╚══════╝

pub trait ServiceBuilder {
  fn service_name(&self) -> String;
  fn service_id(&self) -> u16;
  fn collect_records(&self) -> Vec<&syn::Field>;
  fn collect_inputs(&self) -> Vec<&syn::Field>;
  fn collect_outputs(&self) -> Vec<&syn::Field>;
}

impl ServiceBuilder for syn::MacroInput {
  fn service_name(&self) -> String {
    format!("{}", self.ident.to_string().replace("__", ""))
  }

  fn service_id(&self) -> u16 {
    self.attrs.iter()
      .find(|attr| attr.is_name("id"))
      .map(|attr| attr.get_value())
      .unwrap()
      .parse::<u16>()
      .unwrap()
  }

  fn collect_records(&self) -> Vec<&syn::Field> {
    match self.body {
        syn::Body::Struct(ref variant) => match *variant {
          syn::VariantData::Struct(ref fields) => fields.iter().filter(|f| f.has_attr("record")).collect(),
          _ => panic!("Only struct can be a record")
        },
        _ => panic!("Only struct can be a record"),
    }    
  }

  fn collect_inputs(&self) -> Vec<&syn::Field> {
    match self.body {
        syn::Body::Struct(ref variant) => match *variant {
          syn::VariantData::Struct(ref fields) => fields.iter().filter(|f| f.has_attr("input")).collect(),
          _ => panic!("Only struct can be a record")
        },
        _ => panic!("Only struct can be a record"),
    }    
  }

  fn collect_outputs(&self) -> Vec<&syn::Field> {
    match self.body {
        syn::Body::Struct(ref variant) => match *variant {
          syn::VariantData::Struct(ref fields) => fields.iter().filter(|f| f.has_attr("output")).collect(),
          _ => panic!("Only struct can be a record")
        },
        _ => panic!("Only struct can be a record"),
    }    
  }
}

pub trait RecordBuilder {
  fn record_name(&self) -> String;
  fn message_name(&self) -> String;
  fn record_id(&self)   -> u16;
  fn collect_fields(&self) -> Vec<&syn::Field>;
}
                                                                                                                  
impl RecordBuilder for syn::MacroInput {
  // Struct name to record
  fn record_name(&self) -> String {
    self.ident.to_string().replace("__", "")
  }

  fn message_name(&self) -> String {
    self.ident.to_string().replace("__", "Tx")
  }

  fn record_id(&self) -> u16 {
    self.attrs.iter()
      .find(|attr| attr.is_name("id"))
      .map(|attr| attr.get_value())
      .unwrap()
      .parse::<u16>()
      .unwrap()
  }

  fn collect_fields(&self) -> Vec<&syn::Field> {
    match self.body {
        syn::Body::Struct(ref variant) => match *variant {
          syn::VariantData::Struct(ref fields) => fields.iter().collect(),
          _ => panic!("Only struct can be a record")
        },
        _ => panic!("Only struct can be a record"),
    }    
  }
}

pub trait FieldBuilder {
  fn is_key(&self) -> bool;
  fn has_setter(&self) -> bool;
  fn get_type(&self) -> String;
  fn get_size(&self) -> u16;
  fn collect_mods(&self) -> Option<&Vec<syn::MetaItem>>;
}

impl FieldBuilder for syn::Field {
  fn is_key(&self) -> bool {
    self.has_attr("key")
  }

  fn has_setter(&self) -> bool {
    self.has_attr("set")
  }

  fn get_type(&self) -> String {
    if self.has_attr("ty") {
      // Get from attribute
      self.get_attr_value("ty")
    } else {
      match self.ty {
        syn::Ty::Rptr(_, ref mubox) => match *mubox.as_ref() { 
          syn::MutTy { ty: ref ty, mutability: _} => {
              let t = match *ty {
                  syn::Ty::Path(_, syn::Path { global: _, segments: ref segments}) => segments.as_slice().last().unwrap().ident.to_string(),
                  _ => unimplemented!()
              };

              "&".to_string() + &t
          },
          _ => unimplemented!()
        },
        syn::Ty::Path(_, syn::Path { global: _, segments: ref segments }) => 
          segments.as_slice().last().unwrap().ident.to_string(),
        _ => panic!("Incorrect type")
      }
    }
  }

  fn get_size(&self) -> u16 {
    if self.has_attr("size") {
      self.get_attr_value("size").parse::<u16>().unwrap()
    } else {
      match self.get_type().as_str() {
          "&PublicKey" => 32u16,
          "&Hash"      => 32u16,

          "u8"         => 1u16,
          "u16"        => 2u16,
          "u32"        => 4u16,
          "u64"        => 8u16,
          "u128"       => 16u16,
          "u256"       => 32u16,

          "i8"         => 1u16,
          "i16"        => 2u16,
          "i32"        => 4u16,
          "i64"        => 8u16,
          "i128"       => 16u16,
          "i256"       => 32u16,

          "bool"       => 1u16,
          _            => panic!("Auto size is imposible for {}", self.get_type().as_str()),
      }
    }
  }
  
  fn collect_mods(&self) -> Option<&Vec<syn::MetaItem>> {
    match self.has_attr("config") {
      true => match self.get_attr("config").value {
        syn::MetaItem::List(_, ref nested) => Some(nested),
        _ => panic!("Config attribute should be a list")
      }
      false => None
    }
  }
}

// ███████╗██╗   ██╗███╗   ██╗    ██╗  ██╗███████╗██╗     ██████╗ ███████╗██████╗ ███████╗
// ██╔════╝╚██╗ ██╔╝████╗  ██║    ██║  ██║██╔════╝██║     ██╔══██╗██╔════╝██╔══██╗██╔════╝
// ███████╗ ╚████╔╝ ██╔██╗ ██║    ███████║█████╗  ██║     ██████╔╝█████╗  ██████╔╝███████╗
// ╚════██║  ╚██╔╝  ██║╚██╗██║    ██╔══██║██╔══╝  ██║     ██╔═══╝ ██╔══╝  ██╔══██╗╚════██║
// ███████║   ██║   ██║ ╚████║    ██║  ██║███████╗███████╗██║     ███████╗██║  ██║███████║
// ╚══════╝   ╚═╝   ╚═╝  ╚═══╝    ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝     ╚══════╝╚═╝  ╚═╝╚══════╝

pub trait FieldExt {
  fn get_name(&self) -> String;
  fn has_attr(&self, name: &str) -> bool;
  fn get_attr(&self, name: &str) -> &syn::Attribute;
  fn get_attr_safe(&self, name: &str) -> Result<&syn::Attribute, &str>;
  fn get_attr_value(&self, name: &str) -> String;
  fn get_attr_value_safe(&self, name: &str) -> Result<String, &str>;
}

pub trait AttributeExt {
  fn is_name(&self, name: &str) -> bool;
  fn get_value(&self) -> String;
  fn get_value_safe(&self) -> Result<String, &str>;
}

pub trait IdentExt {
  fn from_u16(num : u16) -> syn::Ident;
}

impl IdentExt for syn::Ident {
  fn from_u16(num : u16) -> syn::Ident {
    syn::Ident::from(num.to_string().replace("u16", ""))
  }
}

impl FieldExt for syn::Field {       
  fn get_name(&self) -> String {
    let ident = self.ident.clone();
    ident.unwrap().to_string()
  }

  fn has_attr(&self, name : &str) -> bool {
    self.attrs.iter().any(|attr| attr.is_name(&name))
  }

  fn get_attr(&self, name : &str) -> &syn::Attribute {
    let found = self.attrs.iter().find(|attr| attr.is_name(&name));
    match found {
      Some(found) => found,
      None => panic!("Attribute not found")
    }
  }

  fn get_attr_safe(&self, name : &str) -> Result<&syn::Attribute, &str> {
    let found = self.attrs.iter().find(|attr| attr.is_name(&name));
    match found {
      Some(found) => Ok(found),
      None => Err("Attribute not found")
    }
  }

  fn get_attr_value(&self, name : &str) -> String {
    self.get_attr(name).get_value()
  }

  fn get_attr_value_safe(&self, name : &str) -> Result<String, &str> {
    self.get_attr_safe(name).expect("Field not found").get_value_safe()
  }
}

impl AttributeExt for syn::Attribute {
  fn is_name(&self, name : &str) -> bool {
    match self.value {
        syn::MetaItem::List(ref ident, _) if ident.to_string() == name => true,
        syn::MetaItem::NameValue(ref ident, _) if ident.to_string() == name => true,
        syn::MetaItem::Word(ref ident) if ident.to_string() == name => true,
        _ => false
    }
  }

  fn get_value(&self) -> String {
    match self.value {
        syn::MetaItem::NameValue(_, syn::Lit::Str(ref raw, _)) => raw.clone(),
        _ => panic!("Attribute isn't name value typed!")
    }
  }

  fn get_value_safe(&self) -> Result<String, &str> {
    match self.value {
        syn::MetaItem::NameValue(_, syn::Lit::Str(ref raw, _)) => Ok(raw.clone()),
        _ => Err("Attribute isn't name value typed!")
    }
  }
}

// ███████╗████████╗██████╗ ██╗███╗   ██╗ ██████╗ ███████╗
// ██╔════╝╚══██╔══╝██╔══██╗██║████╗  ██║██╔════╝ ██╔════╝
// ███████╗   ██║   ██████╔╝██║██╔██╗ ██║██║  ███╗███████╗
// ╚════██║   ██║   ██╔══██╗██║██║╚██╗██║██║   ██║╚════██║
// ███████║   ██║   ██║  ██║██║██║ ╚████║╚██████╔╝███████║
// ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
                                                      
pub fn to_snake_case(s: &str) -> String {
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

