# Chromia

Enables you to connect to a Chrome Debugging Protocol (CDP) server and send commands to it.

## Examples

```bash
# start google chrome in remote debugging mode
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222 --user-data-dir=~/Desktop/chrome_profile
# open a tab with a game of colonist.io
cargo run --example colonist
```

## Links

- https://chromedevtools.github.io/devtools-protocol/