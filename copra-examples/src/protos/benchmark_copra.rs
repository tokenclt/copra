// This file is generated, Do not edit
// @generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub trait MetricService {
    type MetricFuture: ::futures::Future<
        Item = (super::benchmark::Empty, ::copra::controller::Controller), 
        Error = ::copra::service::ProviderSetError,
    > 
        + 'static;

    fn metric(
        &self, 
        msg: (super::benchmark::Empty, ::copra::controller::Controller)
    ) -> Self::MetricFuture;
}

pub struct MetricRegistrant<S> {
    provider: S,
}

impl<S> MetricRegistrant<S> {
    pub fn new(provider: S) -> Self {
        MetricRegistrant { provider }
    }
}

impl<S> ::copra::dispatcher::Registrant for MetricRegistrant<S>
where
    S: MetricService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::copra::service::NewUnifiedMethod)> {
        let mut entries = Vec::new();
        let provider = &self.provider;

        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: MetricService + Clone,
            {
                type Request = (super::benchmark::Empty, ::copra::controller::Controller);
                type Response = (super::benchmark::Empty, ::copra::controller::Controller);
                type Error = ::copra::service::ProviderSetError;
                type Future = <S as MetricService>::MetricFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.metric(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::CodecMethodBundle::new(
                ::copra::codec::ProtobufCodec::new(), 
                wrap
            );
            let new_method = ::copra::service::NewUnifiedMethod::new(method);
            entries.push(("metric".to_string(), new_method));
        }

        entries
    }
}

impl<S> ::copra::dispatcher::NamedRegistrant for MetricRegistrant<S> 
where 
    S: MetricService + Clone + Send + Sync + 'static,
{
    fn name() -> &'static str {
        "Metric"
    }
}

#[derive(Clone)]
pub struct MetricStub<'a> {
    metric_wrapper: ::copra::stub::RpcWrapper<
        'a,
        ::copra::codec::ProtobufCodec<super::benchmark::Empty, super::benchmark::Empty>,
    >,
}

impl<'a> MetricStub<'a> {
    pub fn new(channel: &'a ::copra::channel::Channel) -> Self {
        MetricStub {
            metric_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(),
                channel
            ),
        }
    }

    pub fn metric(
        &'a self, 
        msg: super::benchmark::Empty,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::benchmark::Empty,
            super::benchmark::Empty,
        >,
    > {
        self.metric_wrapper
            .call((msg, "Metric".to_string(), "metric".to_string()))
    }
}

pub trait PressureService {
    type EchoFuture: ::futures::Future<
        Item = (super::benchmark::StringMessage, ::copra::controller::Controller), 
        Error = ::copra::service::ProviderSetError,
    > 
        + 'static;

    type ProcessFuture: ::futures::Future<
        Item = (super::benchmark::Empty, ::copra::controller::Controller), 
        Error = ::copra::service::ProviderSetError,
    > 
        + 'static;

    fn echo(
        &self, 
        msg: (super::benchmark::StringMessage, ::copra::controller::Controller)
    ) -> Self::EchoFuture;

    fn process(
        &self, 
        msg: (super::benchmark::PressureRequest, ::copra::controller::Controller)
    ) -> Self::ProcessFuture;
}

pub struct PressureRegistrant<S> {
    provider: S,
}

impl<S> PressureRegistrant<S> {
    pub fn new(provider: S) -> Self {
        PressureRegistrant { provider }
    }
}

impl<S> ::copra::dispatcher::Registrant for PressureRegistrant<S>
where
    S: PressureService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::copra::service::NewUnifiedMethod)> {
        let mut entries = Vec::new();
        let provider = &self.provider;

        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: PressureService + Clone,
            {
                type Request = (super::benchmark::StringMessage, ::copra::controller::Controller);
                type Response = (super::benchmark::StringMessage, ::copra::controller::Controller);
                type Error = ::copra::service::ProviderSetError;
                type Future = <S as PressureService>::EchoFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.echo(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::CodecMethodBundle::new(
                ::copra::codec::ProtobufCodec::new(), 
                wrap
            );
            let new_method = ::copra::service::NewUnifiedMethod::new(method);
            entries.push(("echo".to_string(), new_method));
        }

        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: PressureService + Clone,
            {
                type Request = (super::benchmark::PressureRequest, ::copra::controller::Controller);
                type Response = (super::benchmark::Empty, ::copra::controller::Controller);
                type Error = ::copra::service::ProviderSetError;
                type Future = <S as PressureService>::ProcessFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.process(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::CodecMethodBundle::new(
                ::copra::codec::ProtobufCodec::new(), 
                wrap
            );
            let new_method = ::copra::service::NewUnifiedMethod::new(method);
            entries.push(("process".to_string(), new_method));
        }

        entries
    }
}

impl<S> ::copra::dispatcher::NamedRegistrant for PressureRegistrant<S> 
where 
    S: PressureService + Clone + Send + Sync + 'static,
{
    fn name() -> &'static str {
        "Pressure"
    }
}

#[derive(Clone)]
pub struct PressureStub<'a> {
    echo_wrapper: ::copra::stub::RpcWrapper<
        'a,
        ::copra::codec::ProtobufCodec<super::benchmark::StringMessage, super::benchmark::StringMessage>,
    >,

    process_wrapper: ::copra::stub::RpcWrapper<
        'a,
        ::copra::codec::ProtobufCodec<super::benchmark::Empty, super::benchmark::PressureRequest>,
    >,
}

impl<'a> PressureStub<'a> {
    pub fn new(channel: &'a ::copra::channel::Channel) -> Self {
        PressureStub {
            echo_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(),
                channel
            ),

            process_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(),
                channel
            ),
        }
    }

    pub fn echo(
        &'a self, 
        msg: super::benchmark::StringMessage,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::benchmark::StringMessage,
            super::benchmark::StringMessage,
        >,
    > {
        self.echo_wrapper
            .call((msg, "Pressure".to_string(), "echo".to_string()))
    }

    pub fn process(
        &'a self, 
        msg: super::benchmark::PressureRequest,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::benchmark::Empty,
            super::benchmark::PressureRequest,
        >,
    > {
        self.process_wrapper
            .call((msg, "Pressure".to_string(), "process".to_string()))
    }
}
