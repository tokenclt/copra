// This file is generated, Do not edit
// @generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub trait DemoService {
    type GreetToFuture: ::futures::Future<
        Item = (super::demo::GreetMessage, ::copra::controller::Controller), 
        Error = ::copra::service::ProviderSetError,
    > 
        + 'static;

    type IsPrimeFuture: ::futures::Future<
        Item = (super::demo::PrimeResponse, ::copra::controller::Controller), 
        Error = ::copra::service::ProviderSetError,
    > 
        + 'static;

    fn greet_to(
        &self, 
        msg: (super::demo::GreetMessage, ::copra::controller::Controller)
    ) -> Self::GreetToFuture;

    fn is_prime(
        &self, 
        msg: (super::demo::PrimeRequest, ::copra::controller::Controller)
    ) -> Self::IsPrimeFuture;
}

pub struct DemoRegistrant<S> {
    provider: S,
}

impl<S> DemoRegistrant<S> {
    pub fn new(provider: S) -> Self {
        DemoRegistrant { provider }
    }
}

impl<S> ::copra::dispatcher::Registrant for DemoRegistrant<S>
where
    S: DemoService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::copra::service::NewUnifiedMethod)> {
        let mut entries = Vec::new();
        let provider = &self.provider;

        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: DemoService + Clone,
            {
                type Request = (super::demo::GreetMessage, ::copra::controller::Controller);
                type Response = (super::demo::GreetMessage, ::copra::controller::Controller);
                type Error = ::copra::service::ProviderSetError;
                type Future = <S as DemoService>::GreetToFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.greet_to(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::CodecMethodBundle::new(
                ::copra::codec::ProtobufCodec::new(), 
                wrap
            );
            let new_method = ::copra::service::NewUnifiedMethod::new(method);
            entries.push(("greet_to".to_string(), new_method));
        }

        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: DemoService + Clone,
            {
                type Request = (super::demo::PrimeRequest, ::copra::controller::Controller);
                type Response = (super::demo::PrimeResponse, ::copra::controller::Controller);
                type Error = ::copra::service::ProviderSetError;
                type Future = <S as DemoService>::IsPrimeFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.is_prime(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::CodecMethodBundle::new(
                ::copra::codec::ProtobufCodec::new(), 
                wrap
            );
            let new_method = ::copra::service::NewUnifiedMethod::new(method);
            entries.push(("is_prime".to_string(), new_method));
        }

        entries
    }
}

impl<S> ::copra::dispatcher::NamedRegistrant for DemoRegistrant<S> 
where 
    S: DemoService + Clone + Send + Sync + 'static,
{
    fn name() -> &'static str {
        "Demo"
    }
}

#[derive(Clone)]
pub struct DemoStub<'a> {
    greet_to_wrapper: ::copra::stub::RpcWrapper<
        'a,
        ::copra::codec::ProtobufCodec<super::demo::GreetMessage, super::demo::GreetMessage>,
    >,

    is_prime_wrapper: ::copra::stub::RpcWrapper<
        'a,
        ::copra::codec::ProtobufCodec<super::demo::PrimeResponse, super::demo::PrimeRequest>,
    >,
}

impl<'a> DemoStub<'a> {
    pub fn new(channel: &'a ::copra::channel::Channel) -> Self {
        DemoStub {
            greet_to_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(),
                channel
            ),

            is_prime_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(),
                channel
            ),
        }
    }

    pub fn greet_to(
        &'a self, 
        msg: super::demo::GreetMessage,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
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
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::demo::PrimeResponse,
            super::demo::PrimeRequest,
        >,
    > {
        self.is_prime_wrapper
            .call((msg, "Demo".to_string(), "is_prime".to_string()))
    }
}
