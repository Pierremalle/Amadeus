# Usage

## Webserver

To compile the project webpart, use `cargo build --release --manifest-path webapp/Cargo.toml` command.

To serve it, use the `dx serve --addr 0.0.0.0 --port XXX` command.

## Embedded

To compile the project, use the `cargo build -Z build-std --release --target riscv32imac-esp-espidf --manifest-path embedded/Cargo.toml` command
