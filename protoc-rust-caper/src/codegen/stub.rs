use protobuf::descriptor::ServiceDescriptorProto;
use protobuf::descriptorx::RootScope;
use std::io;

use super::name::*;

pub fn generate_client_stub(
    proto: &ServiceDescriptorProto,
    root: &RootScope,
) -> io::Result<String> {
    let service_name = service_name(proto)?;
    let stub_name = stub_name(proto)?;
    let method_names = method_names(proto)?;
    let wrapper_names = wrapper_names(proto)?;
    let request_types = request_types(proto, root)?;
    let response_types = response_types(proto, root)?;

    let mut gen = String::new();

    gen = gen
        + &format!(
            r"
#[derive(Clone)]
pub struct {}<'a> {{",
            stub_name
        );

    for ((wrap, resp), req) in wrapper_names
        .iter()
        .zip(response_types.iter())
        .zip(request_types.iter())
    {
        gen = gen
            + &format!(
                r"
    {}: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<{}, {}>>,
",
                wrap, resp, req
            );
    }

    gen = gen
        + &format!(
            r"
}}

impl<'a> {}<'a> {{
    pub fn new(channel: &'a ::caper::channel::Channel) -> Self {{
        {} {{
    ",
            stub_name, stub_name
        );

    for wrap in wrapper_names.iter() {
        gen = gen
            + &format!(
                r"
            {}: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),
",
                wrap
            );
    }

    gen = gen
        + r"
        }
    }";

    for (((method, req), resp), wrap) in method_names
        .iter()
        .zip(request_types.iter())
        .zip(response_types.iter())
        .zip(wrapper_names.iter())
    {
        gen = gen
            + &format!(
                r#"
    pub fn {}(&'a self, msg: {}) -> ::caper::stub::StubCallFuture<'a, {}> {{
        self.{}
            .call((msg, "{}".to_string(), "{}".to_string()))
    }}
"#,
                method, req, resp, wrap, service_name, method
            );
    }

    gen = gen + "}\n";

    Ok(gen)
}
