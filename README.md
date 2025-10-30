# url-opener (example)

A small, focused Rust example that demonstrates how to register a simple "viewer" on macOS that receives and handles specific URL schemes. This repository is intended as a minimal reference showing how an app can be set up to be invoked via custom URL schemes (or the system open-file mechanism) and how the Rust side receives and handles those incoming URLs. See app/main.rs for the exact URL schemes and handler logic used in this example.

Important: Replace any placeholder scheme names in this README with the real scheme(s) declared in app/main.rs or the app bundle's Info.plist.

---

## What this example shows

- How to register a viewer that the OS can call using custom URL schemes or file associations.
- How a minimal Rust binary receives the incoming URL or file path and processes it.
- How to test the registration locally (using the macOS `open` command or by opening a URL from a browser).
- A compact, educational code sample you can adapt for your real application.

This repo is not a full production app — it's a small demonstration for learning how URL-scheme / open-file handling can be wired up with Rust on macOS.

---

## Where to look next

- app/main.rs — contains the handler code and the exact list of URL schemes, argument parsing, and any logic the viewer uses. Verify the exact scheme strings and any expected query parameters there.
- macOS bundle config / Info.plist (if present) — declares the URL scheme(s) and document types the OS will associate with this viewer.

---

## Build & run (developer flow)

Prerequisites:
- macOS
- Rust toolchain (rustup / cargo)

Clone and build:

```bash
git clone https://github.com/dkmstr/url-opener.git
cd url-opener
cargo build --release
# binary: ./target/release/url-opener
```

Run locally (when invoked directly, the program may expect a URL or path argument — check app/main.rs for the exact behavior):

```bash
# Example: replace SCHEME and the rest with the real scheme and parameters used in app/main.rs
./target/release/url-opener "SCHEME://open?url=https://example.com"
# or, if the program expects a file path:
./target/release/url-opener "/path/to/file.pdf"
```

---

## How to test macOS URL-scheme registration

1. If you build and install this as a macOS app bundle (or register the binary), macOS will route URLs matching the declared scheme(s) to the app.
2. Use the `open` command to simulate a URL open event:

```bash
# Replace SCHEME with the actual scheme used by the app (see app/main.rs)
open "SCHEME://some/path?param=value"
```

3. Or, open from a browser or another application using a link with the custom scheme.

Note: If the app is not packaged as a bundle or not registered with the system, the OS will not route scheme URLs to it. For local testing you can often run the binary directly and pass the expected argument(s) (see app/main.rs for accepted argument formats).

---

## Minimal explanation of the runtime flow

1. The OS launches the app (or runs it) in response to a URL or file open request.
2. The app's main routine (app/main.rs) receives the incoming data (either as a command-line argument or via platform-specific open-url events).
3. The handler inspects the URL scheme and any query parameters and then performs the desired action (open a viewer, forward to another program, display a UI, etc.).
4. The example focuses on the parts that show how to receive and parse the incoming URL and demonstrates one simple handling behavior.

---

## What changed

- Re-focused the description to correctly state that this is an example of registering and handling a viewer for custom URL schemes, instead of implying broader or different functionality.
- Added guidance on where to find the exact schemes (app/main.rs) and how to test locally.
- Provided clear build/run/test instructions and an explanation of the runtime flow.

If you prefer, paste the contents of app/main.rs here (or allow me to read it) and I will update the README to include the exact scheme names, the exact command-line format, and concrete examples that match the implementation.

---

## Contributing

This repo is intended as a small, educational example. If you find omissions or want clearer examples for additional platforms or packaging steps, feel free to open an issue or submit a pull request.

---

## License

Check the LICENSE file in the repository. If none is present, contact the owner for licensing details.
