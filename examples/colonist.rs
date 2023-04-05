use chromia::client::Client;

fn main() {
    let mut client = Client::from_page(9222, "colonist.io");
    let html = client.execute("document.getElementById('game-log-text').innerHTML");
    std::fs::write("chat.html", &html["value"].as_str().unwrap()).unwrap();
    let image = client.screenshot();
    std::fs::write("screenshot.png", image).unwrap();
}
