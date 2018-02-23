// This file is generated, Do not edit
// @generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub trait EchoService {
    type EchoFuture: ::futures::Future<
        Item = (super::echo::EchoResponse, ::copra::controller::Controller), 
        Error = ::copra::service::MethodError,
    > + 'static;

    type RevEchoFuture: ::futures::Future<
        Item = (super::echo::EchoResponse, ::copra::controller::Controller), 
        Error = ::copra::service::MethodError,
    > + 'static;

    fn echo(&self, msg: (super::echo::EchoRequest, ::copra::controller::Controller)) -> Self::EchoFuture;

    fn rev_echo(&self, msg: (super::echo::EchoRequest, ::copra::controller::Controller)) -> Self::RevEchoFuture;
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
                type Request = (super::echo::EchoRequest, ::copra::controller::Controller);
                type Response = (super::echo::EchoResponse, ::copra::controller::Controller);
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
        
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: EchoService + Clone,
            {
                type Request = (super::echo::EchoRequest, ::copra::controller::Controller);
                type Response = (super::echo::EchoResponse, ::copra::controller::Controller);
                type Error = ::copra::service::MethodError;
                type Future = <S as EchoService>::RevEchoFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.rev_echo(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::EncapsulatedMethod::new(
                ::copra::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::copra::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "rev_echo".to_string(), 
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
        ::copra::codec::ProtobufCodec<super::echo::EchoResponse, super::echo::EchoRequest>>,

    rev_echo_wrapper: ::copra::stub::RpcWrapper<'a,
        ::copra::codec::ProtobufCodec<super::echo::EchoResponse, super::echo::EchoRequest>>,
}

impl<'a> EchoStub<'a> {
    pub fn new(channel: &'a ::copra::channel::Channel) -> Self {
        EchoStub {
            echo_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(), channel
            ),

            rev_echo_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(), channel
            ),
        }
    }

    pub fn echo(
        &'a self, 
        msg: super::echo::EchoRequest,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::echo::EchoResponse,
            super::echo::EchoRequest,
        >,
    > {
        self.echo_wrapper
            .call((msg, "Echo".to_string(), "echo".to_string()))
    }

    pub fn rev_echo(
        &'a self, 
        msg: super::echo::EchoRequest,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::echo::EchoResponse,
            super::echo::EchoRequest,
        >,
    > {
        self.rev_echo_wrapper
            .call((msg, "Echo".to_string(), "rev_echo".to_string()))
    }
}
