# API Rest with Rust and Actix Web Framework
This project aims to present a Rust project with Actix Web Framework

Technologies used: Rust, Actix Web, Chrono, Serde, Tracing, MongoDB, Bson

### Pre-Requires
- rust and cargo ([Install](https://www.rust-lang.org/tools/install))
- podman ([Install](https://podman.io/getting-started/installation))

### Podman Commands

- Run MongoDB on Container

`podman run --name mongodb -p 27017:27017 -d mongo`

- Stop container

`podman stop mongodb`

- Start container

`podman start mongodb`


### API Commands

- Compile project on develop

`cargo build`

- Compile on release

`cargo build --release`

- Execute on develop with hot reload

`cargo watch -x run`

- Execute on release

`cargo run --release`
