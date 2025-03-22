// SPDX-License-Identifier: MIT

use coap::Server;
use coap_lite::{CoapRequest, ContentFormat, MessageClass, RequestType as Method, ResponseType};
use local_ip_address::local_ip;
use rand::Rng;
use std::net::SocketAddr;
use std::sync::LazyLock;
use tokio::runtime::Runtime;
use uuid::Uuid;
use wot_td::builder::{
    BuildableHumanReadableInfo, BuildableInteractionAffordance, SpecializableDataSchema,
};
use wot_td::Thing;

static TD_UUID: LazyLock<uuid::Uuid> = LazyLock::new(Uuid::new_v4);

pub(crate) fn create_coap_server() {
    let addr = "0.0.0.0:5683";

    Runtime::new()
        .expect("Runtime should be created")
        .block_on(async move {
            let server = Server::new_udp(addr).expect("Server should be up");

            println!("CoAP server running on {}", addr);

            server
                .run(|request| async move { handle_coap_request(request) })
                .await
                .expect("Server should wait for incoming requests");
        });
}

fn handle_coap_request(request: Box<CoapRequest<SocketAddr>>) -> Box<CoapRequest<SocketAddr>> {
    match request.get_method() {
        Method::Get => handle_get_request(request),
        _ => create_response(request, ResponseType::MethodNotAllowed, None, None),
    }
}

fn handle_get_request(request: Box<CoapRequest<SocketAddr>>) -> Box<CoapRequest<SocketAddr>> {
    match request.get_path().as_str() {
        ".well-known/wot" => handle_td_request(request),
        "temperature" => handle_temperature_request(request),
        _ => create_response(request, ResponseType::NotFound, None, None),
    }
}

fn create_td_payload() -> Vec<u8> {
    // TODO: TD could be more interesting
    let local_ip_address = local_ip().expect("Should determine the local IP address.");

    println!("{:}", local_ip_address);

    let thing_description = Thing::builder("Example Thing")
        .security(|builder| builder.no_sec().with_key("nosec_sc").required())
        .id(TD_UUID.urn().to_string())
        .base("coap://127.0.0.1")
        .finish_extend()
        .property("temperature", |property_builder| {
            property_builder
                .title("Temperature")
                .finish_extend_data_schema()
                .integer()
                .form(|form_builder| form_builder.href("/temperature"))
        })
        .build()
        .expect("TD should be valid");

    serde_json::to_vec(&thing_description).expect("Serialization should be successful")
}

fn handle_td_request(request: Box<CoapRequest<SocketAddr>>) -> Box<CoapRequest<SocketAddr>> {
    let payload = create_td_payload();

    create_response(
        request,
        ResponseType::Content,
        Some(payload),
        Some(ContentFormat::ApplicationTdJson),
    )
}

fn handle_temperature_request(
    request: Box<CoapRequest<SocketAddr>>,
) -> Box<CoapRequest<SocketAddr>> {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let temperature: u32 = rng.gen_range(20..25);

    create_response(
        request,
        ResponseType::Content,
        Some(temperature.to_string().into_bytes()),
        Some(ContentFormat::ApplicationJSON),
    )
}

fn create_response(
    mut request: Box<CoapRequest<SocketAddr>>,
    response_type: ResponseType,
    payload: Option<Vec<u8>>,
    content_format: Option<ContentFormat>,
) -> Box<CoapRequest<SocketAddr>> {
    match request.response {
        Some(ref mut message) => {
            message.message.header.code = MessageClass::Response(response_type);

            if let Some(payload) = payload {
                message.message.payload = payload;
            }

            if let Some(content_format) = content_format {
                message.message.set_content_format(content_format);
            }

            request
        }
        _ => request,
    }
}
