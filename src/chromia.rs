use ureq;

use crate::{client::Client, models::ConnectResponse};

pub fn connect_to_page(page_url: &str, port: u16) -> Result<Client, String> {
    let debugger_url = format!("http://localhost:{port}/json");

    let resp: ConnectResponse = ureq::get(&debugger_url)
        .call()
        .map_err(|e| format!("Failed to connect to debugger: {e}"))?
        .into_json()
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    let page = resp
        .iter()
        .find(|item| item.url.contains(page_url))
        .ok_or("Failed to find page".to_owned())?;

    Ok(Client::new(&page.web_socket_debugger_url))
}
