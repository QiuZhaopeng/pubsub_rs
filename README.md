# pubsub_rs
A small multi-threading publisher-subscriber program in **Rust**. Messages that are published and subscribed could contain any type of data including protobuf messages.

## prerequisites

### install rust
One can install **rust** as per the official documentation in [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### install protoc

One can download *protoc* from [https://github.com/protocolbuffers/protobuf/releases](https://github.com/protocolbuffers/protobuf/releases) then install it to the operating system. Make sure that the *protoc* binary's location is added into the system **path** environment.

## how to run this project
Once **Rust** and **protoc** are well installed, one can run this program by just typing

```
cd pubsub-rust
cargo run
```

If everything is working fine, one should see multiple publisher logs printed as:

```
Published new message guid: "88e6a10f-4756-48b3-981b-8042c2a818e0" value: 10 details: "aaaa" details: "zzzz"
```

as well as multiple subscriber logs as:

```
Received: ActionRequest { guid: "88e6a10f-4756-48b3-981b-8042c2a818e0", value: 10, details: ["aaaa", "zzzz"], special_fields: SpecialFields { unknown_fields: UnknownFields { fields: None }, cached_size: CachedSize { size: 0 } } }, timestamp: 2024-02-18 09:09:14.864492000
```