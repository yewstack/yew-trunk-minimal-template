# Yew Trunk Template

This is a fairly minimal template for a Yew app that's built with [Trunk].

## Usage

For a more thorough explanation of Trunk, please head over to the [repository][trunk].

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our bases covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Starts a local server and rebuilds the app whenever a change is detected.
At the time of writing you still have to manually reload the webpage in your browser
but that's a small price to pay.

There's also the `trunk watch` command which does the same thing minus the web server.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass this flag to `trunk serve` if you need every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## Using this template

This example tries to be fairly minimal but there are a few things you should be aware of.

### Remove the example code

The code in [src/main.rs](src/main.rs) specific to the example is limited to only the `view` method.
You can keep the rest.
There is, however, a fair bit of Sass in [index.scss](index.scss) you might want to remove or update.

### Metadata

You'll definitely want to update the `name`, `version`, `description` and `repository` fields.
The [index.html](index.html) file also contains a `<title>` tag that needs updating.

Finally, you should update this very `README` file.

### License

The template ships with both the Apache and MIT license.
If you don't want to have your app dual licensed, just remove one (or both) of the files and update the `license` field in `Cargo.toml`.

There are also two empty spaces in the MIT license you need to fill out. Particularly `{{year}}` and `{{authors}}`.

[trunk]: https://github.com/thedodd/trunk
