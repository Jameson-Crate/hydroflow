use std::collections::HashSet;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use hydroflow::scheduled::graph::Hydroflow;
use hydroflow::util::{bind_udp_bytes, ipv4_resolve};
use hydroflow_macro::hydroflow_syntax;

use crate::{default_server_address, Opts, Role};
use crate::protocol::{Message, MessageWithAddr};
use crate::protocol::Message::ChatMsg;


#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Hash)]
pub struct ChatMessage {
    nickname: String,
    message: String,
    ts: DateTime<Utc>,
}

enum InfectionOperation {
    InfectWithMessage { msg: ChatMessage },
    RemoveForMessage { msg: ChatMessage },
}

pub(crate) async fn run_gossiping_server(opts: Opts) {
    // If a server address & port are provided as command-line inputs, use those, else use the
    // default.
    let server_address = opts
        .address
        .unwrap_or_else(|| default_server_address());

    let gossip_address = match opts.role {
        Role::Client | Role::Server => { panic!("Incorrect role {:?} for gossip server.", opts.role)}
        Role::GossipingServer1 => ipv4_resolve("localhost:54322"),
        Role::GossipingServer2 => ipv4_resolve("localhost:54323"),
        Role::GossipingServer3 => ipv4_resolve("localhost:54324"),
        Role::GossipingServer4 => ipv4_resolve("localhost:54325"),
        Role::GossipingServer5 => ipv4_resolve("localhost:54326"),
    }.unwrap();

    println!("Starting server on {:?}", server_address);

    let (client_outbound, client_inbound, actual_server_addr) = bind_udp_bytes(server_address).await;
    let (gossip_outbound, gossip_inbound, _) = bind_udp_bytes(gossip_address).await;

    println!("Server is live! Listening on {:?}. Gossiping On: {:?}", actual_server_addr, gossip_address);

    let mut hf: Hydroflow = hydroflow_syntax! {
        // Define shared inbound and outbound channels
        client_out = union() -> dest_sink_serde(client_outbound);
        client_in = source_stream_serde(client_inbound)
            -> map(Result::unwrap)
            -> map(|(msg, addr)| MessageWithAddr::from_message(msg, addr))
            -> demux_enum::<MessageWithAddr>();
        clients = client_in[ConnectRequest] -> map(|(addr,)| addr) -> tee();
        client_in[ConnectResponse] -> for_each(|(addr,)| println!("Received unexpected `ConnectResponse` as server from addr {}.", addr));

        // Pipeline 1: Acknowledge client connections
        clients[0] -> map(|addr| (Message::ConnectResponse, addr)) -> [0]client_out;

        // Pipeline 2: Broadcast messages to all clients and gossip to other servers
        messages_from_connected_client = client_in[ChatMsg]
            -> map(|(_addr, nickname, message, ts)| ChatMessage { nickname, message, ts })
            -> tee();

        messages_from_connected_client[0]
            -> map(|chat_msg| Message::ChatMsg {
                nickname: chat_msg.nickname,
                message: chat_msg.message,
                ts: chat_msg.ts     })
            -> [0]broadcast;
        messages_from_connected_client[1]
            -> map(|msg| InfectionOperation::InfectWithMessage { msg })
            -> infecting_messages;

        clients[1] -> [1]broadcast;
        broadcast = cross_join::<'tick, 'static>() -> [1]client_out;

        // Pipeline 3: Gossip-based broadcast to other servers.
        // gossip_out = dest_sink_serde(gossip_outbound);
        // gossip_in = source_stream_serde(gossip_inbound)
        //     -> map(Result::unwrap)
        //     -> null();
        //
        // null() -> gossip_out;

        // If you think there may be a new message, send it here.
        maybe_new_messages = union();

        // Actually new message is a stream of messages that have never been seen before.
        actually_new_messages = difference() -> tee();
        maybe_new_messages -> [pos]actually_new_messages;
        all_messages -> [neg]actually_new_messages;

        // When we have a new message, we should add it to the set of known messages.
        actually_new_messages -> defer_tick() -> all_messages;

        // Holds all the known messages
        all_messages = fold::<'static>(HashSet::<ChatMessage>::new, |accum, message| {
            accum.insert(message)
        }) -> flatten();

        // Holds a set of messages that are currently infecting this server
        infecting_messages = fold::<'static>(HashSet::<ChatMessage>::new, |accum, op| {
            match op {
                InfectionOperation::InfectWithMessage{ msg } => {accum.insert(msg)},
                InfectionOperation::RemoveForMessage{ msg } => { accum.remove(&msg) }
            }
        }) -> null();
    };

    if let Some(graph) = opts.graph {
        let serde_graph = hf
            .meta_graph()
            .expect("No graph found, maybe failed to parse.");
        serde_graph.open_graph(graph, opts.write_config).unwrap();
    }

    hf.run_async().await.unwrap();
}
