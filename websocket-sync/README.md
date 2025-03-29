# Websocket Sync

This is a websocket server for the Open World Game audio-visual experiment.

This server is a prototype for a multiplexer of data coming from multiple sources and outputs in whichever protocol is required. Currently, it takes in UDP data from hardcoded addresses then repacks it into JSON.

Right now, TouchDesigner generates samples of data in realtime over UDP, but that data must be synchronized over to a website. The only way to have realtime synchronization with a website with current standards is Websocket.

## Running

The websocket server can be started with a `cargo run`

## Improvements

We can achieve better performance by lowering the barriers to deserialization. The best way is to not have to deserialize at all and potentially creating a Python library that handles the WebSockets directly.

The second best way is to optimize bottlenecks around the protocol and make the deserialization process smoother. We can do this a bit using WebTransport and the QUIC protocol. This is a bit experimental so might incur costs in wrapping wrappers.

That's why I think the potential best path here is to leverage FFI on both Python and Rust to remove deserialization barriers while having access to WebTransport / QUIC for web sync and performance / ease of both languages.
