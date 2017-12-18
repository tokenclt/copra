// This file is generated, Do not edit
// @generated

#![allow(missing_docs)]
#![allow(dead_code)]

pub trait EchoService {
    type EchoFuture: ::futures::Future<
        Item = (super::echo::EchoResponse, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    type RevEchoFuture: ::futures::Future<
        Item = (super::echo::EchoResponse, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    fn echo(&self, msg: (super::echo::EchoRequest, ::caper::controller::Controller)) -> Self::EchoFuture;


    fn rev_echo(&self, msg: (super::echo::EchoRequest, ::caper::controller::Controller)) -> Self::RevEchoFuture;

}


pub struct EchoRegistrant<S> {
    provider: S,
}

impl<S> EchoRegistrant<S> {
    pub fn new(provider: S) -> Self {
        EchoRegistrant { provider }
    }
}

    

impl<S> ::caper::dispatcher::Registrant for EchoRegistrant<S>
where
    S: EchoService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::caper::service::NewEncapService)> {
        let mut entries = Vec::new();
        let provider = &self.provider;
    
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::caper::service::Service for Wrapper<S>
            where
                S: EchoService + Clone,
            {
                type Request = (super::echo::EchoRequest, ::caper::controller::Controller);
                type Response = (super::echo::EchoResponse, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as EchoService>::EchoFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.echo(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::caper::service::EncapsulatedMethod::new(
                ::caper::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::caper::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "echo".to_string(), 
                Box::new(new_method) as ::caper::service::NewEncapService,
            ));
        }
        
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::caper::service::Service for Wrapper<S>
            where
                S: EchoService + Clone,
            {
                type Request = (super::echo::EchoRequest, ::caper::controller::Controller);
                type Response = (super::echo::EchoResponse, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as EchoService>::RevEchoFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.rev_echo(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::caper::service::EncapsulatedMethod::new(
                ::caper::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::caper::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "rev_echo".to_string(), 
                Box::new(new_method) as ::caper::service::NewEncapService,
            ));
        }
        
        entries
    }
}
    

#[derive(Clone)]
pub struct EchoStub<'a> {
    echo_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::echo::EchoResponse, super::echo::EchoRequest>>,

    rev_echo_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::echo::EchoResponse, super::echo::EchoRequest>>,

}

impl<'a> EchoStub<'a> {
    pub fn new(channel: &'a ::caper::channel::Channel) -> Self {
        EchoStub {
    
            echo_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

            rev_echo_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

        }
    }
    pub fn echo(&'a self, msg: super::echo::EchoRequest) -> ::caper::stub::StubCallFuture<'a, super::echo::EchoResponse> {
        self.echo_wrapper
            .call((msg, "Echo".to_string(), "echo".to_string()))
    }

    pub fn rev_echo(&'a self, msg: super::echo::EchoRequest) -> ::caper::stub::StubCallFuture<'a, super::echo::EchoResponse> {
        self.rev_echo_wrapper
            .call((msg, "Echo".to_string(), "rev_echo".to_string()))
    }
}
