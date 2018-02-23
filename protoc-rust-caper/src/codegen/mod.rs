use protobuf::compiler_plugin::GenResult;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::descriptorx;
use std::collections::HashMap;
use std::io;

use self::provider::*;
use self::stub::*;

pub mod name;
pub mod provider;
pub mod stub;

pub fn gen(
    file_descriptors: &[FileDescriptorProto],
    files_to_generate: &[String],
) -> io::Result<Vec<GenResult>> {
    let files_map: HashMap<_, _> = file_descriptors.iter().map(|f| (f.get_name(), f)).collect();

    let root_scope = descriptorx::RootScope { file_descriptors };

    let mut results = Vec::new();

    for file_name in files_to_generate {
        let file = files_map[file_name.as_str()];
        if file.get_service().is_empty() {
            continue;
        }

        results.push(gen_file(file, &root_scope)?);
    }

    Ok(results)
}

fn gen_file(file: &FileDescriptorProto, root: &descriptorx::RootScope) -> io::Result<GenResult> {
    let base_name = descriptorx::proto_path_to_rust_mod(file.get_name());
    let mut snippets = Vec::new();

    for service in file.get_service() {
        snippets.push(generate_service_trait(service, root)?);
        snippets.push(generate_registrant_basic(service)?);
        snippets.push(generate_registrant_service(service, root)?);
        snippets.push(generate_client_stub(service, root)?);
    }

    let content = snippets.iter().fold(generate_file_header(), |acc, x| acc + "\n" + x);

    Ok(GenResult {
        name: base_name + "_caper.rs",
        content: content.into(),
    })
}

fn generate_file_header() -> String {
    let header = "\
// This file is generated, Do not edit
// @generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(dead_code)]
";
    header.to_string()
}
