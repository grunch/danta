# Danta - Lightning Network event registration app

Simple lightning event registration rust app, this app connects to a lnd node using gRPC.

![Demo](demo.gif)

## Requirements:

0. You need Rust version 1.48 or higher to compile.
1. You need to have LND 0.14.2, ideally v0.15.0-beta

## Install

Clone the repository and then create a new `.env` file based on `.env-sample` file.

```
$ git clone https://github.com/grunch/danta.git
$ cd danta
$ cp .env-sample .env
```

To connect with a lnd node we need to set 3 variables in the `.env` file .

_LND_CERT_FILE:_ LND node TLS certificate file path, the default is `$HOME/.lnd/tls.cert` on the lnd node.

_LND_MACAROON_FILE:_ Macaroon file path, the macaroon file contains permission for doing actions on the lnd node, for this app a good choice is to use the `invoice.macaroon` file, the default is `$HOME/.lnd/data/chain/bitcoin/mainnet/invoice.macaroon`.

_LND_GRPC_HOST:_ IP address or domain name from the LND node, example: `192.168.0.2`.

_LND_GRPC_PORT:_ LND node port to connect, example: `10009`.

### Database

The data is saved in a sqlite db file named by default `data.db`, this file is saved on the root directory of the project and can be change just editing the env var `DATABASE_URL` on the `.env` file.

Before start building we need to initialize the database, for this we need to use `diesel_cli`:

```
$ cargo install diesel_cli --no-default-features --features sqlite
```

Now we can initialize our database:

```
DATABASE_URL=data.db diesel migration run
```

This creates `data.db` in our project file.

## Install dependencies

To compile on Ubuntu/Pop!\_OS, you need to install some dependencies, run the following commands:

```
$ sudo apt update
$ sudo apt install -y cmake build-essential libsqlite3-dev pkg-config libssl-dev
```

## Compile and execute it:

```
$ cargo build --release
$ target/release/danta
```

Go to http://localhost:8000
