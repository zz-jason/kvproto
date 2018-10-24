// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

const METHOD_ENGINE_APPLY_COMMAND: ::grpcio::Method<super::enginepb::CommandRequet, super::enginepb::CommandResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/enginepb.Engine/ApplyCommand",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ENGINE_APPLY_SNAPSHOT: ::grpcio::Method<super::enginepb::SnapshotRequest, super::enginepb::SnapshotDone> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/enginepb.Engine/ApplySnapshot",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct EngineClient {
    client: ::grpcio::Client,
}

impl EngineClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        EngineClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn apply_command_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::enginepb::CommandRequet>, ::grpcio::ClientDuplexReceiver<super::enginepb::CommandResponse>)> {
        self.client.duplex_streaming(&METHOD_ENGINE_APPLY_COMMAND, opt)
    }

    pub fn apply_command(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::enginepb::CommandRequet>, ::grpcio::ClientDuplexReceiver<super::enginepb::CommandResponse>)> {
        self.apply_command_opt(::grpcio::CallOption::default())
    }

    pub fn apply_snapshot_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::enginepb::SnapshotRequest>, ::grpcio::ClientCStreamReceiver<super::enginepb::SnapshotDone>)> {
        self.client.client_streaming(&METHOD_ENGINE_APPLY_SNAPSHOT, opt)
    }

    pub fn apply_snapshot(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::enginepb::SnapshotRequest>, ::grpcio::ClientCStreamReceiver<super::enginepb::SnapshotDone>)> {
        self.apply_snapshot_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Engine {
    fn apply_command(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::enginepb::CommandRequet>, sink: ::grpcio::DuplexSink<super::enginepb::CommandResponse>);
    fn apply_snapshot(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::enginepb::SnapshotRequest>, sink: ::grpcio::ClientStreamingSink<super::enginepb::SnapshotDone>);
}

pub fn create_engine<S: Engine + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ENGINE_APPLY_COMMAND, move |ctx, req, resp| {
        instance.apply_command(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_ENGINE_APPLY_SNAPSHOT, move |ctx, req, resp| {
        instance.apply_snapshot(ctx, req, resp)
    });
    builder.build()
}
