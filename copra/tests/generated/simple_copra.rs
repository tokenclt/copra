// This file is generated, Do not edit
// @generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub trait EchoService {
    type EchoFuture: ::futures::Future<
        Item = (super::simple::Simple, ::copra::controller::Controller), 
        Error = ::copra::service::MethodError,
    > + 'static;

    fn echo(&self, msg: (super::simple::Simple, ::copra::controller::Controller)) -> Self::EchoFuture;
}

pub struct EchoRegistrant<S> {
    provider: S,
}

impl<S> EchoRegistrant<S> {
    pub fn new(provider: S) -> Self {
        EchoRegistrant { provider }
    }
}

impl<S> ::copra::dispatcher::Registrant for EchoRegistrant<S>
where
    S: EchoService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::copra::service::NewEncapService)> {
        let mut entries = Vec::new();
        let provider = &self.provider;
    
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: EchoService + Clone,
            {
                type Request = (super::simple::Simple, ::copra::controller::Controller);
                type Response = (super::simple::Simple, ::copra::controller::Controller);
                type Error = ::copra::service::MethodError;
                type Future = <S as EchoService>::EchoFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.echo(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::EncapsulatedMethod::new(
                ::copra::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::copra::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "echo".to_string(), 
                Box::new(new_method) as ::copra::service::NewEncapService,
            ));
        }
        
        entries
    }
}

impl<S> ::copra::dispatcher::NamedRegistrant for EchoRegistrant<S> 
where 
    S: EchoService + Clone + Send + Sync + 'static,
{
    fn name() -> &'static str {
        "Echo"
    }
}

#[derive(Clone)]
pub struct EchoStub<'a> {
    echo_wrapper: ::copra::stub::RpcWrapper<'a,
        ::copra::codec::ProtobufCodec<super::simple::Simple, super::simple::Simple>>,
}

impl<'a> EchoStub<'a> {
    pub fn new(channel: &'a ::copra::channel::Channel) -> Self {
        EchoStub {
            echo_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(), channel
            ),
        }
    }

    pub fn echo(
        &'a self, 
        msg: super::simple::Simple,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::simple::Simple,
            super::simple::Simple,
        >,
    > {
        self.echo_wrapper
            .call((msg, "Echo".to_string(), "echo".to_string()))
    }
}
