use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread::spawn;
use tungstenite::Bytes;
use tungstenite::Utf8Bytes;
use tungstenite::WebSocket;
use tungstenite::{accept, Message};

#[derive(Serialize, Deserialize, Debug, Default)]
struct Model {
    chan1: Vec<f32>,
}

/// Tacks websocket streams and broadcasts messages
struct WebsocketManager {
    websocket_streams: HashMap<u32, WebSocket<TcpStream>>,
    disconnect_queue: Vec<u32>,
    counter: u32,
}

impl WebsocketManager {
    fn new() -> Self {
        WebsocketManager {
            websocket_streams: HashMap::new(),
            disconnect_queue: vec![],
            counter: 0,
        }
    }

    fn track_stream(&mut self, stream: WebSocket<TcpStream>) {
        self.websocket_streams.insert(self.counter, stream);
        self.counter += 1;
    }

    fn disconnect(&mut self, id: u32) {
        self.websocket_streams.remove(&id);
    }

    fn broadcast(&mut self, message: Message) {
        for (websocket_id, websocket) in self.websocket_streams.iter_mut() {
            if websocket.send(message.clone()).is_err() {
                websocket
                    .close(None)
                    .expect_err("Websocket connection lost, closing...");
                if let tungstenite::Error::ConnectionClosed =
                    websocket.flush().expect_err("Closing websocket connection")
                {
                    self.disconnect_queue.push(*websocket_id);
                };
            }
        }

        while let Some(id) = self.disconnect_queue.pop() {
            self.disconnect(id);
        }
    }
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    let udp_socket = UdpSocket::bind("127.0.0.1:7000").unwrap();
    let (tx, rx) = mpsc::channel::<WebSocket<TcpStream>>();

    spawn(move || {
        let mut websocket_manager = WebsocketManager::new();
        let mut buf = [0; 1024];

        loop {
            if let Ok(stream) = rx.try_recv() {
                websocket_manager.track_stream(stream);
            }

            let (amt, _) = udp_socket
                .recv_from(&mut buf)
                .expect("Couldn't receive from udp socket");

            if let Ok(message) = Utf8Bytes::try_from(Bytes::copy_from_slice(&buf[..amt])) {
                websocket_manager.broadcast(Message::Text(message));
            }
        }
    });

    for stream in server.incoming() {
        let stream = stream.expect("TCP Error encountered. Check your network configuration.");
        let websocket = accept(stream).unwrap();
        tx.send(websocket).unwrap();
    }
}
