# MetaMask Desktop

Experimental desktop version of MetaMask.

## Design

Essentially the application is a web server and native window that embeds a webview.

The existing MetaMask browser extension will be a *bridge* that connects dapps but rather than calling Infura directly and managing accounts it will call the *desktop agent* over HTTP to interact with the blockchain.

The embedded webview can communicate with the host code using JSON-RPC over a privileged IPC channel which uses `window.postMessage` under the hood.

The web server runs a public service exposed at `/rpc` that is protected by [oauth2][] which allows *whitelisted* third-party applications to communicate with MetaMask using permissions explicitly granted by the resource owner (holder of the private keys).

By using standard web technology we can enable third-party applications whether they are browser extensions, websites or desktop applications to connect to MetaMask in a secure manner.

The primary caveat to this design is that we need to use a well-known port to facilitate discovery for the [oauth2][] authentication; the port `7777` was chosen as it represents the ASCII decimal values of MM.

The secondary caveat is that it is currently very hard to run SSL/TLS for a server running on `localhost` without using [mkcert][]. Ideally, browsers would bypass the SSL warnings for self-signed certificates for localhost but that is unlikely to happen so we should have a view to integrate a CA using [mkcert][] in the future. Because requests go via `localhost` and the loopback address then this attack vector implies the machine is already compromised.

To prevent supply chain attacks on the assets (HTML/CSS/Javascript etc.) embedded in the executable with access to the privileged IPC channel it is recommended that vanilla Javascript is used with minimal dependencies. All dependencies should be vetted and vendored - there is no `npm install` and there should not be any build step. It is suggested that [preact][] and [htm][] be the only vendored dependencies.

Applications that connect to the Metamask *desktop agent* should take precautions to prevent supply chain attacks by using [LavaMoat][].

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
[oauth2]: https://oauth.net/2/
[mkcert]: https://github.com/FiloSottile/mkcert
[preact]: https://preactjs.com/
[htm]: https://github.com/developit/htm
[LavaMoat]: https://github.com/LavaMoat/LavaMoat
