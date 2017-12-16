use inflector::Inflector;
use protobuf::compiler_plugin::GenResult;
use protobuf::descriptor::{FieldDescriptorProto, FileDescriptorProto, ServiceDescriptorProto};
use protobuf::descriptorx::RootScope;
use std::io;
use std::slice::Iter;

pub fn gen(
    file_descriptors: &[FileDescriptorProto],
    files_to_generate: &[String],
) -> io::Result<Vec<GenResult>> {
    let content: Vec<u8> = r"// testing".into();
    let results = vec![
        GenResult {
            name: "testing.rs".to_string(),
            content,
        },
    ];
    Ok(results)
}

fn get_service_name(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let service_name = proto.get_name();
    let service_name = if service_name.is_class_case() {
        format!("{}Service", service_name)
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Service name should be ClassCase",
        ));
    };
    Ok(service_name)
}

fn get_method_names(proto: &ServiceDescriptorProto) -> io::Result<Iter<&str>> {
    let method_names: io::Result<Vec<&str>> = proto
        .get_method()
        .iter()
        .map(|method| {
            let name = method.get_name();
            if !name.is_snake_case() {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Method name should be snake_case",
                ))
            } else {
                Ok(name)
            }
        })
        .collect()?;
    Ok(method_names.iter())
}

fn generate_service_trait(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let service_name = get_service_name(protr)?;
    let method_names = get_method_names(proto)?;
    let future_names = method_names
        .map(|name| name.to_class_case())
        .map(|name| format!("{}Future", name));
    let request_types = proto
        .get_method()
        .iter()
        .map(|method| method.get_input_type());
    let response_types = proto
        .get_method()
        .iter()
        .map(|method| method.get_output_type());

    let tokens = quote!{
        pub trait #service_name {
            #(
                type #future_names: ::futures::Future<Item = #response_types, Error = ::caper::service::MethodError> + 'static;
            )*

            #(
                fn #method_names(&self, msg: #request_types) -> Self::#future_names;
            )*
        }
    };

    Ok(tokens.into_string())
}

fn generate_registrant_basic(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let service_name = get_service_name(proto)?;
    let regis_name = format!("{}Registrant", service_name);

    let tokens = quote!{
        pub struct #regis_name<S> {
            provider: S,
        }

        impl<S> #regis_name<S> {
            pub fn new(provider: S) -> Self {
                #regis_name { provider }
            }
        }
    };
}

fn generate_registrant_service(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let service_name = get_service_name(proto)?;
    let regis_name = format!("{}Registrant", service_name);
}
