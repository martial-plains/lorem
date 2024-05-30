# Lorem Ipsum Generator

## Overview

The Lorem Ipsum Generator is a simple and efficient application designed to
generate placeholder text. This tool is ideal for designers, developers, and
content creators who need dummy text for mockups, wireframes, or any other
project that requires temporary text.

## Features

- **Quick Generation:** Instantly generate placeholder text with a single click.
- **Customizable Length:** Choose the number of paragraphs, sentences, or words
  you need.
- **Copy to Clipboard:** Easily copy the generated text to your clipboard.
- **Responsive Design:** Use the app seamlessly on both desktop and mobile
  devices.
- **User-Friendly Interface:** Simple and intuitive interface that requires no
  learning curve.

## Getting Started

### Testing locally

Make sure you are using the latest version of stable rust by running
`rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly)
and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.

1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will
   rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning
   below.

> `assets/sw.js` script will try to cache our app, and loads the cached version
> when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load
> the latest builds during development.

## Usage

1. Open the app.
2. Select the desired amount of text (number of paragraphs or words).
3. Copy the generated text by clicking the "Copy to Clipboard" button.
4. Paste the text into your project as needed.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file
for details.

---

Thank you for using the Lorem Ipsum Generator! I hope it makes your design and
development process smoother and more efficient.
