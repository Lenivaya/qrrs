<h1 align="center">QRrs</h1>

<p align="center">CLI QR code generator and reader written in rust</p>

![CI](https://github.com/Lenivaya/qrrs/workflows/CI/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

# Info

<img src="https://user-images.githubusercontent.com/49302467/111488823-47600b00-8742-11eb-8fe4-125657091599.png" width="40%" height="70%" align="right" float="right" margin="2em">

QRrs is a command-line utility written in rust for working with qr codes (what actually follows from the name).

## Usage

### Generate code, than read it

```console
$ qrrs "Something" /tmp/qr.png
```

```console
$ qrrs --read /tmp/qr.png
Something
```

### Show code as text in terminal, or save it in a file

```console
$ qrrs --read --terminal /tmp/qr.png
```

```

█████████████████████████████
█████████████████████████████
████ ▄▄▄▄▄ █▄ █▄▄█ ▄▄▄▄▄ ████
████ █   █ █▀▄████ █   █ ████
████ █▄▄▄█ █ ▄█▀▄█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█ ▀ ▀ █▄▄▄▄▄▄▄████
████▄███ █▄▄ ▄▀ ▀▄▄▄  █▀▄████
████▄ ▀█▀▄▄▀▄▀▀▄█▀▄█ █▄ ▀████
████▄█▄██▄▄▄▀▀▀█ ▄▀█ ▀█▄ ████
████ ▄▄▄▄▄ █▄▀▄▀ ▄▄▀ ██ █████
████ █   █ █▄█▀ ▀▄▄█ ▀▀ ▀████
████ █▄▄▄█ ██▀ ▄█▀ ▀ ████████
████▄▄▄▄▄▄▄█▄▄▄█▄▄▄▄█▄██▄████
█████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀

```

```console
$ qrrs --read --terminal /tmp/qr.png >> code.txt
```

## Install

### From crates.io

```console
$ cargo install qrrs
```

### From github

```console
$ cargo install --git https://github.com/Lenivaya/qrrs.git
```
