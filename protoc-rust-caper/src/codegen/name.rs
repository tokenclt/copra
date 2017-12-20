use inflector::Inflector;
use protobuf::descriptor::ServiceDescriptorProto;
use protobuf::descriptorx::{RootScope, WithScope};
use std::io;

pub fn full_message_name(root: &RootScope, input: &str) -> String {
    format!("super::{}", root.find_message(input).rust_fq_name())
}

pub fn service_name(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let service_name = proto.get_name();
    if !service_name.is_class_case() {
        warn!(
            "Service name is not ClassCase, should be converted from {} to {}",
            service_name,
            service_name.to_class_case()
        );
    } else {
        // TODO: report inflect issue
        // Err(io::Error::new(
        //     io::ErrorKind::Other,
        //     format!("Service name should be ClassCase, found {}", service_name),
        // ))

    }
    Ok(service_name.into())
}

pub fn registrant_name(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let service_name = service_name(proto)?;
    Ok(format!("{}Registrant", service_name))
}

pub fn trait_name(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let service_name = service_name(proto)?;
    Ok(format!("{}Service", service_name))
}

pub fn stub_name(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let service_name = service_name(proto)?;
    Ok(format!("{}Stub", service_name))
}

pub fn method_names(proto: &ServiceDescriptorProto) -> io::Result<Vec<String>> {
    proto
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
                Ok(name.into())
            }
        })
        .collect()
}

pub fn future_names(proto: &ServiceDescriptorProto) -> io::Result<Vec<String>> {
    let names = method_names(proto)?
        .into_iter()
        .map(|name| name.to_class_case())
        .map(|name| name + "Future")
        .collect();
    Ok(names)
}

pub fn wrapper_names(proto: &ServiceDescriptorProto) -> io::Result<Vec<String>> {
    let names = method_names(proto)?
        .into_iter()
        .map(|name| name + "_wrapper")
        .collect();
    Ok(names)
}

pub fn request_types(proto: &ServiceDescriptorProto, root: &RootScope) -> io::Result<Vec<String>> {
    let types = proto
        .get_method()
        .iter()
        .map(|name| name.get_input_type())
        .map(|name| full_message_name(root, name))
        .collect();
    Ok(types)
}

pub fn response_types(proto: &ServiceDescriptorProto, root: &RootScope) -> io::Result<Vec<String>> {
    let types = proto
        .get_method()
        .iter()
        .map(|name| name.get_output_type())
        .map(|name| full_message_name(root, name))
        .collect();
    Ok(types)
}
