use std::{
  boxed::Box,
  collections::{HashSet},
  fs::{File},
  io::prelude::*,
  path::Path,
};
use clap::{Parser, crate_version, crate_authors};
extern crate codegen2;
use convert_case::{Case, Casing};
use env_logger::Env;
use log::{info, error};
use openapiv3::OpenAPI;
extern crate serde_json;
extern crate serde_yaml;

#[derive(Parser)]
#[clap(version=crate_version!(), author=crate_authors!())]
struct Flags {
  #[clap()]
  api_spec: String,
  #[clap()]
  out_file: String,
}

#[derive(Debug)]
struct PropType {
  pub type_: String,
  pub subtype: Option<Box<PropType>>,
  pub nullable: bool,
  pub required: bool,
  pub serde_annotations: Vec<String>,
  pub doc: Option<Vec<String>>,
}

impl PropType {
  fn new() -> Self {
    PropType {
      type_: "".to_string(),
      subtype: None,
      nullable: false,
      required: true,
      serde_annotations: vec![],
      doc: None,
    }
  }

  fn to_prop_type(&self) -> String {
    let mut type_ = self.type_.clone();
    if let Some(subtype) = &self.subtype {
      type_ = format!("{}<{}>", type_, subtype.to_prop_type());
    }
    if self.nullable || ! self.required {
      type_ = format!("Option<{}>", type_);
    }
    type_
  }

  fn to_field(&self, name: &str) -> codegen2::Field {
    let mut serde_annotations = self.serde_annotations.clone();
    if ! self.required {
      serde_annotations.push("skip_serializing_if = \"Option::is_none\"".to_string());
    }
    let mut field = codegen2::Field::new(name, self.to_prop_type());
    if ! serde_annotations.is_empty() {
      field.annotation(vec![&format!("#[serde({})]", serde_annotations.join(", "))]);
    }
    if let Some(doc) = &self.doc {
      field.doc(doc.iter().map(|v| v.as_str()).collect());
    }
    field
  }
}

struct Builder {
  pub scope: Box<codegen2::Scope>,
  pub module_path: Vec<String>,
}

impl Builder {
  fn new() -> Self {
    Builder {
      scope: Box::new(codegen2::Scope::new()),
      module_path: vec![],
    }
  }

  fn cur_scope_or_module(&mut self) -> &mut codegen2::Scope {
    if self.module_path.is_empty() {
      return self.scope.as_mut();
    }
    let mut iter = self.module_path.iter();
    let mut module: &mut codegen2::Module = self.scope.get_or_new_module(iter.next().unwrap());
    for module_name in iter {
      module = module.get_or_new_module(module_name);
    }
    module.scope()
  }

  fn cur_module(&mut self) -> Option<&mut codegen2::Module> {
    if self.module_path.is_empty() {
      return None;
    }
    let mut iter = self.module_path.iter();
    let mut module: &mut codegen2::Module = self.scope.get_or_new_module(iter.next().unwrap());
    for module_name in iter {
      module = module.get_or_new_module(module_name);
    }
    Some(module)
  }

  fn get_proptype_box(&mut self, parent_name: &str, prop_name: &str, val: &openapiv3::ReferenceOr<Box<openapiv3::Schema>>) -> PropType {
    self.get_proptype(parent_name, prop_name, &val.clone().unbox())
  }

