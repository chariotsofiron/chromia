use crate::models::GCDResponse;
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

/// A connection to a particular tab
pub struct Client {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    request_id: u32,
}

impl Client {
    pub fn new(url: &str) -> Client {
        let (socket, _response) = connect(url).unwrap();

        Client {
            socket,
            request_id: 0,
        }
    }

    /// Sends a request to the server
    pub fn send<T>(&mut self, method: &str, params: Value) -> T
    where
        T: std::fmt::Debug + serde::de::DeserializeOwned,
    {
        let data = json!({
            "id": self.request_id,
            "method": method,
            "params": params,
        });
        let request = Message::Text(data.to_string());
        self.socket.write_message(request).unwrap();

        let msg = self.socket.read_message().expect("Error reading message");
        let json: GCDResponse<T> = serde_json::from_str(msg.to_text().unwrap()).unwrap();
        assert!(json.id == self.request_id);

        self.request_id += 1;
        json.result
    }

    pub fn execute(&mut self, script: &str) -> Value {
        #[derive(Debug, serde::Deserialize)]
        struct Result {
            result: Value,
        }
        let response: Result = self.send("Runtime.evaluate", json!({ "expression": script }));
        response.result
    }

    pub fn screenshot(&mut self) -> Vec<u8> {
        #[derive(Debug, serde::Deserialize)]
        struct Screenshot {
            /// Base64-encoded image data
            data: String,
        }
        let response: Screenshot = self.send("Page.captureScreenshot", json!({}));
        general_purpose::STANDARD
            .decode(response.data)
            .expect("Failed to decode screenshot")
    }
}
