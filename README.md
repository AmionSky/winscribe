# Winscribe

A build script to create, compile and include Windows resource and manifest files in your executables.

## Usage

Add it as a build dependency to your `Cargo.toml`.

```toml
[build-dependencies]
winscribe = "0.1.0"
```

In your `build.rs` use `ResBuilder` to customize your resource file then call `.compile()` on it to link it to your binary.

```rust
use winscribe::icon::Icon;
use winscribe::manifest::{DpiMode, Feature, Manifest};
use winscribe::{ResBuilder, ResError};

fn main() {
    // Only run it if the target is Windows
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        resource().expect("Failed to include resource!");
    }
}

fn resource() -> Result<(), ResError> {
    // Use Cargo's environment variables to fill in some file details
    ResBuilder::from_env()?
        // Add an application icon as a resource
        .push(Icon::app("./assets/application.ico"))
        // Compose a new manifest with DPI awareness and usage of Controls DLL v6
        .push(Manifest::from([
            Feature::DpiAware(DpiMode::PerMonitorV2),
            Feature::ControlsV6,
        ]))
        // Compile and link the resource to the binary
        .compile()
}
```

## Requirements

To compile the resource file a Windows SDK version 10 or later needs to be installed.

## Special Thanks

Winscribe started out as a heavily modifed version of:
* [tauri-winres](https://crates.io/crates/tauri-winres)
* [embed-resource](https://crates.io/crates/embed-resource)

## License

Licensed under either of

 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.