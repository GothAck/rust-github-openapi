use std::{
  boxed::Box,
  collections::{HashSet},
  fs::{File},
  io::prelude::*,
  path::Path,
};
use clap::{Parser, crate_version, crate_authors};
extern crate codegen;
use convert_case::{Case, Casing};
use env_logger::Env;
use log::{info, error};
use openapiv3::OpenAPI;
use serde_json;
use serde_yaml;

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
}

impl PropType {
  fn new() -> Self {
    PropType {
      type_: "".to_string(),
      subtype: None,
      nullable: false,
      required: true,
      serde_annotations: vec![],
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

  fn to_field(&self, name: &String) -> codegen::Field {
    let mut serde_annotations = self.serde_annotations.clone();
    if ! self.required {
      serde_annotations.push("skip_serializing_if = \"Option::is_none\"".to_string());
    }
    let mut field = codegen::Field::new(name, self.to_prop_type());
    if serde_annotations.len() > 0 {
      field.annotation(vec![&format!("#[serde({})]", serde_annotations.join(", ")).to_string()]);
    }
    field
  }
}

struct Builder {
  pub scope: Box<codegen::Scope>,
  pub module_path: Vec<String>,
}

enum BuilderModule <'a> {
  Module(&'a mut codegen::Module),
  Scope(&'a mut codegen::Scope)
}

impl Builder {
  fn new() -> Self {
    Builder {
      scope: Box::new(codegen::Scope::new()),
      module_path: vec![],
    }
  }

  fn cur_scope_or_module(&mut self) -> BuilderModule {
    if self.module_path.len() == 0 {
      return BuilderModule::Scope(self.scope.as_mut());
    }
    let mut iter = self.module_path.iter();
    let mut module: &mut codegen::Module = self.scope.get_or_new_module(iter.next().unwrap());
    for module_name in iter {
      module = module.get_or_new_module(module_name);
    }
    BuilderModule::Module(module)
  }

  fn cur_module(&mut self) -> Option<&mut codegen::Module> {
    if self.module_path.len() == 0 {
      return None;
    }
    let mut iter = self.module_path.iter();
    let mut module: &mut codegen::Module = self.scope.get_or_new_module(iter.next().unwrap());
    for module_name in iter {
      module = module.get_or_new_module(module_name);
    }
    Some(module)
  }

  fn get_proptype_box(&mut self, parent_name: &String, prop_name: &String, val: &openapiv3::ReferenceOr<Box<openapiv3::Schema>>) -> PropType {
    self.get_proptype(parent_name, prop_name, &val.clone().unbox())
  }

  fn get_proptype(&mut self, parent_name: &String, prop_name: &String, val: &openapiv3::ReferenceOr<openapiv3::Schema>) -> PropType {
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
              let name = &format!("{}-{}", parent_name, prop_name).to_string().to_case(Case::Pascal);
              let mut str = self.new_struct(name, &obj);
              res.type_ = name.clone();
            },
            openapiv3::Type::Array(arr) => {
              res.type_ = "Vec".to_string();
              res.subtype = Some(Box::from(self.get_proptype_box(parent_name, prop_name, &arr.items)));
            },
            openapiv3::Type::Boolean {} => res.type_ = "bool".to_string(),
          },
          openapiv3::SchemaKind::OneOf {one_of} => {
            let name = &format!("{}-{}-OneOf", parent_name, prop_name).to_string().to_case(Case::Pascal);
            let sub_name = &format!("{}-{}", parent_name, prop_name).to_string().to_case(Case::Pascal);
            let mut enm = self.new_enum(name, sub_name, &one_of);
            res.type_ = name.clone();
          },
          openapiv3::SchemaKind::AllOf {all_of} => { error!("Prop type AllOf {}", parent_name) },
          openapiv3::SchemaKind::AnyOf {any_of} => { error!("Prop type AnyOf {}", parent_name) },
          openapiv3::SchemaKind::Any(any_schema) => { error!("Prop type Any {}", parent_name) },
        }
      }
      openapiv3::ReferenceOr::Reference {reference} => {
        if ! reference.starts_with("#/") {
          error!("Prop reference {} INVALID", reference);
        } else {
          let reference: String = reference.strip_prefix("#/").unwrap().to_string();
          let mut reference_arr: Vec<String> = reference.split("/").map(|v| v.to_string()).collect();
          let last = reference_arr.pop().unwrap();
          reference_arr.insert(0, "crate".to_string());
          reference_arr.push(last.to_case(Case::Pascal));
          
          res.type_ = reference_arr.join("::");
        }
      },
    };
    res
  }

  fn new_enum(&mut self, name: &String, sub_name: &String, val: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>) -> &mut codegen::Enum {
    let mut variants: Vec<codegen::Variant> = vec![];

    for variant in val {
      let mut proptype = self.get_proptype(sub_name, &"".to_string(), variant);
      proptype.nullable = false;
      variants.push(codegen::Variant::new(&format!("{}({})", proptype.type_, proptype.to_prop_type())));
    }

    let enm = match self.cur_scope_or_module() {
      BuilderModule::Scope(scope) => {
        scope.new_enum(&name.to_case(Case::Pascal))
          .derive("Debug")
          .derive("Serialize")
          .derive("Deserialize")
      },
      BuilderModule::Module(module) => {
        module.new_enum(&name.to_case(Case::Pascal))
          .derive("Debug")
          .derive("Serialize")
          .derive("Deserialize")
      },
    };

    for variant in variants {
      enm.push_variant(variant);
    }

    enm
  }

  fn new_struct(&mut self, name: &String, val: &openapiv3::ObjectType) -> &mut codegen::Struct {
    let mut fields: Vec<codegen::Field> = vec![];
    let required: HashSet<&String> = val.required.iter().collect();
    for (prop_name, prop) in &val.properties {
      let mut proptype = self.get_proptype_box(name, prop_name, &prop);

      proptype.required = required.contains(prop_name);

      let field = match prop_name.as_str() {
        "type" | "mod" | "ref" | "self" => {
          proptype.serde_annotations.push(format!("rename=\"{}\"", prop_name));
          proptype.to_field(&format!("{}_", prop_name))
        },
        "+1" | "-1" => {
          proptype.serde_annotations.push(format!("rename=\"{}\"", prop_name));
          if prop_name == "+1" {
            proptype.to_field(&"plus_one".to_string())
          } else {
            proptype.to_field(&"minus_one".to_string())
          }
        }
        _ => match prop_name.chars().next() {
          Some('@' | '$') => {
            proptype.serde_annotations.push(format!("rename=\"{}\"", prop_name));
            let mut chars = prop_name.chars();
            chars.next();
            proptype.to_field(&String::from(chars.as_str()))
          }
          _ => proptype.to_field(prop_name)
        }
      };

      fields.push(field);
    }

    let str = match self.cur_scope_or_module() {
      BuilderModule::Scope(scope) => {
        scope.new_struct(&name.to_case(Case::Pascal))
          .derive("Debug")
          .derive("Serialize")
          .derive("Deserialize")
      },
      BuilderModule::Module(module) => {
        module.new_struct(&name.to_case(Case::Pascal))
          .derive("Debug")
          .derive("Serialize")
          .derive("Deserialize")
      },
    };

    for field in fields {
      str.push_field(field);
    }

    str
  }

  fn supers(&self, reference: &str) -> String {
    let mut supers: Vec<&str> = self.module_path.iter().map(|v| "super").collect();
    supers.push(reference);
    supers.join("::")
  }
}

