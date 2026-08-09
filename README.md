# Venus

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)

**Venus** is a modular networking and routing library written in Rust. It provides core primitives for building efficient, low-latency network applications.

## Overview

Venus offers a flexible foundation for network session management, packet routing, and data flow control. It is designed to be fast, secure, and easily integrable into larger systems.

## Key Features

* **Flexible Routing** — Pluggable routing logic with support for custom policies.
* **Session Management** — Efficient handling of multiple concurrent connections.
* **High Performance** — Built in Rust with zero-cost abstractions and minimal overhead.
* **Modular Design** — Can be used as a standalone crate or as part of a larger stack.
* **Async-Ready** — Designed to work with async runtimes (Tokio, async-std).

## Getting Started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
venus = { git = "https://github.com/ecdhe-x25519/venus" }