  fn get_proptype(&mut self, parent_name: &str, prop_name: &str, val: &openapiv3::ReferenceOr<openapiv3::Schema>) -> PropType {
    let mut res = PropType::new();
    match val {
      openapiv3::ReferenceOr::Item(item) => {
        res.nullable = item.schema_data.nullable;
        match &item.schema_kind {
          openapiv3::SchemaKind::Type(type_) => match type_ {
            openapiv3::Type::String(_) => res.type_ = "String".to_string(),
            openapiv3::Type::Number(_) => res.type_ = "i64".to_string(),
            openapiv3::Type::Integer(_) => res.type_ = "i64".to_string(),
            openapiv3::Type::Object(obj) => {
              let name = &format!("{}-{}", parent_name, prop_name).to_case(Case::Pascal);
              self.new_struct(name, obj);
              res.type_ = name.clone();
            },
            openapiv3::Type::Array(arr) => {
              res.type_ = "Vec".to_string();
              let mut subtype = Box::from(self.get_proptype_box(parent_name, prop_name, &arr.items));
              if subtype.type_.is_empty() {
                subtype.type_ = "HashMap<String, String>".to_string();
              }
              res.subtype = Some(subtype);
            },
            openapiv3::Type::Boolean {} => res.type_ = "bool".to_string(),
          },
          openapiv3::SchemaKind::OneOf {one_of} => {
            let name = &format!("{}-{}-OneOf", parent_name, prop_name).to_case(Case::Pascal);
            let sub_name = &format!("{}-{}", parent_name, prop_name).to_case(Case::Pascal);
            let enm = self.new_enum(name, sub_name, one_of);
            enm.doc("OneOf");
            enm.r#macro("#[serde(untagged)]");
            res.type_ = name.clone();
          },
          openapiv3::SchemaKind::AllOf {all_of: _} => { error!("UNHANDLED: Prop type AllOf {}", parent_name) },
          openapiv3::SchemaKind::AnyOf {any_of} => {
            let name = &format!("{}-{}-OneOf", parent_name, prop_name).to_case(Case::Pascal);
            let sub_name = &format!("{}-{}", parent_name, prop_name).to_case(Case::Pascal);
            let enm = self.new_enum(name, sub_name, any_of);
            enm.doc(&"AnyOf".to_string());
            enm.r#macro("#[serde(untagged)]");
            res.type_ = name.clone();
          },
          openapiv3::SchemaKind::Any(_) => {
            res.type_ = "HashMap<String, String>".to_string();
            res.serde_annotations.push("skip".to_string());
          },
        }
      }
      openapiv3::ReferenceOr::Reference {reference} => {
        if ! reference.starts_with("#/") {
          error!("Prop reference {} INVALID", reference);
        } else {
          let reference: String = reference.strip_prefix("#/").unwrap().to_string();
          let mut reference_arr: Vec<String> = reference.split('/').map(|v| v.to_string()).collect();
          let last = reference_arr.pop().unwrap();
          reference_arr.insert(0, "crate".to_string());
          reference_arr.push(last.to_case(Case::Pascal));

          res.type_ = reference_arr.join("::");
          res.doc = Some(vec![format!("Ref {}", reference)]);
        }
      },
    };
    res
  }

  fn new_enum(&mut self, name: &str, sub_name: &str, val: &[openapiv3::ReferenceOr<openapiv3::Schema>]) -> &mut codegen2::Enum {
    let mut variants: Vec<codegen2::Variant> = vec![];

    for (i, variant) in val.iter().enumerate() {
      let mut proptype = self.get_proptype(sub_name, &format!("{}", i), variant);
      proptype.nullable = false;
      variants.push(codegen2::Variant::new(&format!("{}({})", proptype.type_.rsplit("::").next().unwrap(), proptype.to_prop_type())));
    }

    let enm = self.cur_scope_or_module().new_enum(&name.to_case(Case::Pascal))
      .derive("Debug")
      .derive("Serialize")
      .derive("Deserialize");

    for variant in variants {
      enm.push_variant(variant);
    }

    enm.vis("pub");

    enm
  }

  fn new_struct(&mut self, name: &str, val: &openapiv3::ObjectType) -> &mut codegen2::Struct {
    let mut fields: Vec<codegen2::Field> = vec![];
    let required: HashSet<&String> = val.required.iter().collect();
    for (prop_name, prop) in &val.properties {
      let mut proptype = self.get_proptype_box(name, prop_name, prop);

      proptype.required = required.contains(prop_name);

      let new_prop_name = match prop_name.as_str() {
        "+1" | "-1" => prop_name.to_string(),
        _ => prop_name.to_case(Case::Snake),
      };

      let field = match new_prop_name.as_str() {
        "type" | "mod" | "ref" | "self" => {
          proptype.serde_annotations.push(format!("rename=\"{}\"", prop_name));
          proptype.to_field(&format!("{}_", new_prop_name))
        },
        "+1" | "-1" => {
          proptype.serde_annotations.push(format!("rename=\"{}\"", prop_name));
          if new_prop_name == "+1" {
            proptype.to_field(&"plus_one".to_string())
          } else {
            proptype.to_field(&"minus_one".to_string())
          }
        },
        "$ref" => {
          proptype.serde_annotations.push(format!("rename=\"{}\"", prop_name));
          let mut chars = new_prop_name.chars();
          chars.next();
          proptype.to_field(&format!("{}_", chars.as_str()))
        },
        _ => match new_prop_name.chars().next() {
          Some('@' | '$') => {
            proptype.serde_annotations.push(format!("rename=\"{}\"", prop_name));
            let mut chars = new_prop_name.chars();
            chars.next();
            proptype.to_field(&String::from(chars.as_str()))
          },
          _ => {
            if &new_prop_name != prop_name {
              proptype.serde_annotations.push(format!("rename=\"{}\"", prop_name))
            }
            proptype.to_field(&new_prop_name)
          },
        }
      };

      fields.push(field);
    }

    let str = self.cur_scope_or_module().new_struct(&name.to_case(Case::Pascal))
      .derive("Debug")
      .derive("Serialize")
      .derive("Deserialize");

    for field in fields {
      str.push_field(field);
    }

    str.vis("pub");

    str
  }

