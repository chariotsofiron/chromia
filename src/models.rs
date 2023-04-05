use serde::{Deserialize, Serialize};
use serde_json::{json, Value};


/// Response returned on any request to the server
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GCDResponse<T> {
    pub id: u32,
    pub result: T,
}

pub type ConnectResponse = Vec<BrowserItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserItem {
    pub description: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub web_socket_debugger_url: String,
}
