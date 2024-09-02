# xilers

A distributed file system written in Rust with cross-platform support.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contact](#contact)

## Features

- TODO

## Installation

### Prerequisites

- rustc
- cargo
- ...

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/JIEEEN/xilers.git
   cd xilers
   ```
2. Install dependencies:

   ```bash
   cargo build
   ```

## Usage

### Basic Usage

You can run the client using the command below. Both CLI and GUI modes are supported, but the GUI is still under development.

```bash
cargo run --bin client [cli | gui]
```

### Config

You can modify the **config.toml** file to test the system locally without using a remote server. Simply update the configuration as shown below:

```toml
[server]
master_ip = "http://127.0.0.1"
master_port = 8080

[client]
file_storage = "/tmp"
listen_port = 8081
```

then run server

```bash
cargo run --bin server
```

### Demo

![Client demo1 of xilers](images/client_demo1.gif)

### API Documentation

TODO

## Contact

- Email: tjtpp009@korea.ac.kr
- GitHub: [JIEEEN](https://github.com/yourusername)