  fn new_typedef(&mut self, name: &str, val: &openapiv3::Type) {
    let name = name.to_case(Case::Pascal);
    match val {
      openapiv3::Type::String(_) => {
        self.cur_scope_or_module().raw(&format!("pub type {} = String;", name));
      },
      openapiv3::Type::Number(_) => {
        self.cur_scope_or_module().raw(&format!("pub type {} = i64;", name));
      },
      openapiv3::Type::Integer(_) => {
        self.cur_scope_or_module().raw(&format!("pub type {} = i64;", name));
      },
      openapiv3::Type::Object(object) => {
        error!("UNHANDLED: new_typedef {} {:?}", name, object);
      },
      openapiv3::Type::Array(array) => {
        let subtype = Box::from(self.get_proptype_box(&name, &"Arr".to_string(), &array.items));
        self.cur_scope_or_module().raw(&format!("pub type {} = Vec<{}>;", name, subtype.to_prop_type()));
      },
      openapiv3::Type::Boolean {} => {
        self.cur_scope_or_module().raw(&format!("pub type {} = bool;", name));
      },
    }
  }

  fn new_anytypedef(&mut self, name: &str) {
    let name = name.to_case(Case::Pascal);
    self.cur_scope_or_module().raw(&format!("pub type {} = HashMap<String, String>;", name));
  }
}

fn main() -> anyhow::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let flags = Flags::parse();
  let api_spec = Path::new(&flags.api_spec);

  let openapi: OpenAPI = match api_spec.extension().map(|v| v.to_str()).flatten() {
    Some("json") => {
      serde_json::from_reader(File::open(api_spec).unwrap()).map_err(anyhow::Error::from)
    },
    Some("yaml") => {
      serde_yaml::from_reader(File::open(api_spec).unwrap()).map_err(anyhow::Error::from)
    },
    Some(ext) => Result::Err(anyhow::anyhow!("Invalid file extension {:?}", ext)),
    None => Result::Err(anyhow::anyhow!("Invalid file type {:?}", api_spec))
  }?;

  if let Some(components) = openapi.components {
    let mut builder = Builder::new();
    builder.scope.raw("#![allow(non_camel_case_types, dead_code)]");
    builder.module_path.push("components".to_string());
    if let Some(module) = builder.cur_module() {
      module.vis("pub");
    }
    builder.module_path.push("schemas".to_string());
    if let Some(module) = builder.cur_module() {
      module.vis("pub");
      module.import("serde", "{Serialize, Deserialize}");
      module.import("std::collections", "HashMap");
    }
    for (name, schema) in components.schemas {
      match schema {
        openapiv3::ReferenceOr::Item(schema) => {
          match schema.schema_kind {
            openapiv3::SchemaKind::Type(type_) => {
              match &type_ {
                openapiv3::Type::String(_) => { builder.new_typedef(&name, &type_); },
                openapiv3::Type::Number(_) => { builder.new_typedef(&name, &type_); },
                openapiv3::Type::Integer(_) => { builder.new_typedef(&name, &type_); },
                openapiv3::Type::Object(val) => { builder.new_struct(&name, val); },
                openapiv3::Type::Array(_) => { builder.new_typedef(&name, &type_); },
                openapiv3::Type::Boolean {} => { builder.new_typedef(&name, &type_); },
              }
            },
            openapiv3::SchemaKind::OneOf {one_of} => {
              let enm = builder.new_enum(&name, &"".to_string(), &one_of);
              enm.r#macro("#[serde(untagged)]");
            },
            openapiv3::SchemaKind::AllOf {all_of: _} => {
              error!("UNHANDLED: Component type AllOf");
            },
            openapiv3::SchemaKind::AnyOf {any_of} => {
              let enm = builder.new_enum(&name, &"".to_string(), &any_of);
              enm.r#macro("#[serde(untagged)]");
            },
            openapiv3::SchemaKind::Any(_) => {
              builder.new_anytypedef(&name);
            },
          }
        },
        openapiv3::ReferenceOr::Reference {reference} => {
          info!("Found reference {} {}", name, reference);
        }
      }
    }

    if flags.out_file == "-" {
      println!("{}", builder.scope.to_string());
    } else {
      File::create(flags.out_file)?
        .write_all(format!("{}\n", builder.scope.to_string()).as_bytes())?;
    }
  }

  Ok(())
}
