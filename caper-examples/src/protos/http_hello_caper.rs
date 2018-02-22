// This file is generated, Do not edit
// @generated

#![allow(missing_docs)]
#![allow(dead_code)]

pub trait HelloService {
    type HelloGeneralFuture: ::futures::Future<
        Item = (super::http_hello::HelloResponse, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    type HelloToFuture: ::futures::Future<
        Item = (super::http_hello::HelloResponse, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    fn hello_general(&self, msg: (super::http_hello::HelloRequest, ::caper::controller::Controller)) -> Self::HelloGeneralFuture;


    fn hello_to(&self, msg: (super::http_hello::HelloRequest, ::caper::controller::Controller)) -> Self::HelloToFuture;

}


pub struct HelloRegistrant<S> {
    provider: S,
}

impl<S> HelloRegistrant<S> {
    pub fn new(provider: S) -> Self {
        HelloRegistrant { provider }
    }
}

    

impl<S> ::caper::dispatcher::Registrant for HelloRegistrant<S>
where
    S: HelloService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::caper::service::NewEncapService)> {
        let mut entries = Vec::new();
        let provider = &self.provider;
    
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::caper::service::Service for Wrapper<S>
            where
                S: HelloService + Clone,
            {
                type Request = (super::http_hello::HelloRequest, ::caper::controller::Controller);
                type Response = (super::http_hello::HelloResponse, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as HelloService>::HelloGeneralFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.hello_general(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::caper::service::EncapsulatedMethod::new(
                ::caper::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::caper::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "hello_general".to_string(), 
                Box::new(new_method) as ::caper::service::NewEncapService,
            ));
        }
        
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::caper::service::Service for Wrapper<S>
            where
                S: HelloService + Clone,
            {
                type Request = (super::http_hello::HelloRequest, ::caper::controller::Controller);
                type Response = (super::http_hello::HelloResponse, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as HelloService>::HelloToFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.hello_to(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::caper::service::EncapsulatedMethod::new(
                ::caper::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::caper::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "hello_to".to_string(), 
                Box::new(new_method) as ::caper::service::NewEncapService,
            ));
        }
        
        entries
    }
}

impl<S> ::caper::dispatcher::NamedRegistrant for HelloRegistrant<S> 
where 
    S: HelloService + Clone + Send + Sync + 'static,
{
    fn name() -> &'static str {
        "Hello"
    }
}


#[derive(Clone)]
pub struct HelloStub<'a> {
    hello_general_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::http_hello::HelloResponse, super::http_hello::HelloRequest>>,

    hello_to_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::http_hello::HelloResponse, super::http_hello::HelloRequest>>,

}

impl<'a> HelloStub<'a> {
    pub fn new(channel: &'a ::caper::channel::Channel) -> Self {
        HelloStub {
    
            hello_general_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

            hello_to_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

        }
    }
    pub fn hello_general(
        &'a self, 
        msg: super::http_hello::HelloRequest,
    ) -> ::caper::stub::StubFuture<
        ::caper::codec::ProtobufCodec<
            super::http_hello::HelloResponse,
            super::http_hello::HelloRequest,
        >,
    > {
        self.hello_general_wrapper
            .call((msg, "Hello".to_string(), "hello_general".to_string()))
    }

    pub fn hello_to(
        &'a self, 
        msg: super::http_hello::HelloRequest,
    ) -> ::caper::stub::StubFuture<
        ::caper::codec::ProtobufCodec<
            super::http_hello::HelloResponse,
            super::http_hello::HelloRequest,
        >,
    > {
        self.hello_to_wrapper
            .call((msg, "Hello".to_string(), "hello_to".to_string()))
    }
}
