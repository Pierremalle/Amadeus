# Environment

## Why

To start programming on this project, a few needs to be followed to unsure a successfull build.

## Docker

Most of the services can be run using docker.
You just need to create a .env file at root with following variable :

```
FRONT_PORT=8080
API_PORT=8000
BDD_PORT=5432

TOKIO_PORT=9000
TOKIO_CONSOLE_PORT=6669

SURREALDB_USER=root
SURREALDB_PASS=root
```

You can absolutely change ports

## Webserver

Now for everyone.

- Install cargo
- Follow this [tutorial](https://dioxuslabs.com/learn/0.6/getting_started/#) to get dioxus.
- Install DIoxus VSCode extension
- Cd into the webapp repo
- Use the command dx serve --addr 0.0.0.0 command

## embedded

To run the embedded part and install it on ESP32.

```
sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
```

Then, globally follow this :

- https://github.com/esp-rs/rust-build?tab=readme-ov-file

- Install all needed toolchain whith the command :

- Install he template generator : `cargo install cargo-generate`
- Install esp-idf :
  - `git clone --recursive https://github.com/espressif/esp-idf.git`
  - `cd esp-idf`
  - `./install.sh`
  - Add `export IDF_PATH=~/esp-idf` in bashrc.
