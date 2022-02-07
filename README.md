# MetaMask Desktop

Experimental desktop version of MetaMask.

## Build

To compile on Linux install `libwebkit2gtk` see the [wry-linux-notes][].

To fetch rates from [coincap][] the `COINCAP_API_KEY` must be set at compile time.

## Bundles

Install [cargo-bundle][] and build for the platform:

```
cargo bundle --release
```

[cargo-bundle]: https://github.com/burtonageo/cargo-bundle
[wry-linux-notes]: https://github.com/tauri-apps/wry#linux
[coincap]: https://coincap.io/
