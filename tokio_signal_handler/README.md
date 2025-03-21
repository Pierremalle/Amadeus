# signal handler

## Goal

This software serve as a middleware between amadeus embedded / file_sender binaries and the amadeus webserver.

## API

Connect to the middleware with through a TCP socket on the address 9000

## Serve

Following command is used to run the middleware :
`cargo run [<server>] [<bind>]`

Both arguments are optionnal, if not precised they will take the values :

- Server : 51.75.126.107:8000
- Bind : 0.0.0.0:9000

You can use in another terminal the `tokio-console` to get a view of all running task in real time.
