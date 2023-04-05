```rust
/// Gets the path to the Chrome executable.
pub fn get_path() -> Option<PathBuf> {
    #[cfg(all(unix, not(target_os = "macos")))]
    let paths: [&str; 0] = [];
    #[cfg(windows)]
    let paths: [&str; 0] = [];
    #[cfg(target_os = "macos")]
    let paths = ["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"];
    for path in paths {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

pub fn launch(port: u16, user_data_dir: &str) {
    let executable = get_path().expect("Could not find Chrome executable");

    let mut cmd = std::process::Command::new(executable);
    cmd.arg(format!("--remote-debugging-port={port}"));
    cmd.arg(format!("--user-data-dir={user_data_dir}"));
    cmd.spawn().expect("Could not launch Chrome");
}
```