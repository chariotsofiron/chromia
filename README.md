# Chromia

Enables you to connect to a Chrome Debugging Protocol (CDP) server and send commands to it.

```
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222 --user-data-dir=~/Desktop/chrome_profile
```

## Linting

```bash
cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::restriction -A clippy::mod_module_files -A clippy::implicit-return -A clippy::missing-inline-in-public-items -A clippy::std-instead-of-core -A clippy::indexing-slicing -A clippy::integer-arithmetic -A clippy::arithmetic-side-effects
```


## Examples

```bash
cargo run --example colonist
```

## Links

- https://chromedevtools.github.io/devtools-protocol/