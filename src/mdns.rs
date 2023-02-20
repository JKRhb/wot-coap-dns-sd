// SPDX-License-Identifier: MIT

use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;

pub(crate) fn create_mdns_deamon() {
    let mdns = ServiceDaemon::new().expect("Should be able to create the ServiceDaemon");

    let td_service = create_td_service();

    mdns.register(td_service)
        .expect("Should succeed with registering TD discovery service");
}

fn create_td_service() -> ServiceInfo {
    let local_ip_address = local_ip().expect("Should be able to determine local IPv4 address");

    let service_type = "_wot._udp.local.";
    let instance_name = "wot_coap_dns_sd";
    let host_ipv4 = local_ip_address.to_string();
    let host_name = format!("{}.local", host_ipv4);
    let port = 5353;
    let mut properties = HashMap::new();
    properties.insert("td".to_string(), "/.well-known/wot".to_string());
    properties.insert("type".to_string(), "Thing".to_string());
    properties.insert("scheme".to_string(), "coap".to_string());

    ServiceInfo::new(
        service_type,
        instance_name,
        &host_name,
        host_ipv4,
        port,
        Some(properties),
    )
    .expect("Should successfully create the TD ServiceInfo")
}
