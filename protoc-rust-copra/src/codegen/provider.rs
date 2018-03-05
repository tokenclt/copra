use protobuf::descriptor::ServiceDescriptorProto;
use protobuf::descriptorx::RootScope;
use std::io;

use super::name::*;

pub fn generate_service_trait(
    proto: &ServiceDescriptorProto,
    root: &RootScope,
) -> io::Result<String> {
    let trait_name = trait_name(proto)?;
    let method_names = method_names(proto)?;
    let future_names = future_names(proto)?;
    let request_types = request_types(proto, root)?;
    let response_types = response_types(proto, root)?;

    let mut gen = String::new();

    gen = gen + &format!(r"pub trait {} {{", trait_name);

    for (future, resp) in future_names.iter().zip(response_types.iter()) {
        gen = gen
            + &format!(
                r"
    type {}: ::futures::Future<
        Item = ({}, ::copra::controller::Controller), 
        Error = ::copra::service::ProviderSetError,
    > + 'static;
",
                future, resp
            );
    }

    for ((method, req), future) in method_names
        .iter()
        .zip(request_types.iter())
        .zip(future_names.iter())
    {
        gen = gen
            + &format!(
                r"
    fn {}(&self, msg: ({}, ::copra::controller::Controller)) -> Self::{};
",
                method, req, future
            );
    }

    gen = gen + "}";

    Ok(gen)
}

pub fn generate_registrant_basic(proto: &ServiceDescriptorProto) -> io::Result<String> {
    let reg_name = registrant_name(proto)?;
    let mut gen = String::new();

    gen = gen
        + &format!(
            r"
pub struct {}<S> {{
    provider: S,
}}

impl<S> {}<S> {{
    pub fn new(provider: S) -> Self {{
        {} {{ provider }}
    }}
}}",
            reg_name, reg_name, reg_name
        );

    Ok(gen)
}

pub fn generate_registrant_service(
    proto: &ServiceDescriptorProto,
    root: &RootScope,
) -> io::Result<String> {
    let service_name = service_name(proto)?;
    let trait_name = trait_name(proto)?;
    let method_names = method_names(proto)?;
    let future_names = future_names(proto)?;
    let request_types = request_types(proto, root)?;
    let response_types = response_types(proto, root)?;
    let reg_name = registrant_name(proto)?;

    let mut gen = String::new();

    // generate Registrant implementation
    gen = gen
        + &format!(
            r"
impl<S> ::copra::dispatcher::Registrant for {}<S>
where
    S: {} + Clone + Send + Sync + 'static,
{{
    fn methods(&self) -> Vec<(String, ::copra::service::BoxedNewUnifiedMethod)> {{
        let mut entries = Vec::new();
        let provider = &self.provider;
    ",
            reg_name, trait_name
        );

    for (((req, resp), future), method) in request_types
        .iter()
        .zip(response_types.iter())
        .zip(future_names.iter())
        .zip(method_names.iter())
    {
        gen = gen
            + &format!(
                r#"
        {{
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: {} + Clone,
            {{
                type Request = ({}, ::copra::controller::Controller);
                type Response = ({}, ::copra::controller::Controller);
                type Error = ::copra::service::ProviderSetError;
                type Future = <S as {}>::{};

                fn call(&self, req: Self::Request) -> Self::Future {{
                    self.0.{}(req)
                }}
            }}

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::CodecMethodBundle::new(
                ::copra::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::copra::service::NewUnifiedMethod::new(method);
            entries.push((
                "{}".to_string(), 
                Box::new(new_method) as ::copra::service::BoxedNewUnifiedMethod,
            ));
        }}
        "#,
                trait_name, req, resp, trait_name, future, method, method
            );
    }

    gen = gen
        + &format!(
            r"
        entries
    }}
}}
"
        );

    // generate NamedRegistrant implementation
    gen = gen
        + &format!(
            r#"
impl<S> ::copra::dispatcher::NamedRegistrant for {}<S> 
where 
    S: {} + Clone + Send + Sync + 'static,
{{
    fn name() -> &'static str {{
        "{}"
    }}
}}"#,
            reg_name, trait_name, service_name
        );

    Ok(gen)
}