fn main() -> anyhow::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let flags = Flags::parse();
  let api_spec = Path::new(&flags.api_spec);

  let openapi: OpenAPI = match api_spec.extension().map(|v| v.to_str()).flatten() {
    Some("json") => {
      serde_json::from_reader(File::open(api_spec).unwrap()).map_err(|e| anyhow::Error::from(e))
    },
    Some("yaml") => {
      serde_yaml::from_reader(File::open(api_spec).unwrap()).map_err(|e| anyhow::Error::from(e))
    },
    Some(ext) => Result::Err(anyhow::anyhow!("Invalid file extension {:?}", ext)),
    None => Result::Err(anyhow::anyhow!("Invalid file type {:?}", api_spec))
  }?;

  if let Some(components) = openapi.components {
    let mut builder = Builder::new();
    builder.module_path.push("components".to_string());
    builder.module_path.push("schemas".to_string());
    builder.cur_module().unwrap().import("serde", "{Serialize, Deserialize}");
    for (name, schema) in components.schemas {
      match schema {
        openapiv3::ReferenceOr::Item(schema) => {
          match schema.schema_kind {
            openapiv3::SchemaKind::Type(type_) => {
              match type_ {
                openapiv3::Type::String(val) => {error!("Component type String {:?}", val)},
                openapiv3::Type::Number(val) => {error!("Component type Number {:?}", val)},
                openapiv3::Type::Integer(val) => {error!("Component type Integer {:?}", val)},
                openapiv3::Type::Object(val) => { builder.new_struct(&name, &val); },
                openapiv3::Type::Array(val) => {error!("Component type Array {:?}", val)},
                openapiv3::Type::Boolean {} => {error!("Component type Boolean")},
              }
            },
            openapiv3::SchemaKind::OneOf {one_of} => {
              error!("Component type OneOf");
            },
            openapiv3::SchemaKind::AllOf {all_of} => {
              error!("Component type AllOf");
            },
            openapiv3::SchemaKind::AnyOf {any_of} => {
              error!("Component type AnyOf");
            },
            openapiv3::SchemaKind::Any(any_schema) => {
              error!("Component type Any");
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
        .write_all(builder.scope.to_string().as_bytes())?;
    }
    // info!("{}", builder.scope.to_string())
    // info!("{:?}", components.schemas);
  }


  Ok(())
}
