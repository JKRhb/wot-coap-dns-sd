// SPDX-License-Identifier: MIT

mod coap;
mod mdns;

use crate::coap::create_coap_server;
use mdns::create_mdns_deamon;

fn main() {
    create_mdns_deamon();

    create_coap_server()
}
