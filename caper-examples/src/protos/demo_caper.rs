// This file is generated, Do not edit
// @generated

#![allow(missing_docs)]
#![allow(dead_code)]

pub trait DemoService {
    type GreetToFuture: ::futures::Future<
        Item = (super::demo::GreetMessage, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    type IsPrimeFuture: ::futures::Future<
        Item = (super::demo::PrimeResponse, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    fn greet_to(&self, msg: (super::demo::GreetMessage, ::caper::controller::Controller)) -> Self::GreetToFuture;


    fn is_prime(&self, msg: (super::demo::PrimeRequest, ::caper::controller::Controller)) -> Self::IsPrimeFuture;

}


pub struct DemoRegistrant<S> {
    provider: S,
}

impl<S> DemoRegistrant<S> {
    pub fn new(provider: S) -> Self {
        DemoRegistrant { provider }
    }
}

    

impl<S> ::caper::dispatcher::Registrant for DemoRegistrant<S>
where
    S: DemoService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::caper::service::NewEncapService)> {
        let mut entries = Vec::new();
        let provider = &self.provider;
    
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::caper::service::Service for Wrapper<S>
            where
                S: DemoService + Clone,
            {
                type Request = (super::demo::GreetMessage, ::caper::controller::Controller);
                type Response = (super::demo::GreetMessage, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as DemoService>::GreetToFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.greet_to(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::caper::service::EncapsulatedMethod::new(
                ::caper::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::caper::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "greet_to".to_string(), 
                Box::new(new_method) as ::caper::service::NewEncapService,
            ));
        }
        
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::caper::service::Service for Wrapper<S>
            where
                S: DemoService + Clone,
            {
                type Request = (super::demo::PrimeRequest, ::caper::controller::Controller);
                type Response = (super::demo::PrimeResponse, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as DemoService>::IsPrimeFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.is_prime(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::caper::service::EncapsulatedMethod::new(
                ::caper::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::caper::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "is_prime".to_string(), 
                Box::new(new_method) as ::caper::service::NewEncapService,
            ));
        }
        
        entries
    }
}
    

#[derive(Clone)]
pub struct DemoStub<'a> {
    greet_to_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::demo::GreetMessage, super::demo::GreetMessage>>,

    is_prime_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::demo::PrimeResponse, super::demo::PrimeRequest>>,

}

impl<'a> DemoStub<'a> {
    pub fn new(channel: &'a ::caper::channel::Channel) -> Self {
        DemoStub {
    
            greet_to_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

            is_prime_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

        }
    }
    pub fn greet_to(
        &'a self, 
        msg: super::demo::GreetMessage,
    ) -> ::caper::stub::StubFuture<
        ::caper::codec::ProtobufCodec<
            super::demo::GreetMessage,
            super::demo::GreetMessage,
        >,
    > {
        self.greet_to_wrapper
            .call((msg, "Demo".to_string(), "greet_to".to_string()))
    }

    pub fn is_prime(
        &'a self, 
        msg: super::demo::PrimeRequest,
    ) -> ::caper::stub::StubFuture<
        ::caper::codec::ProtobufCodec<
            super::demo::PrimeResponse,
            super::demo::PrimeRequest,
        >,
    > {
        self.is_prime_wrapper
            .call((msg, "Demo".to_string(), "is_prime".to_string()))
    }
}
