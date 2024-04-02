# XIAO-Files

Xiao-Files is a minimal file exchange server designed for clients with browsers only.

## Example

Let's say we have a host with IP `10.8.20.1`, and a Windows XP VM client at `10.8.20.2`. Exchanging files from and to this WinXP VM is an annoying thing, since we need to configure a FTP server in the host, or we need to set up OpenSSH server in the WinXP side.

With Xiao-Files, we can easily accomplish this task by using

```shell
RUST_LOG=info xiao-files --address 10.8.20.1 --port 8080 --real-path path/to/shared/dir
```

The WinXP client only needs a browser (IE is also supported) and an active network connection with the host to exchange files. `http://10.8.20.1:8080/files` is for file downloading and `http://10.8.20.1:8080/upload` is for file uploading. The exchanged file will reside in `path/to/shared/dir` in the host side.

## Install

Clone this repo and compile it with Rust:

```shell
git clone --depth 1 https://github.com/Evian-Zhang/xiao-files && cd xiao-files
cargo build --release
```

The generated executable will reside in `./target/release/xiao-files`.

Altenatively, Xiao-Files can be installed by cargo:

```shell
cargo install xiao-files
```

## Usage

```shell
xiao-files --help
Minimal file exchange server designed for clients with browsers only

Usage: xiao-files [OPTIONS] --address <ADDRESS> --port <PORT> --real-path <REAL_PATH>

Options:
      --address <ADDRESS>      IP address to bind
      --port <PORT>            Port to bind
      --real-path <REAL_PATH>  Real path of hosted directory
  -j, --jobs <JOBS>            Number of threads used
  -h, --help                   Print help
```
