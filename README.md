# itl-chorus

## Development

itl-chorus is written in [rust](https://www.rust-lang.org/). To build and run it, you need to install the rust programming language tools. The easiest way to do this is to use [rustup](https://rustup.rs/).

This project is made up of three crates: an [actix-web](https://actix.rs/) `server`, a [yew](https://yew.rs/) `frontend`, and a `common` crate that contains shared types for both the server and the frontend.

To build the frontend:

```cli
# Add the wasm32 target
$ rustup target add wasm32-unknown-unknown

# install trunk
$ cargo install trunk

# build the frontend
$ cd frontend
$ trunk build
# or to watch for changes
$ trunk watch
```

To build or run the server you can use the typical cargo commands:

```cli
cd server
cargo run
```
