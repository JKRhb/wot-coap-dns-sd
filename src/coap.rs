// SPDX-License-Identifier: MIT

use coap::Server;
use coap_lite::{
    CoapRequest, CoapResponse, ContentFormat, MessageClass, RequestType as Method, ResponseType,
};
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use wot_td::Thing;

pub(crate) fn create_coap_server() {
    let addr = "0.0.0.0:5683";

    Runtime::new()
        .expect("Runtime should be created")
        .block_on(async move {
            let mut server = Server::new(addr).expect("Server should be up");

            println!("CoAP server running on {}", addr);

            server
                .run(|request| async { handle_coap_request(request) })
                .await
                .expect("Server should wait for incoming requests");
        });
}

fn handle_coap_request(request: CoapRequest<SocketAddr>) -> Option<CoapResponse> {
    match request.get_method() {
        Method::Get => handle_get_request(request),
        _ => create_response(request, ResponseType::MethodNotAllowed, None, None),
    }
}

fn handle_get_request(request: CoapRequest<SocketAddr>) -> Option<CoapResponse> {
    match request.get_path().as_str() {
        ".well-known/wot" => handle_td_request(request),
        _ => create_response(request, ResponseType::NotFound, None, None),
    }
}

fn create_td_payload() -> Vec<u8> {
    // TODO: TD could be more interesting

    let thing_description = Thing::builder("Example Thing")
        .security(|builder| builder.no_sec().with_key("nosec_sc").required())
        .build()
        .expect("TD should be valid");

    serde_json::to_vec(&thing_description).expect("Serialization should be successful")
}

fn handle_td_request(request: CoapRequest<SocketAddr>) -> Option<CoapResponse> {
    let payload = create_td_payload();

    create_response(
        request,
        ResponseType::Content,
        Some(payload),
        // TODO: Replace with application/td+json
        Some(ContentFormat::ApplicationJSON),
    )
}

fn create_response(
    request: CoapRequest<SocketAddr>,
    response_type: ResponseType,
    payload: Option<Vec<u8>>,
    content_format: Option<ContentFormat>,
) -> Option<CoapResponse> {
    match request.response {
        Some(mut message) => {
            message.message.header.code = MessageClass::Response(response_type);

            if let Some(payload) = payload {
                message.message.payload = payload;
            }

            if let Some(content_format) = content_format {
                message.message.set_content_format(content_format);
            }

            Some(message)
        }
        _ => None,
    }
}
