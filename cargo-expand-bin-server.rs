#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tonic::{transport::Server, Request, Response, Status};
use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};
pub mod hello {
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct HelloRequest {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for HelloRequest {
        #[inline]
        fn clone(&self) -> HelloRequest {
            HelloRequest {
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for HelloRequest {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for HelloRequest {
        #[inline]
        fn eq(&self, other: &HelloRequest) -> bool {
            self.name == other.name
        }
    }
    impl ::prost::Message for HelloRequest {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            if self.name != "" {
                ::prost::encoding::string::encode(1u32, &self.name, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "HelloRequest";
            match tag {
                1u32 => {
                    let mut value = &mut self.name;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "name");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.name != "" {
                    ::prost::encoding::string::encoded_len(1u32, &self.name)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.name.clear();
        }
    }
    impl ::core::default::Default for HelloRequest {
        fn default() -> Self {
            HelloRequest {
                name: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for HelloRequest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("HelloRequest");
            let builder = {
                let wrapper = {
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.name)
                };
                builder.field("name", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct HelloReply {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for HelloReply {
        #[inline]
        fn clone(&self) -> HelloReply {
            HelloReply {
                message: ::core::clone::Clone::clone(&self.message),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for HelloReply {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for HelloReply {
        #[inline]
        fn eq(&self, other: &HelloReply) -> bool {
            self.message == other.message
        }
    }
    impl ::prost::Message for HelloReply {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            if self.message != "" {
                ::prost::encoding::string::encode(1u32, &self.message, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "HelloReply";
            match tag {
                1u32 => {
                    let mut value = &mut self.message;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "message");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.message != "" {
                    ::prost::encoding::string::encoded_len(1u32, &self.message)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.message.clear();
        }
    }
    impl ::core::default::Default for HelloReply {
        fn default() -> Self {
            HelloReply {
                message: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for HelloReply {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("HelloReply");
            let builder = {
                let wrapper = {
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.message)
                };
                builder.field("message", &wrapper)
            };
            builder.finish()
        }
    }
    /// Generated client implementations.
    pub mod greeter_client {
        #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
        use tonic::codegen::*;
        use tonic::codegen::http::Uri;
        pub struct GreeterClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for GreeterClient<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "GreeterClient",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone> ::core::clone::Clone for GreeterClient<T> {
            #[inline]
            fn clone(&self) -> GreeterClient<T> {
                GreeterClient {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        impl GreeterClient<tonic::transport::Channel> {
            /// Attempt to create a new client by connecting to a given endpoint.
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        impl<T> GreeterClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            pub fn with_origin(inner: T, origin: Uri) -> Self {
                let inner = tonic::client::Grpc::with_origin(inner, origin);
                Self { inner }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> GreeterClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<
                            tonic::body::BoxBody,
                        >>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                >>::Error: Into<StdError> + Send + Sync,
            {
                GreeterClient::new(InterceptedService::new(inner, interceptor))
            }
            /// Compress requests with the given encoding.
            ///
            /// This requires the server to support it otherwise it might respond with an
            /// error.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.send_compressed(encoding);
                self
            }
            /// Enable decompressing responses.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.accept_compressed(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_decoding_message_size(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_encoding_message_size(limit);
                self
            }
            pub async fn say_hello(
                &mut self,
                request: impl tonic::IntoRequest<super::HelloRequest>,
            ) -> std::result::Result<tonic::Response<super::HelloReply>, tonic::Status> {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::new(
                            tonic::Code::Unknown,
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Service was not ready: {0}", e.into()),
                                );
                                res
                            }),
                        )
                    })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/hello.Greeter/SayHello",
                );
                let mut req = request.into_request();
                req.extensions_mut()
                    .insert(GrpcMethod::new("hello.Greeter", "SayHello"));
                self.inner.unary(req, path, codec).await
            }
        }
    }
    /// Generated server implementations.
    pub mod greeter_server {
        #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
        use tonic::codegen::*;
        /// Generated trait containing gRPC methods that should be implemented for use with GreeterServer.
        pub trait Greeter: Send + Sync + 'static {
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn say_hello<'life0, 'async_trait>(
                &'life0 self,
                request: tonic::Request<super::HelloRequest>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::HelloReply>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub struct GreeterServer<T: Greeter> {
            inner: _Inner<T>,
            accept_compression_encodings: EnabledCompressionEncodings,
            send_compression_encodings: EnabledCompressionEncodings,
            max_decoding_message_size: Option<usize>,
            max_encoding_message_size: Option<usize>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug + Greeter> ::core::fmt::Debug for GreeterServer<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "GreeterServer",
                    "inner",
                    &self.inner,
                    "accept_compression_encodings",
                    &self.accept_compression_encodings,
                    "send_compression_encodings",
                    &self.send_compression_encodings,
                    "max_decoding_message_size",
                    &self.max_decoding_message_size,
                    "max_encoding_message_size",
                    &&self.max_encoding_message_size,
                )
            }
        }
        struct _Inner<T>(Arc<T>);
        impl<T: Greeter> GreeterServer<T> {
            pub fn new(inner: T) -> Self {
                Self::from_arc(Arc::new(inner))
            }
            pub fn from_arc(inner: Arc<T>) -> Self {
                let inner = _Inner(inner);
                Self {
                    inner,
                    accept_compression_encodings: Default::default(),
                    send_compression_encodings: Default::default(),
                    max_decoding_message_size: None,
                    max_encoding_message_size: None,
                }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
            {
                InterceptedService::new(Self::new(inner), interceptor)
            }
            /// Enable decompressing requests with the given encoding.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.accept_compression_encodings.enable(encoding);
                self
            }
            /// Compress responses with the given encoding, if the client supports it.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.send_compression_encodings.enable(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.max_decoding_message_size = Some(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.max_encoding_message_size = Some(limit);
                self
            }
        }
        impl<T, B> tonic::codegen::Service<http::Request<B>> for GreeterServer<T>
        where
            T: Greeter,
            B: Body + Send + 'static,
            B::Error: Into<StdError> + Send + 'static,
        {
            type Response = http::Response<tonic::body::BoxBody>;
            type Error = std::convert::Infallible;
            type Future = BoxFuture<Self::Response, Self::Error>;
            fn poll_ready(
                &mut self,
                _cx: &mut Context<'_>,
            ) -> Poll<std::result::Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: http::Request<B>) -> Self::Future {
                let inner = self.inner.clone();
                match req.uri().path() {
                    "/hello.Greeter/SayHello" => {
                        #[allow(non_camel_case_types)]
                        struct SayHelloSvc<T: Greeter>(pub Arc<T>);
                        impl<T: Greeter> tonic::server::UnaryService<super::HelloRequest>
                        for SayHelloSvc<T> {
                            type Response = super::HelloReply;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::HelloRequest>,
                            ) -> Self::Future {
                                let inner = Arc::clone(&self.0);
                                let fut = async move { (*inner).say_hello(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self
                            .accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let max_decoding_message_size = self.max_decoding_message_size;
                        let max_encoding_message_size = self.max_encoding_message_size;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = SayHelloSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                )
                                .apply_max_message_size_config(
                                    max_decoding_message_size,
                                    max_encoding_message_size,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    _ => {
                        Box::pin(async move {
                            Ok(
                                http::Response::builder()
                                    .status(200)
                                    .header("grpc-status", "12")
                                    .header("content-type", "application/grpc")
                                    .body(empty_body())
                                    .unwrap(),
                            )
                        })
                    }
                }
            }
        }
        impl<T: Greeter> Clone for GreeterServer<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    accept_compression_encodings: self.accept_compression_encodings,
                    send_compression_encodings: self.send_compression_encodings,
                    max_decoding_message_size: self.max_decoding_message_size,
                    max_encoding_message_size: self.max_encoding_message_size,
                }
            }
        }
        impl<T: Greeter> Clone for _Inner<T> {
            fn clone(&self) -> Self {
                Self(Arc::clone(&self.0))
            }
        }
        impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0:?}", self.0))
            }
        }
        impl<T: Greeter> tonic::server::NamedService for GreeterServer<T> {
            const NAME: &'static str = "hello.Greeter";
        }
    }
}
pub struct MyGreeter {}
#[automatically_derived]
impl ::core::fmt::Debug for MyGreeter {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "MyGreeter")
    }
}
#[automatically_derived]
impl ::core::default::Default for MyGreeter {
    #[inline]
    fn default() -> MyGreeter {
        MyGreeter {}
    }
}
impl Greeter for MyGreeter {
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn say_hello<'life0, 'async_trait>(
        &'life0 self,
        request: Request<HelloRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<Response<HelloReply>, Status>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<Response<HelloReply>, Status>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __self = self;
            let request = request;
            let __ret: Result<Response<HelloReply>, Status> = {
                let name = request.into_inner().name;
                let reply = hello::HelloReply {
                    message: ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("Hello, {0}!", name),
                        );
                        res
                    }),
                };
                Ok(Response::new(reply))
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        let addr = "[::1]:50051".parse()?;
        let greeter = MyGreeter::default();
        {
            ::std::io::_print(format_args!("GreeterServer listening on {0}\n", addr));
        };
        Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await?;
        Ok(())
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
