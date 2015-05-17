#![doc(html_root_url = "https://aergonaut.com/rollbar-rs/")]

//! # Rollbar
//! 
//! Rollbar is a notifier library for the [Rollbar](http://rollbar.com) error
//! tracking service.
//! 
//! ## Installation
//! 
//! 1. Add it to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! rollbar = "0.1.0"
//! ```
//! 
//! 2. Then add the Crate to your code:
//! 
//! ```no_run
//! extern crate rollbar;
//! ```
//! 
//! ## Usage
//! 
//! Create a `Client` using the `Client::new` constructor. You must pass your API
//! key, as well as the environment (i.e. production, staging), the name of your
//! app, and the version of your app as parameters to the constructor.
//! 
//! ```
//! use rollbar::Client;
//!
//! let client = Client::new("8e326ad71017405d60e84172dafd3868", "production", "my_app", "0.1.0");
//! ```
//! 
//! If you encounter an error, use the `report` method on the `client`.

pub mod client;

pub use client::Client;
