use chromia::chromia::connect_to_page;

fn main() {
    let mut client = connect_to_page("https://colonist.io", 9222).unwrap();

    let html = client.execute("document.getElementById('game-log-text').innerHTML");
    std::fs::write("chat.html", &html["value"].as_str().unwrap()).unwrap();

    let image = client.screenshot();
    std::fs::write("screenshot.png", image).unwrap();
}
