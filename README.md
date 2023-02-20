# A Basic WoT CoAP Server supporting DNS-SD

This Repository contains a simple Rust project that provides additional test
results for the
[WoT Discovery specification](https://www.w3.org/TR/wot-discovery).
It demonstrates how to utilize DNS-SD (using Multicast DNS) for discovering
CoAP servers ([RFC 7252](https://www.rfc-editor.org/rfc/rfc7252)) that host
[Web of Things Thing Descriptions](https://www.w3.org/TR/wot-thing-description11/)
(WoT TDs).


## Setup

First you need to make sure that you have
[Rust installed](https://www.rust-lang.org/tools/install).
Then, after cloning the repository, simply type

```sh
cargo run
```

into a terminal of your choice (once you have changed your working directory
to your local copy of the repository).
This will start both a CoAP server and an mDNS daemon, the latter of which
will enable local DNS-Based service discovery (DNS-SD).

## Usage

Once the CoAP server and the mDNS service daemon are running, you can perform
DNS-based service discovery locally to discover the server's TD that is hosted
under `/.well-known/wot`.
Using multicast DNS, you can find the server by browsing for the service
instance using the type `_wot._udp`, which will return the instance name
`wot_coap_dns_sd`.
Looking up the instance name will yield the `td` path mentioned above, a
`scheme` parameter set to `coap`, and a `type` set to `Thing` in the TXT
records, as specified in [section 6.3 of the WoT Discovery specification](https://www.w3.org/TR/wot-discovery/#introduction-dns-sd-sec).

Using this information, you can send a CoAP GET request to the assembled URL
and obtain the (very basic) Thing Description hosted by the CoAP server.

Once you are finished with running the application, you can simply use `Ctrl+C`
to exit it.

## License

```
SPDX-License-Identifier: MIT
```

