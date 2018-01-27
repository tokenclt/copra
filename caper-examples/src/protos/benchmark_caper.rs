// This file is generated, Do not edit
// @generated

#![allow(missing_docs)]
#![allow(dead_code)]

pub trait MetricService {
    type MetricFuture: ::futures::Future<
        Item = (super::benchmark::Empty, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    fn metric(&self, msg: (super::benchmark::Empty, ::caper::controller::Controller)) -> Self::MetricFuture;

}


pub struct MetricRegistrant<S> {
    provider: S,
}

impl<S> MetricRegistrant<S> {
    pub fn new(provider: S) -> Self {
        MetricRegistrant { provider }
    }
}

    

impl<S> ::caper::dispatcher::Registrant for MetricRegistrant<S>
where
    S: MetricService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::caper::service::NewEncapService)> {
        let mut entries = Vec::new();
        let provider = &self.provider;
    
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::caper::service::Service for Wrapper<S>
            where
                S: MetricService + Clone,
            {
                type Request = (super::benchmark::Empty, ::caper::controller::Controller);
                type Response = (super::benchmark::Empty, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as MetricService>::MetricFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.metric(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::caper::service::EncapsulatedMethod::new(
                ::caper::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::caper::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "metric".to_string(), 
                Box::new(new_method) as ::caper::service::NewEncapService,
            ));
        }
        
        entries
    }
}
    

#[derive(Clone)]
pub struct MetricStub<'a> {
    metric_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::benchmark::Empty, super::benchmark::Empty>>,

}

impl<'a> MetricStub<'a> {
    pub fn new(channel: &'a ::caper::channel::Channel) -> Self {
        MetricStub {
    
            metric_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

        }
    }
    pub fn metric(
        &'a self, 
        msg: super::benchmark::Empty,
    ) -> ::caper::stub::StubFuture<
        ::caper::codec::ProtobufCodec<
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
        Item = (super::benchmark::StringMessage, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    type ProcessFuture: ::futures::Future<
        Item = (super::benchmark::Empty, ::caper::controller::Controller), 
        Error = ::caper::service::MethodError,
    > + 'static;


    fn echo(&self, msg: (super::benchmark::StringMessage, ::caper::controller::Controller)) -> Self::EchoFuture;


    fn process(&self, msg: (super::benchmark::PressureRequest, ::caper::controller::Controller)) -> Self::ProcessFuture;

}


pub struct PressureRegistrant<S> {
    provider: S,
}

impl<S> PressureRegistrant<S> {
    pub fn new(provider: S) -> Self {
        PressureRegistrant { provider }
    }
}

    

impl<S> ::caper::dispatcher::Registrant for PressureRegistrant<S>
where
    S: PressureService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::caper::service::NewEncapService)> {
        let mut entries = Vec::new();
        let provider = &self.provider;
    
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::caper::service::Service for Wrapper<S>
            where
                S: PressureService + Clone,
            {
                type Request = (super::benchmark::StringMessage, ::caper::controller::Controller);
                type Response = (super::benchmark::StringMessage, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as PressureService>::EchoFuture;

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
                S: PressureService + Clone,
            {
                type Request = (super::benchmark::PressureRequest, ::caper::controller::Controller);
                type Response = (super::benchmark::Empty, ::caper::controller::Controller);
                type Error = ::caper::service::MethodError;
                type Future = <S as PressureService>::ProcessFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.process(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::caper::service::EncapsulatedMethod::new(
                ::caper::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::caper::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "process".to_string(), 
                Box::new(new_method) as ::caper::service::NewEncapService,
            ));
        }
        
        entries
    }
}
    

#[derive(Clone)]
pub struct PressureStub<'a> {
    echo_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::benchmark::StringMessage, super::benchmark::StringMessage>>,

    process_wrapper: ::caper::stub::RpcWrapper<'a,
        ::caper::codec::ProtobufCodec<super::benchmark::Empty, super::benchmark::PressureRequest>>,

}

impl<'a> PressureStub<'a> {
    pub fn new(channel: &'a ::caper::channel::Channel) -> Self {
        PressureStub {
    
            echo_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

            process_wrapper: ::caper::stub::RpcWrapper::new(
                ::caper::codec::ProtobufCodec::new(), channel
            ),

        }
    }
    pub fn echo(
        &'a self, 
        msg: super::benchmark::StringMessage,
    ) -> ::caper::stub::StubFuture<
        ::caper::codec::ProtobufCodec<
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
    ) -> ::caper::stub::StubFuture<
        ::caper::codec::ProtobufCodec<
            super::benchmark::Empty,
            super::benchmark::PressureRequest,
        >,
    > {
        self.process_wrapper
            .call((msg, "Pressure".to_string(), "process".to_string()))
    }
}
