// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// server interface

pub trait Registration {
    fn register_discovery_handler(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::discovery::RegisterDiscoveryHandlerRequest>, resp: ::grpc::ServerResponseUnarySink<super::discovery::Empty>) -> ::grpc::Result<()>;
}

// client

pub struct RegistrationClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for RegistrationClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        RegistrationClient {
            grpc_client: grpc_client,
        }
    }
}

impl RegistrationClient {
    pub fn register_discovery_handler(&self, o: ::grpc::RequestOptions, req: super::discovery::RegisterDiscoveryHandlerRequest) -> ::grpc::SingleResponse<super::discovery::Empty> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/v0.Registration/RegisterDiscoveryHandler"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct RegistrationServer;


impl RegistrationServer {
    pub fn new_service_def<H : Registration + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/v0.Registration",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/v0.Registration/RegisterDiscoveryHandler"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).register_discovery_handler(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait DiscoveryHandler {
    fn discover(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::discovery::DiscoverRequest>, resp: ::grpc::ServerResponseSink<super::discovery::DiscoverResponse>) -> ::grpc::Result<()>;
}

// client

pub struct DiscoveryHandlerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for DiscoveryHandlerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        DiscoveryHandlerClient {
            grpc_client: grpc_client,
        }
    }
}

impl DiscoveryHandlerClient {
    pub fn discover(&self, o: ::grpc::RequestOptions, req: super::discovery::DiscoverRequest) -> ::grpc::StreamingResponse<super::discovery::DiscoverResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/v0.DiscoveryHandler/Discover"),
            streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_server_streaming(o, req, descriptor)
    }
}

// server

pub struct DiscoveryHandlerServer;


impl DiscoveryHandlerServer {
    pub fn new_service_def<H : DiscoveryHandler + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/v0.DiscoveryHandler",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/v0.DiscoveryHandler/Discover"),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |ctx, req, resp| (*handler_copy).discover(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
