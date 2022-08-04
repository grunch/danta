# Danta - Lightning Network event registration app

Simple lightning event registration rust app, this app connects to a lnd node using gRPC.

# Install

Clone the repository and then create a new `.env` file based on `.env-sample` file.

```
$ git clone https://github.com/grunch/danta.git
$ cd danta
$ cp .env-sample .env
```

To connect with a lnd node we need to set 3 variables in the `.env` file .

_LND_CERT_FILE:_ LND node TLS certificate file path, the default is `$HOME/.lnd/tls.cert` on the lnd node.

_LND_MACAROON_FILE:_ Macaroon file path, the macaroon file contains permission for doing actions on the lnd node, for this app a good choice is to use the `invoice.macaroon` file, the default is `$HOME/.lnd/data/chain/bitcoin/mainnet/invoice.macaroon`.

_LND_GRPC_HOST:_ IP address or domain name from the LND node and the port separated by colon (`:`), example: `192.168.0.2:10009`.

## Requirements:

0. You need Rust version 1.48 or higher to compile.
1. You need to have LND 0.14.2, ideally v0.15.0-beta

## Compile and execute it:

To compile on Ubuntu/Pop!\_OS, please install cargo, then run the following commands:

```
sudo apt update
sudo apt install -y build-essential libsqlite3-dev pkg-config libssl-dev
```

Then build:

```
$ cargo build --release
$ target/debug/danta
```

Go to http://localhost:8000
