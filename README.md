# rollbar-rs: A Rollbar notifier for Rust

[![Build Status](https://travis-ci.org/aergonaut/rollbar-rs.svg)](https://travis-ci.org/aergonaut/rollbar-rs)

rollbar-rs is a simple [Rollbar](https://rollbar.com/) notifier for
[Rust](http://rust-lang.org).

## Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
rollbar = "0.1"
```

Then add the crate to your code:

```rust
extern crate rollbar;
```

## Usage

Create a client:

```rust
let rollbar = Client::new("<YOUR_ROLLBAR_ACCESS_TOKEN>", "production", "your_app_name", "0.1.0");
```

When you encounter an error, use the `report` function on the client.

## Contributing

1. Fork the reposiptory
2. Create a feature branch
3. Push your branch to GitHub
4. Open a Pull Request

Please include tests in your PR!

## License

The MIT License (MIT)

Copyright (c) 2015 Chris Fung

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
